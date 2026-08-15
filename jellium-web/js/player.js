// `sink` receives one json object per event:
//   {"event":"ready","duration":<seconds>}
//   {"event":"progress","generation":<n>,"position":<seconds>,
//    "buffered":<seconds>,"paused":<bool>}
//   {"event":"reportDue","position":<seconds>}
//   {"event":"ended"}
//   {"event":"stalled"}
//   {"event":"playable","generation":<n>,"position":<seconds>}
//   {"event":"failed","fault":"decode"|"network"|"unsupported"}
//   {"event":"command","command":"play"|"pause"|"previous"|"next"|"seek",
//    "position":<seconds>}
//
// every payload carries the generation of the stream it belongs to: a media
// element listener is wired when its stream is loaded and closes over that
// stream's stamp, so an event the outgoing stream raises can never carry the
// stamp of the stream replacing it.
//
// `reportDue` is raised from the element's own timeupdate events while
// playing and from a timer while paused, so a hidden tab keeps reporting.
// `load` detaches the outgoing stream, and its listeners with it, before it
// opens the next generation and returns it; nothing the teardown raises
// reaches the sink, and an event stamped with an earlier generation belongs to
// a stream the player has replaced.
// `bind` takes the element the overlay mounted rather than creating one, wires
// the report clock and the media session, and replaces the old `mount`.
// `unbind` drops the sink before it touches the element, so the pause it
// performs raises nothing; the element itself is the overlay's to remove.
// A `visibilitychange` reports at once and restarts the report clock, which is
// the last report before a hidden tab's timers are throttled and the first
// after they are not.
// `setBeacon` arms a pagehide handler that posts `body` to `path` with
// keepalive, so a closed tab ends its session at once.
// `setGroupBeacon` arms a second pagehide handler that posts an empty body to
// `path`, and it lasts as long as group membership rather than as long as an
// element is mounted, so a reload leaves the group.

const REPORT_INTERVAL_MS = 10000;

let element = null;
let hls = null;
let sink = null;
let ticker = null;
let lastReport = 0;
let beacon = null;
let beaconHandler = null;
let groupBeacon = null;
let groupBeaconHandler = null;
let generation = 0;
let listening = null;

function emit(stamp, payload) {
  if (sink) {
    sink(JSON.stringify({ generation: stamp, ...payload }));
  }
}

document.addEventListener('visibilitychange', () => {
  lastReport = Date.now();
  emit(generation, {
    event: 'reportDue',
    position: element ? element.currentTime : 0
  });
});

function bufferedEnd() {
  if (!element || element.buffered.length === 0) {
    return 0;
  }
  const position = element.currentTime;
  for (let index = 0; index < element.buffered.length; index += 1) {
    if (
      element.buffered.start(index) <= position &&
      position <= element.buffered.end(index)
    ) {
      return element.buffered.end(index);
    }
  }
  return element.buffered.end(element.buffered.length - 1);
}

function reportIfDue() {
  const now = Date.now();
  if (now - lastReport < REPORT_INTERVAL_MS) {
    return;
  }
  lastReport = now;
  emit(generation, {
    event: 'reportDue',
    position: element ? element.currentTime : 0
  });
}

function fault() {
  const code = element && element.error ? element.error.code : 0;
  if (code === 2) {
    return 'network';
  }
  if (code === 4) {
    return 'unsupported';
  }
  return 'decode';
}

function command(name) {
  return (details) => {
    emit(generation, {
      event: 'command',
      command: name,
      position: details && details.seekTime ? details.seekTime : 0
    });
  };
}

function wireMediaSession() {
  if (!('mediaSession' in navigator)) {
    return;
  }
  const handlers = {
    play: command('play'),
    pause: command('pause'),
    previoustrack: command('previous'),
    nexttrack: command('next'),
    seekto: command('seek')
  };
  for (const [action, handler] of Object.entries(handlers)) {
    try {
      navigator.mediaSession.setActionHandler(action, handler);
    } catch (thrown) {
      report('failureMediaSession', thrown);
    }
  }
}

// wires the listeners of the stream `stamp` names under one abort signal, so
// detaching removes them together
function attach(stamp) {
  listening = new AbortController();
  const { signal } = listening;

  element.addEventListener(
    'loadedmetadata',
    () =>
      emit(stamp, {
        event: 'ready',
        duration: Number.isFinite(element.duration) ? element.duration : 0
      }),
    { signal }
  );
  element.addEventListener(
    'timeupdate',
    () => {
      emit(stamp, {
        event: 'progress',
        position: element.currentTime,
        buffered: bufferedEnd(),
        paused: element.paused
      });
      reportIfDue();
    },
    { signal }
  );
  element.addEventListener('ended', () => emit(stamp, { event: 'ended' }), {
    signal
  });
  element.addEventListener('stalled', () => emit(stamp, { event: 'stalled' }), {
    signal
  });
  element.addEventListener(
    'canplaythrough',
    () => emit(stamp, { event: 'playable', position: element.currentTime }),
    { signal }
  );
  element.addEventListener('waiting', () => emit(stamp, { event: 'stalled' }), {
    signal
  });
  element.addEventListener(
    'error',
    () => emit(stamp, { event: 'failed', fault: fault() }),
    { signal }
  );
  element.addEventListener(
    'play',
    () =>
      emit(stamp, {
        event: 'progress',
        position: element.currentTime,
        buffered: bufferedEnd(),
        paused: false
      }),
    { signal }
  );
  element.addEventListener(
    'pause',
    () =>
      emit(stamp, {
        event: 'progress',
        position: element.currentTime,
        buffered: bufferedEnd(),
        paused: true
      }),
    { signal }
  );
}

// drops the outgoing stream: its listeners, and hls.js with them
function detach() {
  if (listening) {
    listening.abort();
    listening = null;
  }
  if (hls) {
    hls.destroy();
    hls = null;
  }
}

export function bind(node, callback) {
  unbind();
  sink = callback;
  lastReport = Date.now();
  element = node;

  ticker = setInterval(reportIfDue, 1000);
  wireMediaSession();
}

export function load(path, useHls, start) {
  detach();
  generation += 1;
  const stamp = generation;
  if (!element) {
    return stamp;
  }
  attach(stamp);

  const begin = () => {
    if (start > 0) {
      element.currentTime = start;
    }
    const played = element.play();
    if (played && played.catch) {
      played.catch(() => emit(stamp, { event: 'failed', fault: 'decode' }));
    }
  };

  if (useHls && typeof Hls !== 'undefined' && Hls.isSupported()) {
    hls = new Hls({ startPosition: start > 0 ? start : -1 });
    hls.on(Hls.Events.ERROR, (_, data) => {
      if (!data.fatal) {
        return;
      }
      const kind =
        data.type === Hls.ErrorTypes.NETWORK_ERROR ? 'network' : 'decode';
      emit(stamp, { event: 'failed', fault: kind });
    });
    hls.on(Hls.Events.MANIFEST_PARSED, () => {
      const played = element.play();
      if (played && played.catch) {
        played.catch(() => emit(stamp, { event: 'failed', fault: 'decode' }));
      }
    });
    hls.loadSource(path);
    hls.attachMedia(element);
    return stamp;
  }

  element.src = path;
  element.load();
  if (element.readyState >= 1) {
    begin();
  } else {
    element.addEventListener('loadedmetadata', begin, {
      once: true,
      signal: listening.signal
    });
  }
  return stamp;
}

export function play() {
  if (element) {
    const played = element.play();
    if (played && played.catch) {
      played.catch(() => emit(generation, { event: 'failed', fault: 'decode' }));
    }
  }
}

export function pause() {
  if (element) {
    element.pause();
  }
}

export function position() {
  return element ? element.currentTime : 0;
}

export function seek(seconds) {
  if (element) {
    element.currentTime = seconds;
  }
}

export function seekToLive() {
  if (!element) {
    return;
  }
  if (hls && hls.liveSyncPosition !== undefined && hls.liveSyncPosition !== null) {
    element.currentTime = hls.liveSyncPosition;
    return;
  }
  const seekable = element.seekable;
  if (seekable && seekable.length > 0) {
    element.currentTime = seekable.end(seekable.length - 1);
  }
}

export function setRate(rate) {
  if (element) {
    element.playbackRate = rate;
  }
}

export function setVolume(volume) {
  if (element) {
    element.volume = Math.min(Math.max(volume, 0), 1);
  }
}

export function setMuted(muted) {
  if (element) {
    element.muted = muted;
  }
}

export function setTextTracks(tracks, selected) {
  if (!element) {
    return;
  }
  for (const existing of Array.from(element.querySelectorAll('track'))) {
    existing.remove();
  }
  const wanted = JSON.parse(tracks);
  wanted.forEach((entry, index) => {
    const track = document.createElement('track');
    track.kind = 'subtitles';
    track.src = entry.path;
    track.label = entry.label;
    if (entry.language) {
      track.srclang = entry.language;
    }
    track.default = index === selected;
    element.appendChild(track);
  });
  for (let index = 0; index < element.textTracks.length; index += 1) {
    element.textTracks[index].mode =
      index === selected ? 'showing' : 'disabled';
  }
}

export function setCueStyle(style) {
  const held = JSON.parse(style);
  let sheet = document.getElementById('jellium-cues');
  if (!sheet) {
    sheet = document.createElement('style');
    sheet.id = 'jellium-cues';
    document.head.appendChild(sheet);
  }
  sheet.textContent =
    `video::cue { font-size: ${held.size}%; color: ${held.colour}; ` +
    `background-color: ${held.background}; text-shadow: ${held.shadow}; }`;
}

export function setFullscreen(full) {
  const root = document.documentElement;
  if (full && !document.fullscreenElement) {
    if (root.requestFullscreen) {
      root.requestFullscreen().catch((thrown) => report('failureFullscreen', thrown));
    }
  } else if (!full && document.fullscreenElement && document.exitFullscreen) {
    document.exitFullscreen().catch((thrown) => report('failureFullscreen', thrown));
  }
}

export function setIdle(idle) {
  if (idle) {
    document.documentElement.dataset.idle = '';
  } else {
    delete document.documentElement.dataset.idle;
  }
}

export function setMetadata(metadata) {
  if (!('mediaSession' in navigator) || typeof MediaMetadata === 'undefined') {
    return;
  }
  const details = JSON.parse(metadata);
  navigator.mediaSession.metadata = new MediaMetadata({
    title: details.title,
    artist: details.subtitle,
    artwork: details.artwork ? [{ src: details.artwork }] : []
  });
}

export function setBeacon(path, body) {
  beacon = { path, body };
  if (beaconHandler) {
    return;
  }
  beaconHandler = () => {
    if (!beacon) {
      return;
    }
    const payload = new Blob([beacon.body], { type: 'application/json' });
    if (navigator.sendBeacon) {
      navigator.sendBeacon(beacon.path, payload);
      return;
    }
    fetch(beacon.path, {
      method: 'POST',
      body: beacon.body,
      headers: { 'Content-Type': 'application/json' },
      keepalive: true
    }).catch((thrown) => report('failureBeacon', thrown));
  };
  window.addEventListener('pagehide', beaconHandler);
}

export function setGroupBeacon(path, armed) {
  groupBeacon = armed ? path : null;
  if (groupBeaconHandler) {
    return;
  }
  groupBeaconHandler = () => {
    if (!groupBeacon) {
      return;
    }
    if (navigator.sendBeacon) {
      navigator.sendBeacon(groupBeacon, new Blob([], { type: 'text/plain' }));
      return;
    }
    fetch(groupBeacon, { method: 'POST', keepalive: true }).catch((thrown) =>
      report('failureBeacon', thrown)
    );
  };
  window.addEventListener('pagehide', groupBeaconHandler);
}

export function unbind() {
  sink = null;
  detach();
  if (ticker) {
    clearInterval(ticker);
    ticker = null;
  }
  if (beaconHandler) {
    window.removeEventListener('pagehide', beaconHandler);
    beaconHandler = null;
  }
  beacon = null;
  if (element) {
    element.pause();
    element.removeAttribute('src');
    element.load();
    element = null;
  }
  if ('mediaSession' in navigator) {
    navigator.mediaSession.metadata = null;
  }
  setIdle(false);
}

// writes one console record carrying the sentence `key` reads as and the
// machine cause `thrown` carries; a page being unloaded shows nothing
let strings = null;
function report(key, thrown) {
  if (strings === null) {
    strings = fetch('/strings/en-us.json')
      .then((answer) => answer.json())
      .catch(() => ({}));
  }
  strings.then((table) => {
    console.error(`${table[key] ?? key} | cause: ${thrown}`);
  });
}
