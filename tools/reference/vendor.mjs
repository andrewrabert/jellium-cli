#!/usr/bin/env node

// Rewrites reference/vendor.tsv from a checkout of the revision
// reference/PINNED's third row names. One row per file of that revision: the
// file's path inside it, what jellium-web/vendor/blurhash does with it, and the
// digest of the upstream file.
//
//     node tools/reference/vendor.mjs <blurhash-rs-checkout>

import { createHash } from 'node:crypto';
import { readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { checkedOut, vendored } from './pinned.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');

// The directories of the upstream revision that carry no file this tree could
// vendor: its own history and the workflows that build it upstream.
const UNREAD = new Set(['.git', '.github']);

// Every file of the checkout, in path order.
function files(checkout) {
    const held = [];
    const walk = (at) => {
        for (const name of readdirSync(at).sort()) {
            if (UNREAD.has(name)) {
                continue;
            }
            const path = join(at, name);
            if (statSync(path).isDirectory()) {
                walk(path);
            } else {
                held.push(relative(checkout, path).split('\\').join('/'));
            }
        }
    };
    walk(checkout);
    return held;
}

// `verbatim` where the vendored file's bytes are the upstream file's,
// `retargeted` where a vendored file exists and differs, `dropped` where the
// vendored tree carries no such file. The standing is derived, so editing a
// vendored file rewrites its row and `git diff --exit-code` fails.
export function standing(checkout, path) {
    let held;
    try {
        held = readFileSync(join(root, 'jellium-web', 'vendor', 'blurhash', path));
    } catch {
        return 'dropped';
    }
    return held.equals(readFileSync(join(checkout, path))) ? 'verbatim' : 'retargeted';
}

const checkout = process.argv[2];
if (!checkout) {
    throw new Error('usage: node tools/reference/vendor.mjs <blurhash-rs-checkout>');
}

const revision = vendored();
checkedOut(checkout, revision.commit);

const lines = ['source\tstanding\tsha256'];
for (const path of files(checkout)) {
    const digest = createHash('sha256').update(readFileSync(join(checkout, path))).digest('hex');
    lines.push(`${path}\t${standing(checkout, path)}\t${digest}`);
}
writeFileSync(join(root, 'reference', 'vendor.tsv'), `${lines.join('\n')}\n`);
