#!/usr/bin/env node

// Rewrites reference/spans out of a checkout of the revision reference/PINNED
// names. One file per row of reference/provenance.tsv, however many rows it
// holds, named for the row's construct and holding the row's lines verbatim.
//
//     node tools/reference/spans.mjs <jellyfin-web-checkout>

import { mkdirSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { apiclient, checkedOut, locked, pinned } from './pinned.mjs';
import { rows, texts } from './provenance.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');
const written = join(root, 'reference', 'spans');

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
