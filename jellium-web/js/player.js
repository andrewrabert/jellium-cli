// `sink` receives one frame per report:
//   {"frame":"media","generation":<n>,"event":{"event":"ready","duration":<seconds>}}
//   {"frame":"media","generation":<n>,"event":{"event":"progress",
//     "position":<seconds>,"buffered":<seconds>,"paused":<bool>}}
//   {"frame":"media","generation":<n>,"event":{"event":"reportDue","position":<seconds>}}
//   {"frame":"media","generation":<n>,"event":{"event":"ended"}}
//   {"frame":"media","generation":<n>,"event":{"event":"stalled"}}
//   {"frame":"media","generation":<n>,"event":{"event":"playable","position":<seconds>}}
//   {"frame":"media","generation":<n>,"event":{"event":"failed","fault":"decode"}}
//   {"frame":"media","generation":<n>,"event":{"event":"command",
//     "command":{"command":"seekTo","position":<seconds>}}}
//   {"frame":"broke","call":"fullscreen"|"mediaSession"|"beacon","cause":"…"}
//
// every media frame carries the generation of the stream it belongs to: a
// media element listener is wired when its stream is loaded and closes over
// that stream's stamp, so an event the outgoing stream raises can never carry
// the stamp of the stream replacing it.
//
// `reportDue` is raised from the element's own timeupdate events while
// playing and from a timer while paused, so a hidden tab keeps reporting.
// `load` detaches the outgoing stream, and its listeners with it, before it
// opens the next generation and returns it as a Number; nothing the teardown
// raises reaches the sink, and an event stamped with an earlier generation
// belongs to a stream the player has replaced.
// `bind` takes the element the overlay mounted rather than creating one, and
// wires the report clock and the media session.
// `unbind` drops the sink before it touches the element, so the pause it
// performs raises nothing; the element itself is the overlay's to remove.
// A `visibilitychange` reports at once and restarts the report clock, which is
// the last report before a hidden tab's timers are throttled and the first
// after they are not.
// The `beacon` ask arms a pagehide handler that posts the stopped report to
// its path with keepalive, so a closed tab ends its session at once.
// `setGroupBeacon` arms a second pagehide handler that posts an empty body,
// and it lasts as long as group membership rather than as long as an element
// is mounted, so a reload leaves the group.

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

function emit(frame) {
  if (sink) {
    sink(JSON.stringify(frame));
  }
}

function media(stamp, event) {
  emit({ frame: 'media', generation: stamp, event });
}

function broke(call, thrown) {
  emit({ frame: 'broke', call, cause: String(thrown) });
}

document.addEventListener('visibilitychange', () => {
  lastReport = Date.now();
  media(generation, {
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
  media(generation, {
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
    media(generation, {
      event: 'command',
      command:
        name === 'seekTo'
          ? {
              command: 'seekTo',
              position: details && details.seekTime ? details.seekTime : 0
            }
          : { command: name }
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
    seekto: command('seekTo')
  };
  for (const [action, handler] of Object.entries(handlers)) {
    try {
      navigator.mediaSession.setActionHandler(action, handler);
    } catch (thrown) {
      broke('mediaSession', thrown);
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
      media(stamp, {
        event: 'ready',
        duration: Number.isFinite(element.duration) ? element.duration : 0
      }),
    { signal }
  );
  element.addEventListener(
    'timeupdate',
    () => {
      media(stamp, {
        event: 'progress',
        position: element.currentTime,
        buffered: bufferedEnd(),
        paused: element.paused
      });
      reportIfDue();
    },
    { signal }
  );
  element.addEventListener('ended', () => media(stamp, { event: 'ended' }), {
    signal
  });
  element.addEventListener('stalled', () => media(stamp, { event: 'stalled' }), {
    signal
  });
  element.addEventListener(
    'canplaythrough',
    () => media(stamp, { event: 'playable', position: element.currentTime }),
    { signal }
  );
  element.addEventListener('waiting', () => media(stamp, { event: 'stalled' }), {
    signal
  });
  element.addEventListener(
    'error',
    () => media(stamp, { event: 'failed', fault: fault() }),
    { signal }
  );
  element.addEventListener(
    'play',
    () =>
      media(stamp, {
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
      media(stamp, {
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

export function load(stream) {
  const wanted = JSON.parse(stream);
  const path = wanted.delivery.path;
  const useHls = wanted.delivery.delivery === 'hls';
  const start = wanted.start;

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
      played.catch(() => media(stamp, { event: 'failed', fault: 'decode' }));
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
      media(stamp, { event: 'failed', fault: kind });
    });
    hls.on(Hls.Events.MANIFEST_PARSED, () => {
      const played = element.play();
      if (played && played.catch) {
        played.catch(() => media(stamp, { event: 'failed', fault: 'decode' }));
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

export function position() {
  return element ? element.currentTime : 0;
}

export function ask(asked) {
  const wanted = JSON.parse(asked);
  switch (wanted.ask) {
    case 'play':
      play();
      return;
    case 'pause':
      if (element) {
        element.pause();
      }
      return;
    case 'seek':
      if (element) {
        element.currentTime = wanted.position;
      }
      return;
    case 'seekToLive':
      seekToLive();
      return;
    case 'rate':
      if (element) {
        element.playbackRate = wanted.rate;
      }
      return;
    case 'volume':
      if (element) {
        element.volume = Math.min(Math.max(wanted.volume, 0), 1);
      }
      return;
    case 'muted':
      if (element) {
        element.muted = true;
      }
      return;
    case 'unmuted':
      if (element) {
        element.muted = false;
      }
      return;
    case 'textTracks':
      setTextTracks(wanted.tracks, wanted.selected);
      return;
    case 'cueStyle':
      setCueStyle(wanted.cues);
      return;
    case 'fullscreen':
      enterFullscreen();
      return;
    case 'windowed':
      leaveFullscreen();
      return;
    case 'idle':
      hideCursor();
      return;
    case 'awake':
      showCursor();
      return;
    case 'metadata':
      setMetadata(wanted.metadata);
      return;
    case 'beacon':
      setBeacon(wanted.path, JSON.stringify(wanted.stopped));
      return;
    default:
      return;
  }
}

function play() {
  if (element) {
    const played = element.play();
    if (played && played.catch) {
      played.catch(() => media(generation, { event: 'failed', fault: 'decode' }));
    }
  }
}

function seekToLive() {
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

function setTextTracks(tracks, selected) {
  if (!element) {
    return;
  }
  for (const existing of Array.from(element.querySelectorAll('track'))) {
    existing.remove();
  }
  tracks.forEach((entry, index) => {
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

function setCueStyle(held) {
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

function enterFullscreen() {
  const root = document.documentElement;
  if (!document.fullscreenElement && root.requestFullscreen) {
    root.requestFullscreen().catch((thrown) => broke('fullscreen', thrown));
  }
}

function leaveFullscreen() {
  if (document.fullscreenElement && document.exitFullscreen) {
    document.exitFullscreen().catch((thrown) => broke('fullscreen', thrown));
  }
}

function hideCursor() {
  document.documentElement.dataset.idle = '';
}

function showCursor() {
  delete document.documentElement.dataset.idle;
}

function setMetadata(details) {
  if (!('mediaSession' in navigator) || typeof MediaMetadata === 'undefined') {
    return;
  }
  navigator.mediaSession.metadata = new MediaMetadata({
    title: details.title,
    artist: details.subtitle,
    artwork: details.artwork ? [{ src: details.artwork }] : []
  });
}

function setBeacon(path, body) {
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
    }).catch((thrown) => broke('beacon', thrown));
  };
  window.addEventListener('pagehide', beaconHandler);
}

export function setGroupBeacon(frame) {
  const wanted = JSON.parse(frame);
  groupBeacon = wanted.beacon === 'armed' ? wanted.path : null;
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
      broke('beacon', thrown)
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
  showCursor();
}
