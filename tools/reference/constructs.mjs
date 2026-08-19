#!/usr/bin/env node

// Rewrites reference/constructs.tsv and jellium-model/src/construct.rs out of a
// checkout of the revision reference/PINNED names. One row per construct each
// page of the reference draws, in the order that page draws it.
//
//     node tools/reference/constructs.mjs <jellyfin-web-checkout>

import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { apiclient, checkedOut, locked, pinned } from './pinned.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');

// A key the markup writes, carried through concatenation as one character no
// markup spells, so that parsing the assembled text finds it where the
// reference wrote it.
const OPEN = '\u0001';
const SHUT = '\u0002';
const NEXT = '\u0003';

function keyed(keys) {
    return keys.length ? `${OPEN}${keys.join(NEXT)}${SHUT}` : '';
}

// What the run records about the reference rather than about itself: the depth
// each assignment chain reaches, and every element the reference leaves open.
// Both are facts of the reference, so the run states them and carries on.
const noted = new Set();

function note(line) {
    noted.add(line);
}

function keysIn(text) {
    const held = [];
    const pattern = new RegExp(`${OPEN}([^${SHUT}]*)${SHUT}`, 'g');
    for (const found of text.matchAll(pattern)) {
        held.push(found[1].split(NEXT).filter(Boolean));
    }
    return held;
}


// Which container of which page an entry module writes into.
//
// This map anchors a mount and names nothing else. It proved incomplete the
// first time it was run as a list of every writing module, because the seven
// modules under `src/components/homesections/sections/` write the home page's
// section titles and its Live TV buttons and no hand-written list caught them.
// It is therefore an anchor, and `reached` below finds the rest.
//
// The generator refuses a page whose named container its own markup does not
// carry, so an anchor cannot rot silently.
export const MOUNTS = {
    // the fixed header and the navigation drawer, on every signed-in page
    '*': [
        ['.skinHeader', 'src/scripts/libraryMenu.js'],
        ['.mainDrawer', 'src/scripts/libraryMenu.js']
    ],
    home: [
        ['.tabContent[data-index="0"] .sections', 'src/components/homesections/homesections.js'],
        ['.tabContent[data-index="1"] .sections', 'src/controllers/favorites.js']
    ]
};

// The classes that name a construct, hyphenated to give the construct's name.
//
// AUTHORED HERE, for the same reason and under the same guard: no mechanical
// rule over an element's class list picks the right one at every site —
// `headerButton headerButtonLeft headerBackButton hide` wants the third,
// `centerMessage padded-left padded-right` wants the first. `constructs`
// refuses an element that is a construct and carries no class named here, and
// refuses one carrying two, so the list cannot silently miss a construct or
// choose between two. Growing it is a loop, not a blocker: run the generator,
// read the refusal, add the class it names, run again.
export const NAMES = [
    'centerMessage', 'headerAudioPlayerButton', 'headerBackButton',
    'headerCastButton', 'headerHomeButton', 'headerSearchButton',
    'headerSyncButton', 'headerTabs', 'headerUserButton', 'itemsContainer',
    'mainDrawer', 'mainDrawerButton', 'navMenuOption', 'navMenuOptionIcon',
    'navMenuOptionText', 'pageTitle', 'raised', 'sectionTitle-cards',
    'sectionTitleContainer-cards', 'sectionTitleTextButton', 'sidebarHeader',
    'skinHeader', 'verticalSection'
];

// The tags that carry a key of their own rather than handing it upward.
const HEADINGS = ['h1', 'h2', 'h3', 'p'];

// The reference's own predicates, resolved for the client this project is,
// each standing on a decision already taken. Markup written only under a branch
// that resolves false is not a row.
export const RESOLVED = {
    // ADR 0001: no NativeShell, so no exit control
    'appHost.supports(AppFeature.ExitMenu)': false,
    'appHost.supports(AppFeature.MultiServer)': true,
    // ADR 0053: the band is the browser's, and a CLI-launched browser is not a
    // television
    'layoutManager.tv': false,
    'layoutManager.desktop': true,
    'browser.safari': false,
    // the home sections a default user sees, read from the reference's own
    // DEFAULT_SECTIONS
    homeSections: 'DEFAULT_SECTIONS'
};

// Where the construct table stops.
//
// An `is="emby-itemscontainer"` element is a construct and a row; what fills it
// is not. The reference fills one from `cardBuilder.getCardsHtml`, whose markup
// is a card's — an item's parts, not a page's — and which the same builder
// writes into twenty pages from options that vary per section rather than per
// page.
//
// Cards are outside this table on the merits, not by omission. A card is the
// most heavily ported construct in this tree: `reference/provenance.tsv`
// carries fifty-three `card-*` rows, each digest-checked against its span, and
// `reference/breakpoints.tsv` carries 16,560 oracle rows over card geometry
// alone. A card's parts are also unnameable per page by this client's own model
// — `jellium-web/src/widget.rs` draws every card and names itself
// `Names::Caller`, because a card's parts follow its `card::Drawing` and not the
// page it stands on.
//
// The boundary is therefore the container, and it is drawn where the reference
// itself draws it.
const ITEM_CONTAINER = 'emby-itemscontainer';

// The class the reference writes on that same container, which is what an
// assignment names it by when it fills one from a variable rather than a
// literal.
const ITEM_CONTAINER_CLASS = 'itemsContainer';

// One thing this boundary moves, and it is stated rather than lost. The
// requirements note asks the construct gate to expect the BlurHash placeholder
// drawn ahead of the image inside the same image container. That container is a
// card's, so it falls outside this table. The placeholder is gated instead by
// `blurhash-decode`, `blurhash-punch` and `blurhash-stretch`, the three rows
// whose values `every_cited_span_holds_the_value_that_cites_it` measures — the
// square the decode fills, the scaling it is decoded at, and the whole of both
// axes the decode is drawn over — and by a line of `docs/verification.md`'s
// home-screen subsection. That is a weaker gate than the note asked for, in
// exchange for a boundary the client can actually name, and it is written here
// so no reader takes it for an oversight.

// The `is` values that make an element a construct in its own right, whatever
// classes it carries.
const BUILT = new Set([
    'emby-linkbutton', 'emby-button', 'emby-itemscontainer', 'emby-scroller',
    'paper-icon-button-light'
]);

// The tags and `is` values the reference wraps a navigation in, which is the
// three of `BUILT` a user presses and not the two that hold what they scroll.
const LINKS = new Set(['a', 'button']);
const PRESSED = new Set([
    'emby-linkbutton', 'emby-button', 'paper-icon-button-light'
]);

// Tags that close themselves, which the tokenizer must not descend into.
const VOID = new Set([
    'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link', 'meta',
    'param', 'source', 'track', 'wbr'
]);

function hyphenate(name) {
    return name
        .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
        .replace(/([A-Z]+)([A-Z][a-z])/g, '$1-$2')
        .toLowerCase()
        .replace(/-+/g, '-')
        .replace(/^-|-$/g, '');
}

function variant(name) {
    return name
        .split('-')
        .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
        .join('');
}

// One element of a parsed fragment.
class Node {
    constructor(tag, attrs) {
        this.tag = tag;
        this.attrs = attrs;
        this.children = [];
        this.text = '';
        this.parent = null;
        this.unclosed = false;
    }

    get classes() {
        return (this.attrs.class ?? '').split(/\s+/).filter(Boolean);
    }
}

// `text` read as a run of tags and the text standing between them.
function tokens(text) {
    const held = [];
    const pattern = /<(\/?)([a-zA-Z][-a-zA-Z0-9]*)((?:[^>"']|"[^"]*"|'[^']*')*?)(\/?)>/g;
    let at = 0;
    let found;
    while ((found = pattern.exec(text)) !== null) {
        held.push({ text: text.slice(at, found.index) });
        at = pattern.lastIndex;
        const [, shut, tag, rest, empty] = found;
        held.push({
            shut: Boolean(shut),
            tag: tag.toLowerCase(),
            rest,
            empty: Boolean(empty)
        });
    }
    held.push({ text: text.slice(at) });
    return held;
}

// Whether the element opening at `at` has an end tag of its own further on.
function shuts(held, at) {
    const tag = held[at].tag;
    let depth = 0;
    for (let one = at + 1; one < held.length; one += 1) {
        const token = held[one];
        if (token.tag !== tag) {
            continue;
        }
        if (token.shut) {
            if (depth === 0) {
                return true;
            }
            depth -= 1;
        } else if (!token.empty) {
            depth += 1;
        }
    }
    return false;
}

// `text` parsed as a fragment of markup. An element closes at its own end tag,
// and one the reference leaves open takes no children rather than swallowing
// what follows, which would give the elements after it a role that is not
// theirs. Each such element is marked, and `balanced` names them.
function parse(text) {
    const held = tokens(text);
    const holder = new Node('#fragment', {});
    let open = holder;
    for (let at = 0; at < held.length; at += 1) {
        const token = held[at];
        if (token.text !== undefined) {
            open.text += token.text;
            continue;
        }
        if (token.shut) {
            for (let one = open; one !== holder; one = one.parent) {
                if (one.tag === token.tag) {
                    open = one.parent;
                    break;
                }
            }
            continue;
        }
        const node = new Node(token.tag, attributes(token.rest));
        node.parent = open;
        open.children.push(node);
        if (token.empty || VOID.has(token.tag)) {
            continue;
        }
        if (shuts(held, at)) {
            open = node;
        } else {
            node.unclosed = true;
        }
    }
    return holder;
}

// Every element of `fragment` the reference left open, which the parser closed
// where it stood rather than nesting what follows inside it.
export function balanced(fragment) {
    const held = [];
    const walk = (node) => {
        if (node.unclosed) {
            held.push(`<${node.tag}${node.classes.length ? ` class="${node.classes.join(' ')}"` : ''}>`);
        }
        for (const child of node.children) {
            walk(child);
        }
    };
    walk(fragment);
    return held;
}

function attributes(rest) {
    const held = {};
    const pattern = /([-a-zA-Z0-9_:@]+)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+)))?/g;
    let found;
    while ((found = pattern.exec(rest)) !== null) {
        held[found[1].toLowerCase()] = found[2] ?? found[3] ?? found[4] ?? '';
    }
    return held;
}

// Whether the element is the container this table stops at, whose contents are
// a card's parts and not a page's.
function filling(node) {
    return (
        node.attrs.is === ITEM_CONTAINER
        || node.classes.includes(ITEM_CONTAINER_CLASS)
    );
}

// Whether the element is one the reference wraps a navigation in.
function navigating(node) {
    return LINKS.has(node.tag) || PRESSED.has(node.attrs.is ?? '');
}

// The keys an element writes in its own attributes or its own direct text.
function written(node) {
    const held = [];
    for (const [name, value] of Object.entries(node.attrs)) {
        if (name === 'class') {
            continue;
        }
        held.push(...keysIn(value));
    }
    held.push(...keysIn(node.text));
    return held;
}

// The one name among `hits` that every other extends, which is what a modifier
// class such as `sectionTitle-cards` is against the class it modifies. Nothing
// where two hits modify nothing of each other, so that a genuine ambiguity is
// still refused.
function specific(hits) {
    if (hits.length === 0) {
        return null;
    }
    const held = hits.reduce((one, two) => (one.length >= two.length ? one : two));
    return hits.every((hit) => held === hit || held.startsWith(`${hit}-`)) ? held : null;
}

// One row per element of `fragment` that is a construct, in document order.
//
// An element is a construct when any of these holds, and is not one otherwise,
// so nothing needs suppressing:
//   it carries a class named in `NAMES`;
//   it is `<a>` or `<button>`, or carries one of the `is` values above;
//   its own attributes or its own direct text write a key.
//
// Its name is the hyphenated form of its `NAMES` class. An element that is a
// construct and carries no class at all takes its nearest named ancestor's name
// followed by its own tag.
//
// Its role is `navigation` where it is a link or a button or stands inside one,
// `stated` where it writes a key and is not, `silent` otherwise.
export function constructs(fragment, site, named, spliced) {
    const rows = [];
    const hitsOf = (node) => node.classes.filter((held) => NAMES.includes(held));
    const constructed = (node) =>
        hitsOf(node).length > 0
        || LINKS.has(node.tag)
        || BUILT.has(node.attrs.is ?? '')
        || (HEADINGS.includes(node.tag) && written(node).length > 0);

    // Every key an element answers for: its own, and those of the descendants
    // that are not constructs in their own right.
    const carried = (node) => {
        const held = [...written(node)];
        for (const child of node.children) {
            if (!constructed(child)) {
                held.push(...carried(child));
            }
        }
        return held;
    };

    const walk = (node, ancestor, inside) => {
        const hits = hitsOf(node);
        const link = navigating(node);
        const below = inside || link;
        let name = ancestor;
        const isConstruct = constructed(node);
        if (isConstruct) {
            const chosen = specific(hits);
            if (hits.length > 1 && !chosen) {
                throw new Error(
                    `${site}: <${node.tag}> carries ${hits.length} names of its own, ${hits.join(' and ')}; NAMES must hold one`
                );
            }
            if (chosen) {
                name = hyphenate(chosen);
            } else if (BUILT.has(node.attrs.is ?? '')) {
                name = node.attrs.is;
            } else if (ancestor) {
                name = `${ancestor}-${node.tag}`;
            } else {
                throw new Error(
                    `${site}: <${node.tag} class="${node.classes.join(' ')}"> is a construct and NAMES holds none of its classes`
                );
            }
            const keys = carried(node);
            const role = below ? 'navigation' : keys.length ? 'stated' : 'silent';
            if (keys.length === 0) {
                rows.push({ construct: name, role, key: 'silent' });
            } else {
                for (const alternatives of keys) {
                    for (const key of alternatives) {
                        rows.push({ construct: name, role, key });
                    }
                }
            }
        }
        if (spliced) {
            rows.push(...spliced(node));
        }
        if (filling(node)) {
            return;
        }
        for (const child of node.children) {
            walk(child, name, below);
        }
    };
    for (const child of fragment.children) {
        walk(child, named ?? null, false);
    }
    return rows;
}

// Every `name = globalize.translate('Key')` binding the source makes, held as
// the set of keys the branches of one name choose between. A key an assignment
// sets after the markup is built is not markup and is not read here.
function bindings(source) {
    const held = new Map();
    const pattern = /(?:const|let|var)?\s*([A-Za-z_$][\w$]*)\s*=\s*globalize\.translate\(\s*'([^']+)'/g;
    let found;
    while ((found = pattern.exec(source)) !== null) {
        const keys = held.get(found[1]) ?? [];
        if (!keys.includes(found[2])) {
            keys.push(found[2]);
        }
        held.set(found[1], keys);
    }
    return held;
}

// One expression of an accumulation, read for the keys it writes and the text
// it spells. Anything neither a literal nor a key contributes nothing, because
// a value the reference computes is not markup.
function value(text, bound) {
    const trimmed = text.trim();
    const call = /^globalize\.translate\(\s*(?:'([^']+)'|([A-Za-z_$][\w$]*))\s*[),]/.exec(trimmed);
    if (call) {
        return keyed(call[1] ? [call[1]] : (bound.get(call[2]) ?? []));
    }
    const name = /^([A-Za-z_$][\w$]*)$/.exec(trimmed);
    if (name && bound.has(name[1])) {
        return keyed(bound.get(name[1]));
    }
    return '';
}

// `text` split at each `+` that stands outside a literal, a template and a
// bracket.
function summands(text) {
    const parts = [];
    let held = '';
    let depth = 0;
    for (let at = 0; at < text.length; at += 1) {
        const letter = text[at];
        if (letter === '\'' || letter === '"' || letter === '`') {
            const shut = literal(text, at);
            held += text.slice(at, shut);
            at = shut - 1;
            continue;
        }
        if ('([{'.includes(letter)) {
            depth += 1;
        } else if (')]}'.includes(letter)) {
            depth -= 1;
        }
        if (letter === '+' && depth === 0) {
            parts.push(held);
            held = '';
            continue;
        }
        held += letter;
    }
    parts.push(held);
    return parts;
}

// Where the literal opening at `at` shuts, one past its closing quote.
function literal(text, at) {
    const quote = text[at];
    let held = at + 1;
    while (held < text.length) {
        if (text[held] === '\\') {
            held += 2;
            continue;
        }
        if (quote === '`' && text[held] === '$' && text[held + 1] === '{') {
            let depth = 1;
            held += 2;
            while (held < text.length && depth > 0) {
                if ('\'"`'.includes(text[held])) {
                    held = literal(text, held);
                    continue;
                }
                if (text[held] === '{') depth += 1;
                if (text[held] === '}') depth -= 1;
                held += 1;
            }
            continue;
        }
        if (text[held] === quote) {
            return held + 1;
        }
        held += 1;
    }
    return text.length;
}

// One literal's own text, its escapes cooked and each `${}` read for the keys
// it writes.
function cooked(text, bound) {
    const quote = text[0];
    const body = text.slice(1, -1);
    if (quote !== '`') {
        return body.replace(/\\(.)/g, (_, letter) => (letter === 'n' ? '\n' : letter));
    }
    let held = '';
    for (let at = 0; at < body.length; at += 1) {
        if (body[at] === '\\') {
            held += body[at + 1];
            at += 1;
            continue;
        }
        if (body[at] === '$' && body[at + 1] === '{') {
            let depth = 1;
            let shut = at + 2;
            while (shut < body.length && depth > 0) {
                if (body[shut] === '{') depth += 1;
                if (body[shut] === '}') depth -= 1;
                shut += 1;
            }
            held += value(body.slice(at + 2, shut - 1), bound);
            at = shut - 1;
            continue;
        }
        held += body[at];
    }
    return held;
}

// One expression's contribution to an accumulation.
function spelled(text, bound) {
    let held = '';
    for (const part of summands(text)) {
        const trimmed = part.trim();
        if (!trimmed) {
            continue;
        }
        if ('\'"`'.includes(trimmed[0]) && literal(trimmed, 0) === trimmed.length) {
            held += cooked(trimmed, bound);
            continue;
        }
        held += value(trimmed, bound);
    }
    return held;
}

// Where the expression opening at `at` ends: the first `;` or newline standing
// outside a literal and outside a bracket.
function expression(source, at) {
    let held = at;
    let depth = 0;
    while (held < source.length) {
        const letter = source[held];
        if ('\'"`'.includes(letter)) {
            held = literal(source, held);
            continue;
        }
        if ('([{'.includes(letter)) depth += 1;
        if (')]}'.includes(letter)) depth -= 1;
        if (depth === 0 && (letter === ';' || letter === '\n')) {
            break;
        }
        held += 1;
    }
    return held;
}

// Where the block whose brace stands at `at` shuts, one past its own brace.
function block(source, at) {
    let held = at;
    let depth = 0;
    while (held < source.length) {
        const letter = source[held];
        if ('\'"`'.includes(letter)) {
            held = literal(source, held);
            continue;
        }
        if (letter === '{') depth += 1;
        if (letter === '}') {
            depth -= 1;
            if (depth === 0) {
                return held + 1;
            }
        }
        held += 1;
    }
    return source.length;
}

// `source` with the span from `from` to `to` spelling nothing, its length and
// its lines kept so that every offset already taken still stands.
function blanked(source, from, to) {
    return (
        source.slice(0, from)
        + source.slice(from, to).replace(/[^\n]/g, ' ')
        + source.slice(to)
    );
}

// `source` with every branch `RESOLVED` settles taken: a branch that resolves
// false spells nothing, and a branch that resolves true stands on its own.
function settled(source) {
    let held = source;
    const pattern = /\bif\s*\(\s*(!?)\s*([^()]*(?:\([^()]*\))?[^()]*?)\s*\)\s*\{/g;
    let found;
    while ((found = pattern.exec(held)) !== null) {
        const stated = RESOLVED[found[2]];
        if (typeof stated !== 'boolean') {
            continue;
        }
        const opened = found.index + found[0].length - 1;
        const shut = block(held, opened);
        const otherwise = /^\s*else\s*\{/.exec(held.slice(shut));
        const elseOpened = otherwise ? shut + otherwise[0].length - 1 : -1;
        const elseShut = otherwise ? block(held, elseOpened) : -1;
        if (found[1] === '!' ? !stated : stated) {
            held = blanked(held, found.index, opened + 1);
            held = blanked(held, shut - 1, shut);
            if (otherwise) {
                held = blanked(held, shut, elseShut);
            }
        } else {
            held = blanked(held, found.index, shut);
            if (otherwise) {
                held = blanked(held, shut, elseOpened + 1);
                held = blanked(held, elseShut - 1, elseShut);
            }
        }
        pattern.lastIndex = found.index;
    }
    return held;
}

// Where each element variable the source binds stands: the class its own
// selector names, under the variable it was queried from.
function holders(source) {
    const held = new Map();
    const pattern =
        /([A-Za-z_$][\w$]*)\s*=\s*(document|[A-Za-z_$][\w$]*)\.querySelector\(\s*'\.([-\w]+)'\s*\)/g;
    let found;
    while ((found = pattern.exec(source)) !== null) {
        held.set(found[1], {
            under: found[2] === 'document' ? null : found[2],
            name: found[3]
        });
    }
    return held;
}

// The classes standing between the document and one element variable, outermost
// first, and nothing for a variable the source binds no selector to.
function standing(held, name) {
    if (!held.has(name)) {
        return null;
    }
    const chain = [];
    let at = name;
    while (at && held.has(at)) {
        chain.unshift(held.get(at).name);
        at = held.get(at).under;
    }
    return chain;
}

// Every accumulation the source writes into an element, in source order: the
// variable naming that element, and the markup the accumulation spells.
//
// The accumulation is read per container rather than flattened over the module,
// because `libraryMenu.js` writes the header, the drawer and the drawer's own
// library rows from three functions of one file and each lands somewhere else.
function regions(source, bound) {
    const held = [];
    const opened = new Map();
    const sites = /([A-Za-z_$][\w$]*)\.innerHTML\s*=\s*([A-Za-z_$][\w$]*)\s*;/g;
    let site;
    while ((site = sites.exec(source)) !== null) {
        const [, target, name] = site;
        const from = opened.get(name) ?? 0;
        const to = site.index;
        opened.set(name, to);
        const pattern = new RegExp(`\\b${name}\\s*(\\+?)=(?!=)\\s*`, 'g');
        pattern.lastIndex = from;
        let text = '';
        let found;
        while ((found = pattern.exec(source)) !== null && found.index < to) {
            const start = found.index + found[0].length;
            const end = expression(source, start);
            const spoken = spelled(source.slice(start, end), bound);
            text = found[1] ? text + spoken : spoken;
            pattern.lastIndex = end;
        }
        held.push({ target, text });
    }
    return held;
}

// The markup a module builds where it writes into no element of its own, taken
// from every accumulation it makes, in source order.
function accumulated(source, bound) {
    let held = '';
    const pattern = /([A-Za-z_$][\w$]*)\s*\+=\s*/g;
    let found;
    while ((found = pattern.exec(source)) !== null) {
        const opened = found.index + found[0].length;
        const at = expression(source, opened);
        held += spelled(source.slice(opened, at), bound);
        pattern.lastIndex = at;
    }
    return held;
}

// The markup a `.tsx` module returns, its JSX read as markup: a brace holding a
// key writes that key, every other brace writes nothing, and `className` is the
// class attribute React spells that way.
function returned(source) {
    const bound = bindings(source);
    let held = '';
    let at = source.indexOf('return (');
    if (at < 0) {
        return '';
    }
    at += 'return ('.length;
    let depth = 1;
    while (at < source.length && depth > 0) {
        const letter = source[at];
        if (letter === '(') depth += 1;
        else if (letter === ')') depth -= 1;
        if (depth === 0) break;
        held += letter;
        at += 1;
    }
    held = components(held);
    held = held.replace(/className=/g, 'class=');
    held = held.replace(/=\{([^{}]*)\}/g, (_, expr) => `="${value(expr, bound)}"`);
    held = held.replace(/\{([^{}]*)\}/g, (_, expr) => value(expr, bound));
    return held;
}


// A capitalised JSX tag is a component rather than an element of the
// reference's markup; its own markup is reached through its own module. Its tag
// falls away and the keys its own props write stand where it stood.
function components(text) {
    let held = '';
    let at = 0;
    while (at < text.length) {
        const letter = text[at];
        if (letter !== '<') {
            held += letter;
            at += 1;
            continue;
        }
        const name = /^<\/?([A-Za-z][\w.]*)/.exec(text.slice(at));
        if (!name || !/^[A-Z]/.test(name[1])) {
            held += letter;
            at += 1;
            continue;
        }
        let shut = at + 1;
        let depth = 0;
        while (shut < text.length) {
            if ('\'"`'.includes(text[shut])) {
                shut = literal(text, shut);
                continue;
            }
            if (text[shut] === '{') depth += 1;
            else if (text[shut] === '}') depth -= 1;
            else if (text[shut] === '>' && depth === 0) {
                shut += 1;
                break;
            }
            shut += 1;
        }
        const whole = text.slice(at, shut);
        for (const found of whole.matchAll(/globalize\.translate\(\s*'([^']+)'/g)) {
            held += keyed([found[1]]);
        }
        at = shut;
    }
    return held;
}


// The sections a default user is shown, read out of the reference's own
// DEFAULT_SECTIONS rather than written here.
function defaultSections(checkout) {
    const source = readFileSync(join(checkout, 'src/types/homeSectionType.ts'), 'utf8');
    const named = source.indexOf('DEFAULT_SECTIONS');
    const opened = source.indexOf('[', source.indexOf('=', named));
    const body = source.slice(opened, source.indexOf(']', opened));
    return [...body.matchAll(/HomeSectionType\.(\w+)/g)].map((found) => found[1]);
}

// Where a module specifier resolves in the checkout, and nothing for one that
// resolves outside the reference's own source.
function resolved(checkout, from, specifier) {
    const bases = specifier.startsWith('.')
        ? [join(dirname(from), specifier)]
        : [join('src', specifier)];
    for (const base of bases) {
        for (const suffix of ['.ts', '.js', '.tsx', '/index.ts', '/index.js', '/index.tsx']) {
            try {
                readFileSync(join(checkout, `${base}${suffix}`));
                return `${base}${suffix}`;
            } catch {
                continue;
            }
        }
    }
    return null;
}

// Where the parenthesis opening at `at` shuts, one past its own.
function through(source, at) {
    let held = at;
    let depth = 0;
    while (held < source.length) {
        const letter = source[held];
        if ('\'"`'.includes(letter)) {
            held = literal(source, held);
            continue;
        }
        if (letter === '(') depth += 1;
        if (letter === ')') {
            depth -= 1;
            if (depth === 0) {
                return held + 1;
            }
        }
        held += 1;
    }
    return source.length;
}

// The functions a module exports that write markup into an element their caller
// hands them, which is how the reference assigns one module's markup into a
// container another module owns.
//
// A function that answers markup for its caller to do as it likes with is not
// one of these, which is what separates `resume.ts`'s `loadResume` from
// `cardBuilder`'s `getCardsHtml`.
// Every function a module declares, with the parameters it takes, the body it
// holds, and whether the module exports it.
function declared(source) {
    const held = new Map();
    const pattern = /(export\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\s*\(/g;
    let found;
    while ((found = pattern.exec(source)) !== null) {
        const opened = found.index + found[0].length - 1;
        const shut = through(source, opened);
        const brace = source.indexOf('{', shut);
        held.set(found[2], {
            exported: Boolean(found[1]),
            params: source
                .slice(opened + 1, shut - 1)
                .split(',')
                .map((one) => one.trim().split(/[\s=:]/)[0])
                .filter((one) => /^[A-Za-z_$][\w$]*$/.test(one)),
            body: source.slice(shut, block(source, brace))
        });
        pattern.lastIndex = shut;
    }
    return held;
}

// Whether `holder` hands one of its own parameters to a call of `name`.
function handing(holder, name) {
    const pattern = new RegExp(`\\b${name}\\s*\\(`, 'g');
    let found;
    while ((found = pattern.exec(holder.body)) !== null) {
        const opened = found.index + found[0].length - 1;
        const args = holder.body.slice(opened, through(holder.body, opened));
        if (holder.params.some((one) => new RegExp(`\\b${one}\\b`).test(args))) {
            return true;
        }
        pattern.lastIndex = opened + 1;
    }
    return false;
}

// The functions a module exports that write markup into an element their caller
// hands them, directly or through the module's own helpers, which is how the
// reference assigns one module's markup into a container another module owns.
//
// A function that answers markup for its caller to do as it likes with is not
// one of these, which is what separates `resume.ts`'s `loadResume` from
// `cardBuilder`'s `getCardsHtml`.
function assigning(source) {
    const all = declared(source);
    const writes = new Set();
    for (const [name, held] of all) {
        if (
            held.params.some(
                (one) =>
                    one !== ITEM_CONTAINER_CLASS
                    && new RegExp(`\\b${one}\\.innerHTML\\s*=`).test(held.body)
            )
        ) {
            writes.add(name);
        }
    }
    for (let more = true; more; ) {
        more = false;
        for (const [name, held] of all) {
            if (writes.has(name)) {
                continue;
            }
            if ([...writes].some((one) => handing(held, one))) {
                writes.add(name);
                more = true;
            }
        }
    }
    return new Set([...writes].filter((name) => all.get(name).exported));
}

// The names a module's own `switch` dispatches to under a case value RESOLVED
// excludes, and under no case value it admits.
function excluded(checkout, source) {
    const shown = new Set(defaultSections(checkout));
    const arms = /case\s+HomeSectionType\.(\w+)\s*:([^]*?)(?=\n\s*case\s|\n\s*default\s*:)/g;
    const held = new Set();
    for (const found of source.matchAll(arms)) {
        if (shown.has(found[1])) {
            continue;
        }
        for (const call of found[2].matchAll(/([A-Za-z_$][\w$]*)\s*\(/g)) {
            held.add(call[1]);
        }
    }
    for (const found of source.matchAll(arms)) {
        if (!shown.has(found[1])) {
            continue;
        }
        for (const call of found[2].matchAll(/([A-Za-z_$][\w$]*)\s*\(/g)) {
            held.delete(call[1]);
        }
    }
    return held;
}

// Every module whose markup lands in a mounted container, each under the chain
// of assignments that reaches it.
//
// The walk is over assignment, not over import. A module is reached when the
// module reaching it calls a function of that module which writes into an
// element the caller hands it; a module merely imported contributes nothing.
// That is what holds the walk at one level under the home page's own sections
// without a rule about levels: each of them imports
// `components/cardbuilder/cardBuilder` and not one of them assigns its markup.
//
// Three further edges: a bare side-effect import is not followed, because a
// custom element upgrades markup the page already carries; a module reached only
// through a `switch` arm whose case value RESOLVED excludes is not walked; and
// markup assigned into an item container is not read, whoever assigns it.
export function reached(checkout, entry) {
    const held = [];
    const seen = new Set();
    const walk = (module, chain) => {
        if (seen.has(module)) {
            return;
        }
        seen.add(module);
        held.push({ module, chain });
        let source;
        try {
            source = settled(readFileSync(join(checkout, module), 'utf8'));
        } catch {
            return;
        }
        const withheld = excluded(checkout, source);
        for (const found of source.matchAll(/import\s+([^;]*?)\s*from\s*'([^']+)'/g)) {
            const bound = [...found[1].matchAll(/([A-Za-z_$][\w$]*)/g)].map((one) => one[1]);
            if (bound.length > 0 && bound.every((one) => withheld.has(one))) {
                continue;
            }
            const path = resolved(checkout, module, found[2]);
            if (!path) {
                continue;
            }
            let deeper;
            try {
                deeper = settled(readFileSync(join(checkout, path), 'utf8'));
            } catch {
                continue;
            }
            const writes = [...assigning(deeper)].filter((name) => !withheld.has(name));
            if (!writes.some((name) => new RegExp(`\\b${name}\\s*\\(`).test(source))) {
                continue;
            }
            walk(path, [...chain, path]);
        }
    };
    walk(entry, [entry]);
    return held;
}

// The keys a reached module's own parameters stand for, taken from the string
// literals the module reaching it passes at each call site, because the
// reference writes `globalize.translate(title)` and binds `title` at the call.
function passed(checkout, entry, module) {
    const caller = readFileSync(join(checkout, entry), 'utf8');
    const source = readFileSync(join(checkout, module), 'utf8');
    const held = new Map();
    for (const [name, one] of declared(source)) {
        if (!one.exported) {
            continue;
        }
        const pattern = new RegExp(`\\b${name}\\s*\\(`, 'g');
        let call;
        while ((call = pattern.exec(caller)) !== null) {
            const opened = call.index + call[0].length - 1;
            const shut = through(caller, opened);
            caller.slice(opened + 1, shut - 1)
                .split(',')
                .map((word) => word.trim())
                .forEach((word, at) => {
                    const spoken = /^'([^']+)'$/.exec(word);
                    if (!spoken || !one.params[at]) {
                        return;
                    }
                    const keys = held.get(one.params[at]) ?? [];
                    if (!keys.includes(spoken[1])) {
                        keys.push(spoken[1]);
                    }
                    held.set(one.params[at], keys);
                });
            pattern.lastIndex = shut;
        }
    }
    return held;
}

// Every markup fragment one module writes, in source order, each under the
// container the module writes it into. Refuses a carrier it cannot parse rather
// than answering an empty list.
export function fragments(checkout, module, extra) {
    const source = readFileSync(join(checkout, module), 'utf8');
    if (module.endsWith('.html')) {
        return [{ chain: null, fragment: parse(source) }];
    }
    if (module.endsWith('.tsx')) {
        const written = returned(source);
        if (!/<[a-zA-Z]/.test(written)) {
            throw new Error(`${module} writes no markup this tool can read`);
        }
        return [{ chain: null, fragment: parse(written) }];
    }
    const taken = settled(source);
    const bound = new Map([...bindings(taken), ...(extra ?? new Map())]);
    const under = holders(taken);
    const parts = regions(taken, bound)
        .filter((one) => /<[a-zA-Z]/.test(one.text))
        .map((one) => ({
            into: one.target,
            chain: standing(under, one.target),
            fragment: parse(one.text)
        }))
        .filter(
            (one) =>
                one.into !== ITEM_CONTAINER_CLASS
                && one.chain?.[one.chain.length - 1] !== ITEM_CONTAINER_CLASS
        );
    if (parts.length > 0) {
        return parts;
    }
    const text = accumulated(taken, bound);
    if (!/<[a-zA-Z]/.test(text)) {
        throw new Error(`${module} writes no markup this tool can read`);
    }
    return [{ chain: null, fragment: parse(text) }];
}

// One module's fragments as one, for a reader that asks what the module writes
// rather than where each piece of it lands.
export function markup(checkout, module, extra) {
    const parts = fragments(checkout, module, extra);
    if (parts.length === 1) {
        return parts[0].fragment;
    }
    const holder = new Node('#fragment', {});
    for (const part of parts) {
        for (const child of part.fragment.children) {
            child.parent = holder;
            holder.children.push(child);
        }
    }
    return holder;
}

// The tab names a tabbed page writes, taken from its controller's own
// `getTabs`, in the order it returns them.
export function tabs(checkout, module) {
    const source = readFileSync(join(checkout, module), 'utf8');
    const at = source.indexOf('getTabs()');
    if (at < 0) {
        return [];
    }
    let depth = 0;
    let shut = source.indexOf('{', at);
    let held = shut;
    while (held < source.length) {
        if (source[held] === '{') depth += 1;
        if (source[held] === '}') {
            depth -= 1;
            if (depth === 0) break;
        }
        held += 1;
    }
    const body = source.slice(shut, held);
    return [...body.matchAll(/globalize\.translate\(\s*'([^']+)'/g)].map((found) => found[1]);
}

// A page's own name, taken from the route path its route table gives it: the
// path's separators become hyphens and its parameters fall away, because a
// parameter names an item rather than a page.
function paged(path) {
    const name = path
        .split('/')
        .filter((part) => part && !part.startsWith(':'))
        .join('-')
        .toLowerCase();
    return name || 'dashboard';
}

// Every route of one table, read from the object literals it holds.
function table(source, held) {
    const rows = [];
    const pattern = /path:\s*'([^']*)'/g;
    let found;
    while ((found = pattern.exec(source)) !== null) {
        const rest = source.slice(found.index, found.index + 400);
        const view = /view:\s*'([^']+)'/.exec(rest);
        const controller = /controller:\s*'([^']+)'/.exec(rest);
        const page = /page:\s*'([^']+)'/.exec(rest);
        rows.push({
            path: found[1],
            page: paged(found[1]),
            view: view ? `src/controllers/${view[1]}` : null,
            controller: controller ? `src/controllers/${controller[1]}.js` : null,
            module: page ? page[1] : found[1],
            held
        });
    }
    return rows;
}

// The reference's own route tables, read from the checkout. Each row gives a
// route path and the view or page module that paints it.
export function routes(checkout) {
    const read = (path) => readFileSync(join(checkout, path), 'utf8');
    const rows = [
        ...table(read('src/apps/stable/routes/legacyRoutes/user.ts'), 'stable'),
        ...table(read('src/apps/stable/routes/legacyRoutes/public.ts'), 'stable'),
        ...table(read('src/apps/stable/routes/asyncRoutes/user.ts'), 'stable'),
        ...table(read('src/apps/stable/routes/asyncRoutes/public.ts'), 'stable'),
        ...table(read('src/apps/dashboard/routes/_legacyRoutes.ts'), 'dashboard'),
        ...table(read('src/apps/dashboard/routes/_asyncRoutes.ts'), 'dashboard'),
        ...table(read('src/apps/wizard/routes/routes.tsx'), 'wizard')
    ];
    for (const row of rows) {
        if (!row.view) {
            row.async = `src/apps/${row.held}/routes/${row.module}`;
        }
    }
    return rows;
}

// The class list on a page's root element, which is what decides the room the
// page reserves above itself.
export function pageClass(checkout, route) {
    const root = rootOf(checkout, route);
    const classes = root ? root.classes : [];
    if (route.held === 'wizard') return 'Wizard';
    if (route.held === 'dashboard') return 'Modern';
    if (classes.includes('libraryPage')) {
        return classes.includes('noSecondaryNavPage') ? 'Library' : 'LibraryWithNav';
    }
    if (classes.includes('itemDetailPage')) return 'ItemDetail';
    return 'Standalone';
}

function rootOf(checkout, route) {
    const carrier = route.view ?? found(checkout, route.async);
    if (!carrier) {
        return null;
    }
    try {
        const fragment = markup(checkout, carrier);
        return fragment.children.find((node) => node.tag !== '#fragment') ?? null;
    } catch {
        return null;
    }
}

// The file an async route's page module names, which the reference writes
// either as a module or as a directory holding an index.
function found(checkout, module) {
    for (const suffix of ['.tsx', '.ts', '/index.tsx', '/index.ts']) {
        const path = `${module}${suffix}`;
        try {
            readFileSync(join(checkout, path));
            return path;
        } catch {
            continue;
        }
    }
    return null;
}

// Whether an element answers a mount's selector, whose last compound names the
// container and whose earlier compounds name an ancestor of it.
function matches(node, selector) {
    const compounds = selector.trim().split(/\s+/);
    const answers = (held, compound) => {
        for (const part of compound.matchAll(/\.([-\w]+)|\[([-\w]+)="([^"]*)"\]/g)) {
            if (part[1] && !held.classes.includes(part[1])) {
                return false;
            }
            if (part[2] && held.attrs[part[2]] !== part[3]) {
                return false;
            }
        }
        return true;
    };
    if (!answers(node, compounds[compounds.length - 1])) {
        return false;
    }
    let held = node.parent;
    for (let at = compounds.length - 2; at >= 0; at -= 1) {
        while (held && !answers(held, compounds[at])) {
            held = held.parent;
        }
        if (!held) {
            return false;
        }
        held = held.parent;
    }
    return true;
}

// The rows every module reached from one entry writes into the container `cls`
// names, and nothing for the containers a mount of its own names.
//
// A fragment the module writes into a container standing inside this one is
// spliced at the element carrying that container's own class, so the drawer's
// library rows stand where the drawer writes their holder.
function mounted(checkout, entry, route, tabbed, cls) {
    const rows = [];
    for (const { module, chain } of reached(checkout, entry)) {
        const extra = module === entry ? undefined : passed(checkout, entry, module);
        let parts;
        try {
            parts = fragments(checkout, module, extra);
        } catch {
            continue;
        }
        if (chain.length > 2) {
            note(`reached by assignment: ${chain.join(' -> ')}`);
        }
        for (const part of parts) {
            for (const open of balanced(part.fragment)) {
                note(`${module} leaves ${open} open; the parser closes it where it stands`);
            }
        }
        const own = parts.filter((part) => (cls ? part.chain?.[0] === cls : !part.chain));
        const rest = parts.filter((part) => !own.includes(part) && part.chain);
        const splice = (node) => {
            const held = [...tabbed(node)];
            for (const part of rest) {
                if (node.classes.includes(part.chain[part.chain.length - 1])) {
                    held.push(...constructs(part.fragment, module, route.page, splice));
                }
            }
            return held;
        };
        for (const part of own) {
            rows.push(...constructs(part.fragment, module, route.page, splice));
        }
    }
    return rows;
}

// The class the last compound of a selector names, which is the container the
// selector points at.
function named(selector) {
    const compounds = selector.trim().split(/\s+/);
    const last = compounds[compounds.length - 1];
    const classes = [...last.matchAll(/\.([-\w]+)/g)].map((found) => found[1]);
    return classes.length > 0 ? classes[classes.length - 1] : null;
}

// The container a mount names is itself a construct, named by the class the
// selector's last compound gives it.
function container(selector) {
    const held = named(selector);
    return held && NAMES.includes(held) ? hyphenate(held) : null;
}

// One page's rows, in the order the page draws them: the header and the drawer
// first, then its own markup in document order with each mount's rows spliced
// in at the position of the container that mount names, and each tab strip's
// rows at the position of the `.headerTabs` element.
export function page(checkout, route) {
    const rows = [];
    const carrier = route.view ?? found(checkout, route.async);
    if (!carrier) {
        return rows;
    }
    let fragment;
    try {
        fragment = markup(checkout, carrier);
    } catch {
        return rows;
    }

    const strip = route.controller
        ? (() => {
            try {
                return tabs(checkout, route.controller);
            } catch {
                return [];
            }
        })()
        : [];
    const tabbed = (node) =>
        node.classes.includes('headerTabs')
            ? strip.map((key) => ({ construct: 'header-tabs', role: 'navigation', key }))
            : [];

    for (const [selector, entry] of MOUNTS['*'] ?? []) {
        const name = container(selector);
        if (name) {
            rows.push({ construct: name, role: 'silent', key: 'silent' });
        }
        rows.push(...mounted(checkout, entry, route, tabbed, named(selector)));
    }

    const own = MOUNTS[route.page] ?? [];
    for (const [selector] of own) {
        if (!holds(fragment, selector)) {
            throw new Error(`${carrier} carries no ${selector}, which MOUNTS names for ${route.page}`);
        }
    }
    const splice = (node) => {
        const held = [];
        for (const [selector, entry] of own) {
            if (matches(node, selector)) {
                held.push(...mounted(checkout, entry, route, tabbed, null));
            }
        }
        return held;
    };

    const root = fragment.children[0];
    const holder = new Node('#fragment', {});
    holder.children = root ? root.children : fragment.children;
    for (const child of holder.children) {
        child.parent = holder;
    }
    for (const open of balanced(fragment)) {
        note(`${carrier} leaves ${open} open; the parser closes it where it stands`);
    }
    rows.push(...constructs(holder, carrier, route.page, splice));
    return rows;
}

// Whether a page's own markup carries the container a mount names.
function holds(fragment, selector) {
    let held = false;
    const walk = (node) => {
        if (matches(node, selector)) {
            held = true;
        }
        for (const child of node.children) {
            walk(child);
        }
    };
    walk(fragment);
    return held;
}

function tsv(pages) {
    const lines = ['page\tconstruct\trole\tkey'];
    for (const [page, rows] of pages) {
        for (const row of rows) {
            lines.push(`${page}\t${row.construct}\t${row.role}\t${row.key}`);
        }
    }
    return `${lines.join('\n')}\n`;
}

// Rows identical in page, name, role and key collapse to one.
function collapsed(rows) {
    const held = [];
    const seen = new Set();
    for (const row of rows) {
        const at = `${row.construct}\t${row.role}\t${row.key}`;
        if (seen.has(at)) {
            continue;
        }
        seen.add(at);
        held.push(row);
    }
    return held;
}

function enumerated(name, doc, variants) {
    const body = variants.map((held) => `    ${held},`).join('\n');
    return `${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub enum ${name} {\n${body}\n}\n`;
}

function rust(pages, classes, keys) {
    const names = [...pages.keys()].sort();
    const drawnNames = new Set();
    for (const rows of pages.values()) {
        for (const row of rows) {
            drawnNames.add(row.construct);
        }
    }
    const arms = names
        .map((page) => `            Page::${variant(page)} => PageClass::${classes.get(page)},`)
        .join('\n');
    return `//! One page, one construct and one key of the pinned reference.
//!
//! Generated whole by \`just constructs\` from \`reference/constructs.tsv\` and
//! the reference's own string table. Nothing here is hand-written.

${enumerated(
        'Page',
        `/// One page of the pinned reference, named by the route its own route table\n/// gives it.\n/// One variant per distinct \`page\` of \`reference/constructs.tsv\`.`,
        names.map(variant)
    )}
/// The class the reference writes on a page's root element, which is what
/// decides the room the page reserves above itself.
/// \`Modern\` is the page carrying none of the legacy layout's own page
/// classes, which is every route the dashboard's react app paints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageClass {
    Library,
    /// A library page carrying a secondary nav, which is a library page whose
    /// root omits \`noSecondaryNavPage\`.
    LibraryWithNav,
    ItemDetail,
    Standalone,
    Wizard,
    Modern,
}

impl Page {
    /// Every page of the pinned reference, which is what a module drawing the
    /// chrome every page stands under names itself as.
    pub const ALL: &'static [Page] = &[
${names.map((page) => `        Page::${variant(page)},`).join('\n')}
    ];

    /// The class the page's own root element carries.
    /// Generated with the enum: the generator reads the class list off each
    /// page's root element and emits one match arm per page, so no arm is
    /// hand-written and no page is missed.
    pub fn class(self) -> PageClass {
        match self {
${arms}
        }
    }
}

${enumerated(
        'Construct',
        `/// One construct of the pinned reference, named as\n/// \`reference/constructs.tsv\` names it.\n/// One variant per distinct \`construct\` of that table, its name each\n/// hyphen-separated word capitalised.`,
        [...drawnNames].sort().map(variant)
    )}
${enumerated(
        'Sentence',
        `/// One key of the pinned reference's own string table, spelled as that table\n/// spells it, so no conversion stands between a key and its variant.\n/// One variant per key of \`src/strings/en-us.json\` at the pinned revision,\n/// which is every key rather than only the keys\n/// \`reference/constructs.tsv\` names, so a \`Text\` variant can name the\n/// reference's own sentence whether or not the construct table carries it.`,
        keys
    )}`;
}

const checkout = process.argv[2];
if (!checkout) {
    throw new Error('usage: node tools/reference/constructs.mjs <jellyfin-web-checkout>');
}

const revision = pinned();
checkedOut(checkout, revision.commit);
const release = apiclient();
locked(checkout, release.version, release.integrity);

const pages = new Map();
const classes = new Map();
const refused = [];
for (const route of routes(checkout)) {
    let rows;
    try {
        rows = collapsed(page(checkout, route));
    } catch (trouble) {
        refused.push(trouble.message);
        continue;
    }
    if (rows.length === 0) {
        continue;
    }
    pages.set(route.page, rows);
    classes.set(route.page, pageClass(checkout, route));
}

// Every refusal at once, so that answering them is one pass of the loop rather
// than one run per class.
if (refused.length > 0) {
    throw new Error(`${refused.length} refusals:\n${[...new Set(refused)].join('\n')}`);
}

for (const line of [...noted].sort()) {
    process.stderr.write(`${line}\n`);
}

const sorted = [...pages.entries()].sort(([one], [two]) => (one < two ? -1 : 1));
writeFileSync(join(root, 'reference', 'constructs.tsv'), tsv(sorted));

const strings = JSON.parse(
    readFileSync(join(checkout, 'src', 'strings', 'en-us.json'), 'utf8')
);
const keys = Object.keys(strings)
    .filter((key) => /^[A-Za-z][A-Za-z0-9]*$/.test(key))
    .sort();
writeFileSync(
    join(root, 'jellium-model', 'src', 'construct.rs'),
    rust(new Map(sorted), classes, keys)
);
