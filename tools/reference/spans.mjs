#!/usr/bin/env node

// Rewrites reference/spans out of a checkout of the revision reference/PINNED
// names. One file per row of reference/provenance.tsv, however many rows it
// holds, named for the row's construct and holding the row's lines verbatim.
//
//     node tools/reference/spans.mjs <jellyfin-web-checkout>

import { mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { apiclient, checkedOut, locked, pinned } from './pinned.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const written = join(root, 'reference', 'spans');

const HEADER = 'construct\tpath\tfirst\tlast\tsha256\tkind';

// The root a path names when it is not read from the jellyfin-web checkout.
const BUNDLED = 'jellyfin-apiclient:';

// The entry of the bundle's `sourcesContent` a `BUNDLED` path names.
const SOURCE = 'src/apiClient.js';

function rows() {
    const lines = readFileSync(join(root, 'reference', 'provenance.tsv'), 'utf8').split('\n');
    if (lines[0] !== HEADER) {
        throw new Error('reference/provenance.tsv does not open with its header');
    }
    return lines.slice(1).filter((line) => line !== '').map((line, offset) => {
        const fields = line.split('\t');
        if (fields.length !== 6) {
            throw new Error(`reference/provenance.tsv:${offset + 2} holds ${fields.length} fields, and a row holds six`);
        }
        const [construct, path, first, last] = fields;
        return { construct, path, first: Number(first), last: Number(last) };
    });
}

function bundled(checkout) {
    const map = join(checkout, 'node_modules', 'jellyfin-apiclient', 'dist', 'jellyfin-apiclient.js.map');
    const document = JSON.parse(readFileSync(map, 'utf8'));
    const at = document.sources.findIndex((source) => source.endsWith(SOURCE));
    if (at < 0) {
        throw new Error(`${map} carries no ${SOURCE}`);
    }
    return document.sourcesContent[at];
}

function texts(checkout) {
    const read = new Map();
    return (path) => {
        if (!read.has(path)) {
            read.set(path, path.startsWith(BUNDLED) ? bundled(checkout) : readFileSync(join(checkout, path), 'utf8'));
        }
        return read.get(path);
    };
}

const checkout = process.argv[2];
if (!checkout) {
    throw new Error('usage: node tools/reference/spans.mjs <jellyfin-web-checkout>');
}

const revision = pinned();
checkedOut(checkout, revision.commit);
const release = apiclient();
locked(checkout, release.version, release.integrity);

const held = rows();
const source = texts(checkout);
mkdirSync(written, { recursive: true });

const named = new Set();
for (const row of held) {
    const lines = source(row.path).split('\n');
    if (row.last > lines.length) {
        throw new Error(`${row.path} is shorter than line ${row.last}`);
    }
    writeFileSync(join(written, `${row.construct}.txt`), lines.slice(row.first - 1, row.last).join('\n'));
    named.add(`${row.construct}.txt`);
}

for (const entry of readdirSync(written)) {
    if (entry.endsWith('.txt') && !named.has(entry)) {
        rmSync(join(written, entry));
    }
}
