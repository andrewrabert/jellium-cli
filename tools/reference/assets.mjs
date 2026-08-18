#!/usr/bin/env node

// Copies the pinned checkout's own font, icon and branding bytes into this
// tree, and records where each came from and what it digests to. Nothing here
// re-encodes a byte: every copied file is the checkout's file, and every table
// below is read out of the checkout's own stylesheets.
//
//     node tools/reference/assets.mjs <jellyfin-web-checkout>

import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { apiclient, checkedOut, locked, pinned } from './pinned.mjs';
import { span, texts } from './provenance.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const web = join(root, 'jellium-web');

// The eight subsets base-400-normal.scss and base-700-normal.scss declare, in
// the order the bundle embeds them.
const SUBSETS = [
    'latin',
    'latin-ext',
    'cyrillic',
    'cyrillic-ext',
    'greek',
    'greek-ext',
    'vietnamese',
    'devanagari'
];

// The five families the reference serves outside its base coverage, in the
// order its own `en-us` font stack puts them.
const FAMILIES = ['hk', 'jp', 'kr', 'sc', 'tc'];

const WEIGHTS = ['400', '700'];

// The Material Icons ligatures jellium-web/src/icon.rs names. A ligature here
// with no `Icon` variant, or a variant with no ligature here, is a failure of
// the_icon_table_and_the_variants_agree in that file.
const ICONS = [
    'access_time', 'add', 'album', 'analytics', 'arrow_back', 'arrow_drop_down',
    'article', 'audiotrack', 'autorenew', 'book', 'cast', 'check', 'check_box',
    'check_box_outline_blank', 'check_circle_outline', 'close', 'closed_caption',
    'dashboard', 'delete', 'devices',
    'dvr', 'edit', 'expand_less', 'expand_more', 'explore', 'extension',
    'fast_forward', 'fast_rewind', 'favorite', 'favorite_border',
    'fiber_manual_record', 'fiber_smart_record', 'filter_alt',
    'folder', 'folder_open', 'fullscreen', 'fullscreen_exit', 'groups', 'home',
    'keyboard', 'keyboard_arrow_down', 'keyboard_arrow_up', 'lan',
    'library_add', 'live_tv', 'lock', 'mode_edit', 'more_vert', 'movie',
    'music_note', 'music_video', 'open_in_new', 'palette',
    'pause', 'people', 'perm_media', 'person', 'phonelink_lock', 'photo',
    'photo_album',
    'play_arrow', 'play_circle', 'play_circle_filled', 'queue', 'quiz',
    'repeat', 'repeat_one', 'replay', 'schedule', 'search', 'settings',
    'shuffle', 'skip_next', 'skip_previous', 'sort_by_alpha', 'stop', 'storage',
    'theaters', 'tv', 'video_library', 'volume_off', 'volume_up', 'vpn_key'
];

const BRANDING = [
    ['jellium-web/branding/avatar.png', 'src/assets/img/avatar.png'],
    ['jellium-web/branding/banner-light.png', 'node_modules/@jellyfin/ux-web/banner-light.png'],
    ['jellium-web/branding/icon-transparent.png', 'node_modules/@jellyfin/ux-web/icon-transparent.png'],
    ['jellium-web/branding/favicons/favicon.ico', 'node_modules/@jellyfin/ux-web/favicons/favicon.ico'],
    ['jellium-web/branding/favicons/touchicon.png', 'node_modules/@jellyfin/ux-web/favicons/touchicon.png'],
    ['jellium-web/branding/favicons/touchicon144.png', 'node_modules/@jellyfin/ux-web/favicons/touchicon144.png']
];

const MATERIAL_TTF = 'node_modules/material-design-icons-iconfont/dist/fonts/MaterialIcons-Regular.ttf';
const MATERIAL_JSON = 'node_modules/material-design-icons-iconfont/dist/fonts/MaterialIcons-Regular.json';

// `U+0460-052F` and `U+20B4` both read as one `[start, end]` pair, so a face's
// coverage is one shape however the stylesheet wrote it.
function range(token) {
    const span = /^U\+([0-9A-Fa-f]{1,6})(?:-([0-9A-Fa-f]{1,6}))?$/.exec(token.trim());
    if (!span) {
        throw new Error(`not a unicode-range token: ${JSON.stringify(token)}`);
    }
    return [span[1].toUpperCase(), (span[2] ?? span[1]).toUpperCase()];
}

// Every `@include font.face(...)` of one stylesheet, as the woff2 its `$url`
// names and the pairs its `$range` declares.
function faces(checkout, path) {
    const text = readFileSync(join(checkout, path), 'utf8');
    const blocks = text.matchAll(/@include font\.face\(([^)]*?\$range: \([^)]*\))\s*\);/gs);
    const held = [];
    for (const [, body] of blocks) {
        const url = /\$url: "~(@fontsource\/[^"]+)"/.exec(body);
        const ranges = /\$range: \(([^)]*)\)/.exec(body);
        if (!url || !ranges) {
            throw new Error(`${path} holds a font.face include with no $url or no $range`);
        }
        held.push({ url: url[1], ranges: ranges[1].split(',').map(range) });
    }
    if (held.length === 0) {
        throw new Error(`${path} declares no font face`);
    }
    return held;
}

const copied = [];

// One file of the checkout, written to `asset` in this tree and recorded.
// `asset` is written from the root of this repository.
function copy(checkout, asset, source) {
    const bytes = readFileSync(join(checkout, source));
    const at = join(root, asset);
    mkdirSync(dirname(at), { recursive: true });
    writeFileSync(at, bytes);
    copied.push({
        asset,
        source,
        sha256: createHash('sha256').update(bytes).digest('hex')
    });
}

// A generated table, recorded against the stylesheets it was read out of
// rather than against a file of the checkout.
function generated(asset, source, text) {
    const at = join(root, asset);
    mkdirSync(dirname(at), { recursive: true });
    writeFileSync(at, text);
    copied.push({
        asset,
        source,
        sha256: createHash('sha256').update(Buffer.from(text, 'utf8')).digest('hex')
    });
}

function row(fields) {
    return `${fields.join('\t')}\n`;
}

function ranges(pairs) {
    return pairs.map(([start, end]) => `${start}-${end}`);
}

// The sixteen base faces: copied into the bundle, and their coverage written
// out so nothing has to re-read a stylesheet to know what they draw.
function embedded(checkout) {
    let table = '';
    for (const weight of WEIGHTS) {
        const declared = faces(checkout, `src/styles/noto-sans/base-${weight}-normal.scss`);
        const named = new Map(
            declared.map((face) => [/noto-sans-(.+)-\d+-normal\.woff2$/.exec(face.url)[1], face])
        );
        for (const subset of SUBSETS) {
            const face = named.get(subset);
            if (!face) {
                throw new Error(`base-${weight}-normal.scss declares no ${subset} face`);
            }
            copy(checkout, `jellium-web/fonts/noto-sans-${subset}-${weight}-normal.woff2`, `node_modules/${face.url}`);
            table += row([subset, weight, ...ranges(face.ranges)]);
        }
        if (named.size !== SUBSETS.length) {
            throw new Error(`base-${weight}-normal.scss declares ${named.size} faces, and the bundle embeds ${SUBSETS.length}`);
        }
    }
    generated('jellium-web/fonts/embedded.tsv', 'src/styles/noto-sans/base-{400,700}-normal.scss', table);
}

// The five CJK families: copied under the path the page serves them at, and
// their coverage written out so a miss knows which one to ask for.
function served(checkout) {
    let table = '';
    for (const family of FAMILIES) {
        for (const weight of WEIGHTS) {
            const stylesheet = `src/styles/noto-sans/${family}-${weight}-normal.scss`;
            const text = readFileSync(join(checkout, stylesheet), 'utf8');
            const declared = /\$\w+Family: "([^"]+)"/.exec(text);
            if (!declared) {
                throw new Error(`${stylesheet} names no family`);
            }
            for (const face of faces(checkout, stylesheet)) {
                const file = face.url.slice(face.url.lastIndexOf('/') + 1);
                copy(checkout, `jellium-web/fonts/served/${file}`, `node_modules/${face.url}`);
                table += row([declared[1], weight, `/fonts/served/${file}`, ...ranges(face.ranges)]);
            }
        }
    }
    generated('jellium-web/fonts/coverage.tsv', 'src/styles/noto-sans/{hk,jp,kr,sc,tc}-{400,700}-normal.scss', table);
}

function icons(checkout) {
    copy(checkout, 'jellium-web/fonts/MaterialIcons-Regular.ttf', MATERIAL_TTF);
    const codepoints = JSON.parse(readFileSync(join(checkout, MATERIAL_JSON), 'utf8'));
    let table = '';
    for (const ligature of ICONS) {
        const glyph = codepoints[ligature];
        if (!glyph) {
            throw new Error(`MaterialIcons-Regular.json names no ${ligature}`);
        }
        table += row([ligature, glyph]);
    }
    generated('jellium-web/icons/material.tsv', MATERIAL_JSON, table);
}

function branding(checkout) {
    for (const [asset, source] of BRANDING) {
        copy(checkout, asset, source);
    }
}

// The five non-default schemes are never loaded beside the dark one, and
// fonts.sized.scss is imported only under `browser.tv && !browser.android`, so
// no viewport this client is drawn at ever resolves either.
const UNRESOLVED = new Set([
    'themes/appletv/theme.scss',
    'themes/blueradiance/theme.scss',
    'themes/light/theme.scss',
    'themes/purplehaze/theme.scss',
    'themes/wmc/theme.scss',
    'styles/fonts.sized.scss'
]);

// The base a media query resolves an em against, which is the initial font
// size rather than whatever the root is set to.
// standard: css-initial-font-size — sixteen css pixels is that size in every
// engine this client runs on, and the reference never writes it
const QUERY_EM = 16;

const WIDTH = 'width';
const HEIGHT = 'height';

// The threshold and the pixel on either side of it, so a `min-` bound is
// straddled by the pair below it and a `max-` bound by the pair above it.
const STRADDLE = [-1, 0, 1];

// How far apart two thresholds stand before the pixels straddling one reach
// the other, which is what keeps a walked pixel from reading as a threshold of
// its own.
const APART = 3;

// The width a height row is resolved at, which no walked width falls on, so the
// row's columns move with its height alone.
const UNTESTED_WIDTH = 1440;

// The layout `layoutManager.tv` draws in, named as the table's `layout` column
// writes it.
const TELEVISION = 'tv';

// The three layouts jellyfin-web draws in, each named as the table's `layout`
// column writes it and paired with the construct whose rule writes its root.
const LAYOUTS = [
    ['mobile', 'type-mobile-root'],
    ['desktop', 'type-root'],
    [TELEVISION, 'type-tv-root']
];

// The construct whose span writes `setCardData`'s width request.
const WIDTH_REQUEST = 'card-width-request';

// The construct whose span writes the query the letter jump is hidden under,
// and the rule that query hides.
const LETTER_JUMP = 'letter-jump';
const ALPHA_PICKER_FIXED = '.alphaPicker-fixed';

// The construct whose span writes the query a dialog fills the page under, and
// the rule that query pins.
const DIALOG_FULLSCREEN = 'dialog-fullscreen';
const DIALOG_FIXED_SIZE = '.dialog-fixedSize';

// The class `layoutManager.tv` puts on an items container, which is what makes
// a rule under it outrank every step of a ladder by specificity.
const TELEVISED = '.itemsContainer-tv';

// The eight shapes a wall row is written for, and the four a rail row is: the
// name the row carries, the selector card.scss sizes it by, and the name
// getPostersPerRow dispatches on. A mixed card's name reaches no case of that
// switch, which is how the reference gives it the default four.
const WALL = [
    ['portrait', '.portraitCard', 'portrait'],
    ['backdrop', '.backdropCard', 'backdrop'],
    ['smallBackdrop', '.smallBackdropCard', 'smallBackdrop'],
    ['square', '.squareCard', 'square'],
    ['banner', '.bannerCard', 'banner'],
    ['mixedPortrait', '.mixedPortraitCard', null],
    ['mixedSquare', '.mixedSquareCard', null],
    ['mixedBackdrop', '.mixedBackdropCard', null]
];

const RAIL = [
    ['portrait', '.overflowPortraitCard', 'overflowPortrait'],
    ['backdrop', '.overflowBackdropCard', 'overflowBackdrop'],
    ['smallBackdrop', '.overflowSmallBackdropCard', 'overflowSmallBackdrop'],
    ['square', '.overflowSquareCard', 'overflowSquare']
];

// The twelve selectors the table walks.
const CARDS = new Set([...WALL, ...RAIL].map(([, selector]) => selector));

const CARD_STYLESHEET = 'src/components/cardbuilder/card.scss';

const LIBRARY_BROWSER = 'src/styles/librarybrowser.scss';

// The dashboard's own overrides, which declare MUI's four default breakpoints
// as scss variables and are the one place under `src` that writes them.
const MUI_BREAKPOINTS = 'src/apps/dashboard/AppOverrides.scss';

// Every width MUI's grid tests, read from the dashboard's own declarations.
function muiBreakpoints(checkout) {
    const text = readFileSync(join(checkout, MUI_BREAKPOINTS), 'utf8');
    const held = [];
    for (const [, written] of text.matchAll(/^\$mui-bp-[a-z]+:([^\n]*)$/gm)) {
        const width = /^\s*([0-9.]+)px\s*;\s*$/.exec(written);
        if (!width) {
            throw new Error(
                `${MUI_BREAKPOINTS} writes a breakpoint this reader cannot read: ${JSON.stringify(written.trim())}`
            );
        }
        held.push(Number(width[1]));
    }
    if (held.length === 0) {
        throw new Error(`${MUI_BREAKPOINTS} declares no MUI breakpoint`);
    }
    return held;
}

// Every stylesheet under the checkout's `src` that a browser drawing this
// client would resolve.
function resolved(checkout) {
    const src = join(checkout, 'src');
    const held = [];
    const met = new Set();
    const walk = (at) => {
        for (const entry of readdirSync(join(src, at), { withFileTypes: true })) {
            const path = at ? `${at}/${entry.name}` : entry.name;
            if (entry.isDirectory()) {
                walk(path);
            } else if (/\.(scss|css)$/.test(entry.name)) {
                if (UNRESOLVED.has(path)) {
                    met.add(path);
                } else {
                    held.push(readFileSync(join(src, path), 'utf8'));
                }
            }
        }
    };
    walk('');
    if (held.length === 0) {
        throw new Error(`${src} holds no stylesheet this client resolves`);
    }
    for (const path of UNRESOLVED) {
        if (!met.has(path)) {
            throw new Error(`${path} is held unresolved and the walk never met it`);
        }
    }
    return held;
}

// Every viewport width and every viewport height the client tests, in css
// pixels, ascending: what the resolved stylesheets bound, what the dashboard's
// MUI overrides declare, and the `requested` widths the switches compare a page
// width against. Refuses two thresholds of one axis standing closer than
// `APART`.
function thresholds(checkout, requested) {
    const held = { [WIDTH]: new Set(), [HEIGHT]: new Set() };
    for (const text of resolved(checkout)) {
        for (const [, , axis, count, unit] of text.matchAll(
            /\((min|max)-(width|height)\s*:\s*([0-9.]+)([a-z%]*)\)/g
        )) {
            if (unit !== 'px' && unit !== 'em') {
                throw new Error(
                    `a media query bounds the ${axis} at ${count}${unit}, which this reader cannot read as css pixels`
                );
            }
            held[axis].add(unit === 'px' ? Number(count) : Number(count) * QUERY_EM);
        }
    }
    for (const at of muiBreakpoints(checkout)) {
        held[WIDTH].add(at);
    }
    for (const at of requested) {
        held[WIDTH].add(at);
    }
    const ascending = (axis) => {
        if (held[axis].size === 0) {
            throw new Error(`no stylesheet this client resolves tests a viewport ${axis}`);
        }
        const sorted = [...held[axis]].sort((left, right) => left - right);
        for (let index = 1; index < sorted.length; index += 1) {
            if (sorted[index] - sorted[index - 1] < APART) {
                throw new Error(
                    `the client tests a ${axis} of ${sorted[index - 1]}px and one of ${sorted[index]}px, which stand closer than the ${APART}px the walk straddles a threshold by`
                );
            }
        }
        return sorted;
    };
    return { [WIDTH]: ascending(WIDTH), [HEIGHT]: ascending(HEIGHT) };
}

// One `@media` condition, as the browser tests it: css makes both bounds
// inclusive.
function condition(text) {
    const bound = /^(min|max)-(width|height):\s*([0-9.]+)(em|px)$/.exec(text);
    if (bound) {
        const px = bound[4] === 'px' ? Number(bound[3]) : Number(bound[3]) * QUERY_EM;
        const axis = bound[2];
        return bound[1] === 'min'
            ? (viewport) => viewport[axis] >= px
            : (viewport) => viewport[axis] <= px;
    }
    const orientation = /^orientation:\s*(landscape|portrait)$/.exec(text);
    if (orientation) {
        return (viewport) => viewport.orientation === orientation[1];
    }
    if (text === 'all' || text === 'screen') {
        return () => true;
    }
    throw new Error(`unread media condition: ${JSON.stringify(text)}`);
}

// One `@media` prelude: a comma is an or, an `and` is an and.
function query(prelude) {
    const alternatives = prelude.split(',').map((alternative) =>
        alternative
            .split(/\s+and\s+/)
            .map((part) => condition(part.trim().replace(/^\(|\)$/g, '')))
    );
    return (viewport) =>
        alternatives.some((parts) => parts.every((holds) => holds(viewport)));
}

// Every block of one css body, braces balanced: the text before its braces and
// the text between them, so a declaration list is never read as a selector
// list.
function blocks(path, body) {
    const held = [];
    let at = 0;
    let opened = 0;
    let depth = 0;
    for (let index = 0; index < body.length; index += 1) {
        const brace = body[index];
        if (brace === '{') {
            if (depth === 0) {
                opened = index;
            }
            depth += 1;
        } else if (brace === '}') {
            depth -= 1;
            if (depth < 0) {
                throw new Error(`${path} closes a block it never opened`);
            }
            if (depth === 0) {
                held.push([body.slice(at, opened), body.slice(opened + 1, index)]);
                at = index + 1;
            }
        }
    }
    if (depth !== 0) {
        throw new Error(`${path} holds an unclosed block`);
    }
    return held;
}

// One rule's own declarations: its body with every block nested in it cut out.
function flattened(body) {
    let held = '';
    let at = 0;
    let depth = 0;
    for (let index = 0; index < body.length; index += 1) {
        const brace = body[index];
        if (brace === '{') {
            if (depth === 0) {
                held += body.slice(at, index);
            }
            depth += 1;
        } else if (brace === '}') {
            depth -= 1;
            if (depth === 0) {
                at = index + 1;
            }
        }
    }
    return held + body.slice(at);
}

// One selector of a width rule: the class it sizes, and whether `TELEVISED`
// qualifies it. `null` for a selector that sizes no card the table walks; a
// selector naming a walked card in any other shape is refused, this reader
// having no ranking to give it.
function sized(path, selector) {
    const at = selector.trim();
    if (CARDS.has(at)) {
        return { card: at, televised: false };
    }
    const child = /^(\.[A-Za-z][\w-]*)\s*>\s*(\.[A-Za-z][\w-]*)$/.exec(at);
    if (child && CARDS.has(child[2])) {
        if (child[1] !== TELEVISED) {
            throw new Error(
                `${path} sizes ${child[2]} under ${child[1]}, which this reader cannot rank`
            );
        }
        return { card: child[2], televised: true };
    }
    for (const [name] of at.matchAll(/\.[A-Za-z][\w-]*/g)) {
        if (CARDS.has(name)) {
            throw new Error(
                `${path} names ${name} in a selector this reader cannot rank: ${JSON.stringify(at)}`
            );
        }
    }
    return null;
}

// The units a width declaration is read in.
const WIDTH_UNITS = new Set(['%', 'vw', 'em']);

// Every width declaration of one stylesheet, in the order the cascade reads
// them: the selectors it sizes, the count and unit it sets, the query that has
// to hold for it to apply, and whether the walk has left it standing.
function widths(checkout, path) {
    const text = readFileSync(join(checkout, path), 'utf8').replace(/\/\*[\s\S]*?\*\//g, '');
    const held = [];
    const nested = (body) => {
        for (const [prelude, inner] of blocks(path, body)) {
            for (const selector of prelude.split(',')) {
                if (sized(path, selector)) {
                    throw new Error(
                        `${path} sizes ${selector.trim()} in a rule nested inside another`
                    );
                }
            }
            nested(inner);
        }
    };
    const read = (body, holds) => {
        for (const [prelude, inner] of blocks(path, body)) {
            const at = prelude.trim();
            const media = /^@media\b([\s\S]*)$/.exec(at);
            if (media) {
                const inside = query(media[1].trim());
                read(inner, (viewport) => holds(viewport) && inside(viewport));
                continue;
            }
            if (at.startsWith('@')) {
                throw new Error(`${path} holds an at-rule this reader cannot read: ${JSON.stringify(at)}`);
            }
            const cards = at
                .split(',')
                .map((selector) => sized(path, selector))
                .filter((one) => one !== null);
            nested(inner);
            const declaration = /(?:^|;)\s*width:\s*([^;{}]+?)\s*(?:!important)?\s*(?:;|$)/.exec(
                flattened(inner)
            );
            if (!declaration) {
                continue;
            }
            const width = /^([0-9.]+)(%|[a-z]+)$/.exec(declaration[1]);
            if (!width || !WIDTH_UNITS.has(width[2])) {
                if (cards.length > 0) {
                    throw new Error(
                        `${path} sizes ${cards.map(({ card }) => card).join(', ')} as ${JSON.stringify(declaration[1])}, which this reader cannot read`
                    );
                }
                continue;
            }
            if (cards.length === 0) {
                continue;
            }
            held.push({ sized: cards, count: width[1], unit: width[2], holds, read: false });
        }
    };
    read(text, () => true);
    return held;
}

// The width the cascade leaves standing for one card, and marks that rule
// read: the `TELEVISED` rule where the viewport is drawn in the television
// layout, whatever the source order, and the last step whose query the
// viewport answers otherwise.
function standing(ladder, selector, viewport) {
    let stepped;
    let televised;
    for (const rule of ladder) {
        if (!rule.holds(viewport)) {
            continue;
        }
        for (const card of rule.sized) {
            if (card.card !== selector) {
                continue;
            }
            if (card.televised) {
                televised = rule;
            } else {
                stepped = rule;
            }
        }
    }
    const held = viewport.layout === TELEVISION && televised ? televised : stepped;
    if (!held) {
        throw new Error(`no rule of the stylesheet sizes ${selector} at ${viewport.width}px`);
    }
    held.read = true;
    return held;
}

// Every rule of `ladder` that sizes a card the table walks and that no viewport
// of the walk left standing.
function unread(ladder) {
    return ladder
        .filter((rule) => !rule.read)
        .map((rule) =>
            rule.sized
                .map(({ card, televised }) => (televised ? `${TELEVISED} > ${card}` : card))
                .join(', ')
        );
}

// The decimals a written percentage carries, which is what decides whether it
// names a count.
function digits(count) {
    const dot = count.indexOf('.');
    return dot < 0 ? 0 : count.length - dot - 1;
}

// `100 / cards` rendered to `scale` decimals, divided exactly rather than in a
// double, because the stylesheet writes its shares to thirty digits and a
// double carries seventeen.
function reciprocal(cards, scale) {
    const over = 100n * 10n ** BigInt(scale);
    const whole = (2n * over + BigInt(cards)) / (2n * BigInt(cards));
    const written = whole.toString().padStart(scale + 1, '0');
    return scale === 0
        ? written
        : `${written.slice(0, written.length - scale)}.${written.slice(written.length - scale)}`;
}

// Whether the reference's own written percentage is a count's share: it names
// `cards` exactly when `100 / cards`, rendered to the digits the reference
// wrote, is what the reference wrote.
function names(count, cards) {
    return cards >= 1 && reciprocal(cards, digits(count)) === count;
}

// How many whole card pitches a viewport holds: the count the reference's own
// digits name, and the floor of the share otherwise.
function across(count) {
    const share = Number(count);
    const whole = Math.round(100 / share);
    if (names(count, whole)) {
        return whole;
    }
    return Math.max(1, Math.floor(100 / share));
}

// A percentage as the oracle writes it: six decimal places, trailing zeros cut.
function written(percent) {
    return percent.toFixed(6).replace(/0+$/, '').replace(/\.$/, '');
}

const CARD_BUILDER_UTILS = 'src/components/cardbuilder/cardBuilderUtils.ts';

// The decimals an arm of the cards-per-row ladder is written to, which is what
// keeps two arms of the same value written to different digits apart.
const ARM_DIGITS = 11;

// The body of one arrow function of cardBuilderUtils.ts, braces balanced.
function declared(text, name) {
    const at = text.indexOf(`const ${name} = (`);
    if (at < 0) {
        throw new Error(`${CARD_BUILDER_UTILS} declares no ${name}`);
    }
    const arrow = text.indexOf('=>', at) + '=>'.length;
    const gap = /^\s*\{/.exec(text.slice(arrow));
    if (!gap) {
        throw new Error(`${CARD_BUILDER_UTILS} does not open ${name}'s body at its arrow`);
    }
    const opened = arrow + gap[0].length - 1;
    let depth = 1;
    let end = opened + 1;
    while (depth > 0) {
        const brace = text[end];
        if (brace === '{') {
            depth += 1;
        } else if (brace === '}') {
            depth -= 1;
        } else if (brace === undefined) {
            throw new Error(`${CARD_BUILDER_UTILS} leaves ${name} unclosed`);
        }
        end += 1;
    }
    return text.slice(opened + 1, end - 1);
}

// The condition the reference writes for a television, which it tests before
// every other arm of the switch it stands in.
const TELEVISED_ARM = 'isTV';

// One arm's condition, as a test of the viewport, `wider` being the factor
// `setCardData` writes; every count a `screenWidth >= <count>` compares against
// is added to `tested`.
function reached(condition, wider, tested) {
    const tests = [];
    for (const part of condition.split('&&').map((one) => one.trim())) {
        if (part === TELEVISED_ARM) {
            throw new Error(
                `a cards-per-row arm tests a television beside another test: ${JSON.stringify(condition.trim())}`
            );
        }
        if (part === 'isLandscape') {
            tests.push((viewport) => viewport.width > viewport.height * wider);
            continue;
        }
        const wide = /^screenWidth >= ([0-9]+)$/.exec(part);
        if (!wide) {
            throw new Error(`unread cards-per-row condition: ${JSON.stringify(part)}`);
        }
        const at = Number(wide[1]);
        tested.add(at);
        tests.push((viewport) => viewport.width >= at);
    }
    return (viewport) => tests.every((holds) => holds(viewport));
}

// One arm's value, which the reference writes either as a count or as
// `100 / percent`, and which stays unrounded either way.
function rate(expression) {
    const over = /^100 \/ ([0-9.]+)$/.exec(expression);
    if (over) {
        return 100 / Number(over[1]);
    }
    if (!/^[0-9]+$/.test(expression)) {
        throw new Error(`unread cards-per-row value: ${JSON.stringify(expression)}`);
    }
    return Number(expression);
}

// Whether a line opens an arm of a switch, which is what makes an unread one a
// refusal rather than a line this reader passes over.
const ARM_LINE = /^\s*(?:case\b|default\s*:)/;

// Every arm of one `switch (true)`: the television arm the reference tests
// before all of them, and the steps under it in the source order; every page
// width they compare against is added to `tested`.
function arms(source, wider, tested) {
    const steps = [];
    let televised;
    for (const line of source.split('\n')) {
        if (!ARM_LINE.test(line)) {
            continue;
        }
        const arm = /^\s*(?:case (.+?)|(default)):\s*return (.+?);\s*$/.exec(line);
        if (!arm) {
            throw new Error(`unread cards-per-row arm: ${JSON.stringify(line.trim())}`);
        }
        const held = rate(arm[3].trim());
        if (arm[2] !== undefined) {
            steps.push({ holds: () => true, rate: held, answered: false });
            continue;
        }
        if (arm[1].trim() === TELEVISED_ARM) {
            if (televised) {
                throw new Error(`${CARD_BUILDER_UTILS} holds a switch with two television arms`);
            }
            televised = { rate: held, answered: false };
            continue;
        }
        steps.push({ holds: reached(arm[1], wider, tested), rate: held, answered: false });
    }
    if (steps.length === 0) {
        throw new Error(`${CARD_BUILDER_UTILS} holds a switch with no step`);
    }
    return { televised, steps };
}

// `getPostersPerRow`, read out of the reference: each shape's own ladder, the
// `default` the switch gives a shape it holds no case for, and every page width
// any arm of any ladder compares against.
function requesting(checkout, wider) {
    const text = readFileSync(join(checkout, CARD_BUILDER_UTILS), 'utf8');
    const dispatch = declared(text, 'getPostersPerRow');
    const shapes = new Map();
    const tested = new Set();
    let otherwise;
    for (const line of dispatch.split('\n')) {
        if (!ARM_LINE.test(line)) {
            continue;
        }
        const named = /^\s*case '([^']+)':\s*return (\w+)\([^)]*\);\s*$/.exec(line);
        if (named) {
            shapes.set(named[1], arms(declared(text, named[2]), wider, tested));
            continue;
        }
        const fallback = /^\s*default:\s*return ([0-9]+);\s*$/.exec(line);
        if (!fallback) {
            throw new Error(`unread cards-per-row case: ${JSON.stringify(line.trim())}`);
        }
        otherwise = Number(fallback[1]);
    }
    if (shapes.size === 0 || otherwise === undefined) {
        throw new Error(`${CARD_BUILDER_UTILS} holds no cards-per-row switch`);
    }
    if (tested.size === 0) {
        throw new Error(`${CARD_BUILDER_UTILS} holds no cards-per-row arm testing a page width`);
    }
    return { shapes, otherwise, tested };
}

// The arm one shape's ladder answers for a viewport, and marks it answered: the
// television arm where the viewport is drawn in the television layout and the
// ladder holds one, and the first step whose condition the viewport answers
// otherwise. `null` asks for the switch's own `default`, which is what a mixed
// card's name reaches.
function asked(dispatch, shape, viewport) {
    if (shape === null) {
        return dispatch.otherwise;
    }
    const ladder = dispatch.shapes.get(shape);
    if (!ladder) {
        throw new Error(`${CARD_BUILDER_UTILS} holds no case for ${shape}`);
    }
    if (viewport.layout === TELEVISION && ladder.televised) {
        ladder.televised.answered = true;
        return ladder.televised.rate;
    }
    for (const arm of ladder.steps) {
        if (arm.holds(viewport)) {
            arm.answered = true;
            return arm.rate;
        }
    }
    throw new Error(`${shape} answers no arm at ${viewport.width}px`);
}

// Every arm of a walked shape's ladder that no request of the walk answered.
function unanswered(dispatch) {
    const held = [];
    for (const [shape, ladder] of dispatch.shapes) {
        if (ladder.televised && !ladder.televised.answered) {
            held.push(`${shape}: ${TELEVISED_ARM}`);
        }
        for (const [at, arm] of ladder.steps.entries()) {
            if (!arm.answered) {
                held.push(`${shape}: step ${at}`);
            }
        }
    }
    return held;
}

// The rule whose whole selector is this, which is where the page's own side
// share is written.
const PADDED = '.padded-left';

// `.padded-left`: the share of the page a card wall's own container keeps clear
// on each side, read inside that rule's own braces.
function padding(checkout) {
    const text = readFileSync(join(checkout, LIBRARY_BROWSER), 'utf8').replace(
        /\/\*[\s\S]*?\*\//g,
        ''
    );
    for (const [prelude, body] of blocks(LIBRARY_BROWSER, text)) {
        if (prelude.trim() !== PADDED) {
            continue;
        }
        const share = /conditional-max\(padding-left,\s*([0-9.]+)%/.exec(body);
        if (!share) {
            throw new Error(
                `${LIBRARY_BROWSER}'s ${PADDED} rule holds no conditional-max(padding-left, ..%)`
            );
        }
        return Number(share[1]);
    }
    throw new Error(`${LIBRARY_BROWSER} holds no rule whose whole selector is ${PADDED}`);
}

// The root size one construct's rule sets, as the percentage of the 16px base
// the reference writes it in. Refuses a span writing no font size in percent,
// and one writing more than one.
function rootPercent(read, construct) {
    const written = [
        ...span(read, construct).matchAll(/(?:font-size|\$size)\s*:\s*([0-9.]+)%/g)
    ];
    if (written.length !== 1) {
        throw new Error(
            `${construct} writes ${written.length} font sizes in percent, and a root rule writes one`
        );
    }
    return Number(written[0][1]);
}

// The query one construct's span writes over `selector`, as a test of a
// viewport. Refuses unless exactly one `@media` block of that span holds a rule
// naming `selector`.
function queried(read, construct, selector) {
    const text = span(read, construct);
    const held = [];
    for (const [prelude, body] of blocks(construct, text)) {
        const media = /^@media\b([\s\S]*)$/.exec(prelude.trim());
        if (!media) {
            continue;
        }
        const names = blocks(construct, body).some(([inner]) =>
            inner.split(',').some((one) => one.trim() === selector)
        );
        if (names) {
            held.push(query(media[1].trim()));
        }
    }
    if (held.length !== 1) {
        throw new Error(
            `${construct} holds ${held.length} media blocks naming ${selector}, and this reader reads one`
        );
    }
    return held[0];
}

// How much wider than tall `setCardData` asks a page to be before it calls it
// landscape, read from the call it writes. Refuses a span that does not write
// `getImageWidth(.., screenWidth > (screenHeight * <count>))`.
function landscape(read) {
    const written =
        /getImageWidth\([^;]*screenWidth > \(screenHeight \* ([0-9.]+)\)\)/.exec(
            span(read, WIDTH_REQUEST)
        );
    if (!written) {
        throw new Error(
            `${WIDTH_REQUEST} writes no getImageWidth call this reader can read a landscape factor out of`
        );
    }
    const wider = Number(written[1]);
    if (!(wider > 0)) {
        throw new Error(
            `${WIDTH_REQUEST} writes a landscape factor of ${written[1]}, and a page is landscape above a positive multiple of its height`
        );
    }
    return wider;
}

// Everything the walk reads out of the checkout, read once: the width ladder,
// the page's own side share, the request ladder, the landscape factor that
// ladder is read with, every threshold the client tests, the two queries the
// row's last columns answer, and each layout beside the root percentage its own
// rule writes.
function reading(checkout) {
    const read = texts(checkout);
    const wider = landscape(read);
    const dispatch = requesting(checkout, wider);
    return {
        ladder: widths(checkout, CARD_STYLESHEET),
        side: padding(checkout),
        dispatch,
        wider,
        tested: thresholds(checkout, dispatch.tested),
        letters: queried(read, LETTER_JUMP, ALPHA_PICKER_FIXED),
        dialog: queried(read, DIALOG_FULLSCREEN, DIALOG_FIXED_SIZE),
        layouts: LAYOUTS.map(([layout, construct]) => [layout, rootPercent(read, construct)])
    };
}

// One viewport the table walks: the size the page reports, the orientation that
// size implies, the layout the browser showing it is drawn in, and the root
// percentage that layout writes over the 16px base.
// standard: css-media-queries — a viewport is landscape where its width is at
// least its height, which the reference never writes and this reader resolves
// its own `orientation:` queries by
function walked(width, height, layout, root) {
    return {
        width,
        height,
        orientation: width >= height ? 'landscape' : 'portrait',
        layout,
        root
    };
}

// One row: the whole viewport it was resolved at, and what the reference draws
// there.
function measured(kind, shapes, read, viewport) {
    const { width, height, orientation, layout, root } = viewport;
    let rows = '';
    for (const [shape, selector, dispatched] of shapes) {
        const held = standing(read.ladder, selector, viewport);
        const box = width * (1 - (2 * read.side) / 100);
        const percent =
            held.unit === 'em'
                ? (100 * Number(held.count) * ((root / 100) * QUERY_EM)) / box
                : Number(held.count);
        const cards =
            held.unit === '%' ? across(held.count) : Math.max(1, Math.floor(100 / percent));
        if (kind === WIDTH && held.unit === '%' && !names(held.count, cards)) {
            throw new Error(
                `${selector} is ${held.count}% at ${width}px, which is no ${cards}th of a viewport`
            );
        }
        const rate = asked(read.dispatch, dispatched, viewport);
        rows += row([
            kind,
            width,
            height,
            shape,
            orientation,
            written(percent),
            cards,
            rate.toFixed(ARM_DIGITS),
            Math.round(width / rate),
            layout,
            root,
            read.letters(viewport) ? 'hidden' : 'shown',
            read.dialog(viewport) ? 'fullscreen' : 'fixed'
        ]);
    }
    return rows;
}

// The heights one width is walked at, ascending and distinct: the tallest page
// `getImageWidth` still calls landscape and the shortest it calls portrait, and
// the tallest css calls landscape and the shortest it calls portrait. Refuses a
// width whose landscape height is not a positive count of css pixels.
function heights(width, wider) {
    const tallest = Math.ceil(width / wider) - 1;
    if (!Number.isInteger(tallest) || tallest < 1) {
        throw new Error(
            `a page ${width}px wide is landscape up to a height of ${tallest}, which is no positive count of css pixels`
        );
    }
    return [...new Set([tallest, tallest + 1, width, width + 1])].sort(
        (left, right) => left - right
    );
}

function breakpoints(checkout) {
    const read = reading(checkout);
    const { tested } = read;
    // `kind`, `shape` and `layout` name the walk's own vocabulary rather than
    // any measurement, and `orientation` is the css rule above resolved on
    // both sides; every other column is read out of the reference.
    let table = row([
        'kind',
        'width',
        'height',
        'shape',
        'orientation',
        'percent',
        'across',
        'requested',
        'fill',
        'layout',
        'root',
        'letter_jump',
        'dialog'
    ]);
    if (tested[WIDTH].some((threshold) => STRADDLE.some((step) => threshold + step === UNTESTED_WIDTH))) {
        throw new Error(
            `a height row is resolved at ${UNTESTED_WIDTH}px, which the walk tests as a width`
        );
    }
    for (const [kind, shapes] of [
        [WIDTH, WALL],
        ['rail', RAIL]
    ]) {
        for (const threshold of tested[WIDTH]) {
            for (const step of STRADDLE) {
                const width = threshold + step;
                for (const height of heights(width, read.wider)) {
                    for (const [layout, root] of read.layouts) {
                        table += measured(kind, shapes, read, walked(width, height, layout, root));
                    }
                }
            }
        }
    }
    for (const threshold of tested[HEIGHT]) {
        for (const step of STRADDLE) {
            for (const [layout, root] of read.layouts) {
                table += measured(
                    HEIGHT,
                    WALL,
                    read,
                    walked(UNTESTED_WIDTH, threshold + step, layout, root)
                );
            }
        }
    }
    const missed = unread(read.ladder);
    if (missed.length > 0) {
        throw new Error(
            `${CARD_STYLESHEET} sizes a walked card in ${missed.length} rules the walk never left standing: ${missed.join('; ')}`
        );
    }
    const dropped = unanswered(read.dispatch);
    if (dropped.length > 0) {
        throw new Error(
            `${CARD_BUILDER_UTILS} holds ${dropped.length} arms the walk never answered: ${dropped.join('; ')}`
        );
    }
    generated('reference/breakpoints.tsv', 'src', table);
}

const checkout = process.argv[2];
if (!checkout) {
    throw new Error('usage: node tools/reference/assets.mjs <jellyfin-web-checkout>');
}

const revision = pinned();
checkedOut(checkout, revision.commit);

// This is the one tool that runs against an installed node_modules, so it is
// where the second row of reference/PINNED is enforced.
const release = apiclient();
locked(checkout, release.version, release.integrity);

for (const held of ['fonts', 'icons', 'branding']) {
    rmSync(join(web, held), { recursive: true, force: true });
}

embedded(checkout);
served(checkout);
icons(checkout);
branding(checkout);
breakpoints(checkout);

copied.sort((left, right) => (left.asset < right.asset ? -1 : left.asset > right.asset ? 1 : 0));
let register = row(['asset', 'source', 'sha256']);
for (const { asset, source, sha256 } of copied) {
    register += row([asset, source, sha256]);
}
writeFileSync(join(root, 'reference', 'assets.tsv'), register);

// Nothing under the three rewritten directories may survive that no row names.
for (const held of ['fonts', 'icons', 'branding']) {
    const named = new Set(copied.map(({ asset }) => asset));
    const walk = (at) => {
        for (const entry of readdirSync(join(web, at), { withFileTypes: true })) {
            const path = `${at}/${entry.name}`;
            if (entry.isDirectory()) {
                walk(path);
            } else if (!named.has(`jellium-web/${path}`)) {
                throw new Error(`jellium-web/${path} carries no row`);
            }
        }
    };
    walk(held);
}
