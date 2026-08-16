// The overlay hosts more than one DOM element at a time over the iced canvas,
// each with its own identity, stacking, pointer behaviour and message channel.
//
// mount(wanted, sink) takes one rendered description:
//   {"id":"media","kind":"video","stacking":"below","pointer":false,
//    "source":null,"sandbox":null,"hidden":false,"accept":null}
// an absent source, sandbox or accept is null and sets no attribute.
// It creates the
// element under `data-overlay="<id>"`, `data-stack="above"|"below"` and
// `data-pointer` when it takes pointer events, and wires its channel:
//   a media element reports through player.js's own sink
//   a frame reports every `message` event whose source is its contentWindow
// open(id) opens a file input's own picker
// post(id, payload) delivers to a frame's contentWindow with targetOrigin '*',
// because the frame's origin is opaque
// unmount(id) removes one element; unmountAll() removes every one
// node(id) is the DOM node, which is what player.js binds to
//
// A mount replaces whatever was mounted under the same id, so no id ever holds
// two elements and no channel outlives the element that opened it.

import * as player from './player.js';

const mounted = new Map();

function tagFor(kind) {
  if (kind === 'frame') {
    return 'iframe';
  }
  if (kind === 'file') {
    return 'input';
  }
  return kind === 'audio' ? 'audio' : 'video';
}

export function node(id) {
  const held = mounted.get(id);
  return held ? held.element : null;
}

export function mount(wanted, sink) {
  const { id, kind, stacking, pointer, source, sandbox, hidden, accept } =
    JSON.parse(wanted);
  unmount(id);

  const element = document.createElement(tagFor(kind));
  element.dataset.overlay = id;
  element.dataset.stack = stacking === 'above' ? 'above' : 'below';
  if (pointer) {
    element.dataset.pointer = '';
  }
  if (hidden) {
    element.hidden = true;
  }

  let listener = null;
  if (kind === 'file') {
    element.type = 'file';
    element.hidden = true;
    element.style.display = 'none';
    if (accept !== null) {
      element.accept = accept;
    }
    element.addEventListener('change', () => {
      const file = element.files && element.files[0];
      if (!file) {
        return;
      }
      const reader = new FileReader();
      reader.onload = () => {
        const bytes = new Uint8Array(reader.result);
        let binary = '';
        for (let i = 0; i < bytes.length; i += 1) {
          binary += String.fromCharCode(bytes[i]);
        }
        sink(
          JSON.stringify({
            name: file.name,
            mime: file.type,
            size: file.size,
            data: btoa(binary),
          }),
        );
      };
      reader.readAsArrayBuffer(file);
    });
    document.body.appendChild(element);
  } else if (kind === 'frame') {
    if (sandbox !== null) {
      element.setAttribute('sandbox', sandbox);
    }
    element.setAttribute('referrerpolicy', 'no-referrer');
    if (source !== null) {
      element.src = source;
    }
    document.body.appendChild(element);
    listener = (event) => {
      if (!element.contentWindow || event.source !== element.contentWindow) {
        return;
      }
      const payload =
        typeof event.data === 'string' ? event.data : JSON.stringify(event.data);
      sink(payload);
    };
    window.addEventListener('message', listener);
  } else {
    element.controls = false;
    element.autoplay = false;
    element.playsInline = true;
    element.preload = 'auto';
    element.crossOrigin = 'same-origin';
    document.body.appendChild(element);
    player.bind(element, sink);
  }

  mounted.set(id, { element, kind, listener });
}

export function post(id, payload) {
  const held = mounted.get(id);
  if (!held || held.kind !== 'frame' || !held.element.contentWindow) {
    return;
  }
  held.element.contentWindow.postMessage(payload, '*');
}

export function unmount(id) {
  const held = mounted.get(id);
  if (!held) {
    return;
  }
  mounted.delete(id);
  if (held.listener) {
    window.removeEventListener('message', held.listener);
  }
  if (held.kind !== 'frame' && held.kind !== 'file') {
    player.unbind();
  }
  held.element.remove();
}

export function open(id) {
  const held = mounted.get(id);
  if (!held || held.kind !== 'file') {
    return;
  }
  held.element.value = '';
  held.element.click();
}

export function unmountAll() {
  for (const id of Array.from(mounted.keys())) {
    unmount(id);
  }
}
