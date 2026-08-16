// The environment both the port and the sliced reference read.
//
// It is an authored input, not a recording: it names no browser and carries no
// browser's answers. One window, one document and one navigator are installed
// on globalThis when this module is imported, and every later install mutates
// those same objects, so a handle either side cached stays live.

class Node {}

class Element extends Node {
    constructor(tag) {
        super();
        this.tagName = tag.toUpperCase();
        this.style = { animationName: '' };
    }
}

class HTMLElement extends Element {}

class HTMLDivElement extends HTMLElement {}

class HTMLUnknownElement extends HTMLElement {}

class TextTrackList {
    get length() {
        return 0;
    }
}

class AudioTrackList {
    get length() {
        return 0;
    }
}

class HTMLMediaElement extends HTMLElement {
    canPlayType(type) {
        const table = this instanceof HTMLVideoElement ? state.video : state.audio;
        return Object.prototype.hasOwnProperty.call(table, type) ? table[type] : '';
    }
}

class HTMLVideoElement extends HTMLMediaElement {
    get textTracks() {
        return state.textTracks ? new TextTrackList() : null;
    }

    get audioTracks() {
        return state.audioTracks ? new AudioTrackList() : null;
    }
}

class HTMLAudioElement extends HTMLMediaElement {}

class CanvasRenderingContext2D {}

class HTMLCanvasElement extends HTMLElement {
    getContext(kind) {
        return kind === '2d' && state.canvas2d ? new CanvasRenderingContext2D() : null;
    }
}

class Document extends Node {
    constructor() {
        super();
        this.documentElement = new HTMLElement('html');
    }

    createElement(tag) {
        switch (tag) {
            case 'video':
                return new HTMLVideoElement(tag);
            case 'audio':
                return new HTMLAudioElement(tag);
            case 'canvas':
                return new HTMLCanvasElement(tag);
            case 'div':
                return new HTMLDivElement(tag);
            default:
                return new HTMLUnknownElement(tag);
        }
    }
}

class Navigator {}

class Screen {}

class AudioDestinationNode {}

class AudioContext {
    constructor() {
        this.destination = new AudioDestinationNode();
        this.destination.maxChannelCount = state.speakers;
    }
}

class MediaSource {}

class Window {}

// Every getter above reads this, so an element created before an install
// answers what the install asked for.
const state = {
    userAgent: '',
    platform: '',
    appVersion: '',
    maxTouchPoints: 0,
    hasTouchStart: false,
    tizenGlobal: false,
    animates: false,
    width: 0,
    height: 0,
    devicePixelRatio: 1,
    speakers: null,
    mediaSource: false,
    textTracks: false,
    canvas2d: false,
    audioTracks: false,
    video: {},
    audio: {}
};

const navigatorObject = new Navigator();
const documentObject = new Document();
const screenObject = new Screen();

function defined(host, name, value) {
    Object.defineProperty(host, name, {
        value,
        writable: true,
        enumerable: true,
        configurable: true
    });
}

function removed(host, name) {
    if (Object.prototype.hasOwnProperty.call(host, name)) {
        delete host[name];
    }
}

// window, document and navigator carry the prototypes web_sys::window()'s
// `instanceof` check requires, and globalThis is given Window.prototype.
export function install(spec) {
    Object.assign(state, spec);

    defined(navigatorObject, 'userAgent', state.userAgent);
    defined(navigatorObject, 'platform', state.platform);
    defined(navigatorObject, 'appVersion', state.appVersion);
    defined(navigatorObject, 'maxTouchPoints', state.maxTouchPoints);

    defined(documentObject.documentElement, 'animate', state.animates ? () => null : null);

    defined(screenObject, 'width', state.width);
    defined(screenObject, 'height', state.height);

    defined(globalThis, 'navigator', navigatorObject);
    defined(globalThis, 'document', documentObject);
    defined(globalThis, 'window', globalThis);
    defined(globalThis, 'screen', screenObject);
    defined(globalThis, 'devicePixelRatio', state.devicePixelRatio);

    defined(globalThis, 'Element', Element);
    defined(globalThis, 'HTMLElement', HTMLElement);
    defined(globalThis, 'HTMLDivElement', HTMLDivElement);
    defined(globalThis, 'HTMLMediaElement', HTMLMediaElement);
    defined(globalThis, 'HTMLVideoElement', HTMLVideoElement);
    defined(globalThis, 'HTMLAudioElement', HTMLAudioElement);
    defined(globalThis, 'HTMLCanvasElement', HTMLCanvasElement);
    defined(globalThis, 'CanvasRenderingContext2D', CanvasRenderingContext2D);
    defined(globalThis, 'TextTrackList', TextTrackList);
    defined(globalThis, 'Document', Document);
    defined(globalThis, 'Navigator', Navigator);
    defined(globalThis, 'Screen', Screen);
    defined(globalThis, 'AudioDestinationNode', AudioDestinationNode);
    defined(globalThis, 'Window', Window);

    if (state.hasTouchStart) {
        defined(globalThis, 'ontouchstart', null);
    } else {
        removed(globalThis, 'ontouchstart');
    }

    if (state.tizenGlobal) {
        defined(globalThis, 'tizen', {});
    } else {
        removed(globalThis, 'tizen');
    }

    if (state.mediaSource) {
        defined(globalThis, 'MediaSource', MediaSource);
    } else {
        removed(globalThis, 'MediaSource');
    }

    if (state.speakers === null) {
        removed(globalThis, 'AudioContext');
        removed(globalThis, 'webkitAudioContext');
    } else {
        defined(globalThis, 'AudioContext', AudioContext);
        removed(globalThis, 'webkitAudioContext');
    }

    removed(globalThis, 'NativeShell');

    if (Object.getPrototypeOf(globalThis) !== Window.prototype) {
        Object.setPrototypeOf(Window.prototype, Object.getPrototypeOf(globalThis));
        Object.setPrototypeOf(globalThis, Window.prototype);
    }
}

install({});
