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
    'arrow_back', 'audiotrack', 'autorenew', 'book', 'cast', 'check',
    'closed_caption', 'explore', 'fast_forward', 'fast_rewind', 'favorite',
    'favorite_border', 'filter_alt', 'folder', 'fullscreen', 'fullscreen_exit',
    'groups', 'live_tv', 'movie', 'music_note', 'music_video', 'pause',
    'person', 'photo', 'play_arrow', 'queue', 'quiz', 'repeat', 'repeat_one',
    'replay', 'search', 'settings', 'shuffle', 'skip_next', 'skip_previous',
    'sort_by_alpha', 'stop', 'storage', 'theaters', 'tv', 'video_library',
    'volume_off', 'volume_up'
];

const BRANDING = [
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
const QUERY_EM = 16;

const WIDTH = 'width';
const HEIGHT = 'height';

const LIBRARY_BROWSER = 'src/styles/librarybrowser.scss';

// Every stylesheet under the checkout's `src` that a browser drawing this
// client would resolve.
function resolved(checkout) {
    const src = join(checkout, 'src');
    const held = [];
    const walk = (at) => {
        for (const entry of readdirSync(join(src, at), { withFileTypes: true })) {
            const path = at ? `${at}/${entry.name}` : entry.name;
            if (entry.isDirectory()) {
                walk(path);
            } else if (/\.(scss|css)$/.test(entry.name) && !UNRESOLVED.has(path)) {
                held.push(readFileSync(join(src, path), 'utf8'));
            }
        }
    };
    walk('');
    return held;
}

// Every viewport width and every viewport height the resolved stylesheets test,
// in css pixels, ascending.
function thresholds(checkout) {
    const held = { [WIDTH]: new Set(), [HEIGHT]: new Set() };
    for (const text of resolved(checkout)) {
        for (const [, , axis, count, unit] of text.matchAll(
            /\((min|max)-(width|height)\s*:\s*([0-9.]+)(em|px)\)/g
        )) {
            held[axis].add(unit === 'px' ? Number(count) : Number(count) * QUERY_EM);
        }
    }
    const ascending = (axis) => [...held[axis]].sort((left, right) => left - right);
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

// The width declarations of one stylesheet, in the order the cascade reads
// them, each with the query that has to hold for it to apply.
function widths(checkout, path) {
    const text = readFileSync(join(checkout, path), 'utf8').replace(/\/\*[\s\S]*?\*\//g, '');
    const held = [];
    const rules = (body, holds) => {
        const pattern = /([^{}]+)\{([^{}]*)\}/g;
        for (const [, selectors, declarations] of body.matchAll(pattern)) {
            const width = /(?:^|;)\s*width:\s*([0-9.]+)(%|vw|em)\s*(?:!important)?\s*(?:;|$)/.exec(
                declarations
            );
            if (!width) {
                continue;
            }
            held.push({
                selectors: selectors.split(',').map((selector) => selector.trim()),
                count: width[1],
                unit: width[2],
                holds
            });
        }
    };
    let at = 0;
    while (at < text.length) {
        const block = /@media([^{]*)\{/g;
        block.lastIndex = at;
        const found = block.exec(text);
        if (!found) {
            rules(text.slice(at), () => true);
            break;
        }
        rules(text.slice(at, found.index), () => true);
        let depth = 1;
        let end = block.lastIndex;
        while (depth > 0) {
            const brace = text[end];
            if (brace === '{') {
                depth += 1;
            } else if (brace === '}') {
                depth -= 1;
            } else if (brace === undefined) {
                throw new Error(`${path} holds an unclosed @media block`);
            }
            end += 1;
        }
        rules(text.slice(block.lastIndex, end - 1), query(found[1].trim()));
        at = end;
    }
    return held;
}

// The width the cascade leaves standing for one selector, as the reference
// wrote it.
function standing(ladder, selector, viewport) {
    let held;
    for (const rule of ladder) {
        if (rule.selectors.includes(selector) && rule.holds(viewport)) {
            held = rule;
        }
    }
    if (!held) {
        throw new Error(`card.scss leaves ${selector} no width`);
    }
    return held;
}

// The eight shapes a wall row is written for, and the four a rail row is: the
// name the row carries, the selector card.scss sizes it by, and the name
// getPostersPerRow dispatches on. A mixed card's name reaches no arm of that
// switch, which is how the reference gives it the default four.
const WALL = [
    ['portrait', '.portraitCard', 'portrait'],
    ['backdrop', '.backdropCard', 'backdrop'],
    ['smallBackdrop', '.smallBackdropCard', 'smallBackdrop'],
    ['square', '.squareCard', 'square'],
    ['banner', '.bannerCard', 'banner'],
    ['mixedPortrait', '.mixedPortraitCard', 'mixedPortrait'],
    ['mixedSquare', '.mixedSquareCard', 'mixedSquare'],
    ['mixedBackdrop', '.mixedBackdropCard', 'mixedBackdrop']
];

const RAIL = [
    ['portrait', '.overflowPortraitCard', 'overflowPortrait'],
    ['backdrop', '.overflowBackdropCard', 'overflowBackdrop'],
    ['smallBackdrop', '.overflowSmallBackdropCard', 'overflowSmallBackdrop'],
    ['square', '.overflowSquareCard', 'overflowSquare']
];

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
    const opened = text.indexOf('{', text.indexOf('=>', at));
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

// One arm's condition, as a test of the request. A television arm answers
// nothing, this client being drawn on no television.
function reached(condition) {
    const tests = [];
    for (const part of condition.split('&&').map((one) => one.trim())) {
        if (part === 'isTV') {
            return null;
        }
        if (part === 'isLandscape') {
            tests.push((request) => request.landscape);
            continue;
        }
        const wide = /^screenWidth >= ([0-9]+)$/.exec(part);
        if (!wide) {
            throw new Error(`unread cards-per-row condition: ${JSON.stringify(part)}`);
        }
        const at = Number(wide[1]);
        tests.push((request) => request.width >= at);
    }
    return (request) => tests.every((holds) => holds(request));
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

// Every arm of one `switch (true)`, in the source order the reference tests
// them.
function arms(source) {
    const held = [];
    for (const line of source.split('\n')) {
        const arm = /^\s*(?:case (.+?)|(default)):\s*return (.+?);\s*$/.exec(line);
        if (!arm) {
            continue;
        }
        const holds = arm[2] === undefined ? reached(arm[1]) : () => true;
        if (!holds) {
            continue;
        }
        held.push({ holds, rate: rate(arm[3].trim()) });
    }
    if (held.length === 0) {
        throw new Error(`${CARD_BUILDER_UTILS} holds a switch with no arm this client reaches`);
    }
    return held;
}

// `getPostersPerRow`, read out of the reference rather than transcribed: the
// arm each shape's own ladder answers, and the default a shape the switch does
// not name is given.
function requesting(checkout) {
    const text = readFileSync(join(checkout, CARD_BUILDER_UTILS), 'utf8');
    const dispatch = declared(text, 'getPostersPerRow');
    const named = new Map();
    for (const [, shape, helper] of dispatch.matchAll(/case '([^']+)':\s*return (\w+)\(/g)) {
        named.set(shape, arms(declared(text, helper)));
    }
    const otherwise = /default:\s*return ([0-9]+);/.exec(dispatch);
    if (named.size === 0 || !otherwise) {
        throw new Error(`${CARD_BUILDER_UTILS} holds no cards-per-row switch`);
    }
    return (shape, request) => {
        const ladder = named.get(shape);
        if (!ladder) {
            return Number(otherwise[1]);
        }
        for (const arm of ladder) {
            if (arm.holds(request)) {
                return arm.rate;
            }
        }
        throw new Error(`${shape} answers no arm at ${request.width}px`);
    };
}

// `.padded-left` and `.padded-right`: the share of the page a card wall's own
// container keeps clear on each side.
function padding(checkout) {
    const text = readFileSync(join(checkout, LIBRARY_BROWSER), 'utf8');
    const share = /\.padded-left\s*\{[\s\S]*?conditional-max\(padding-left,\s*([0-9.]+)%/.exec(
        text
    );
    if (!share) {
        throw new Error(`${LIBRARY_BROWSER} holds no .padded-left share`);
    }
    return Number(share[1]);
}

// One row: the whole viewport it was resolved at, and what the reference draws
// there.
function measured(kind, ladder, requested, shapes, side, width, height) {
    const orientation = width >= height ? 'landscape' : 'portrait';
    const band = width <= 600 ? 'mobile' : 'desktop';
    const root = band === 'mobile' ? 900 : 930;
    const viewport = { width, height, orientation };
    // getImageWidth's own landscape test, which is not the css orientation
    // above: it asks whether the page is half again as wide as it is tall.
    const request = { width, landscape: width > height * 1.3 };
    let rows = '';
    for (const [shape, selector, asked] of shapes) {
        const held = standing(ladder, selector, viewport);
        const box = width * (1 - (2 * side) / 100);
        const percent =
            held.unit === 'em'
                ? (100 * Number(held.count) * ((root / 1000) * QUERY_EM)) / box
                : Number(held.count);
        const cards =
            held.unit === '%' ? across(held.count) : Math.max(1, Math.floor(100 / percent));
        if (kind === WIDTH && held.unit === '%' && !names(held.count, cards)) {
            throw new Error(
                `${selector} is ${held.count}% at ${width}px, which is no ${cards}th of a viewport`
            );
        }
        const rate = requested(asked, request);
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
            band,
            root,
            height <= 500 ? 'hidden' : 'shown',
            width <= 1280 || height <= 720 ? 'fullscreen' : 'fixed'
        ]);
    }
    return rows;
}

// The width a height row is resolved at, which no rule tests, so the row's
// columns move with its height alone.
const UNTESTED_WIDTH = 1440;

function breakpoints(checkout) {
    const ladder = widths(checkout, 'src/components/cardbuilder/card.scss');
    const side = padding(checkout);
    const requested = requesting(checkout);
    const tested = thresholds(checkout);
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
        'band',
        'root',
        'letter_jump',
        'dialog'
    ]);
    for (const threshold of tested[WIDTH]) {
        for (const width of [threshold - 1, threshold]) {
            for (const height of [width * 2, Math.max(200, width / 2)]) {
                table += measured(WIDTH, ladder, requested, WALL, side, width, height);
            }
        }
    }
    for (const threshold of tested[WIDTH]) {
        for (const width of [threshold - 1, threshold]) {
            for (const height of [width * 2, Math.max(200, width / 2)]) {
                table += measured('rail', ladder, requested, RAIL, side, width, height);
            }
        }
    }
    for (const threshold of tested[HEIGHT]) {
        for (const height of [threshold - 1, threshold]) {
            table += measured(HEIGHT, ladder, requested, WALL, side, UNTESTED_WIDTH, height);
        }
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
