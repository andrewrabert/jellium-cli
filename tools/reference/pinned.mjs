#!/usr/bin/env node

// The one reader of reference/PINNED. Its first row names the jellyfin-web
// revision every slice and every copied asset is taken from; its second names
// the jellyfin-apiclient release the reference itself depends on; its third
// names the blurhash-rs revision jellium-web/vendor/blurhash was taken from.

import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');

function rows() {
    return readFileSync(join(root, 'reference', 'PINNED'), 'utf8')
        .split('\n')
        .map((line) => line.trimEnd());
}

export function pinned() {
    const line = rows()[0] ?? '';
    const [tag, commit] = line.split('\t');
    if (!tag || !/^[0-9a-f]{40}$/.test(commit ?? '')) {
        throw new Error(`reference/PINNED row 1 is not '<tag>\\t<40-hex-commit>': ${line}`);
    }
    return { tag, commit };
}

export function apiclient() {
    const line = rows()[1] ?? '';
    const [version, integrity] = line.split('\t');
    if (!/^\d+\.\d+\.\d+$/.test(version ?? '') || !/^sha\d{3}-[A-Za-z0-9+/]+={0,2}$/.test(integrity ?? '')) {
        throw new Error(`reference/PINNED row 2 is not '<version>\\t<npm integrity>': ${line}`);
    }
    return { version, integrity };
}

export function vendored() {
    const line = rows()[2] ?? '';
    const [name, commit] = line.split('\t');
    if (!name || !/^[0-9a-f]{40}$/.test(commit ?? '')) {
        throw new Error(`reference/PINNED row 3 is not '<name>\\t<40-hex-commit>': ${line}`);
    }
    return { name, commit };
}

export function checkedOut(checkout, commit) {
    const head = execFileSync('git', ['-C', checkout, 'rev-parse', 'HEAD'], { encoding: 'utf8' }).trim();
    if (head !== commit) {
        throw new Error(`${checkout} is at ${head}, and reference/PINNED names ${commit}`);
    }
}

export function locked(checkout, version, integrity) {
    const lock = JSON.parse(readFileSync(join(checkout, 'package-lock.json'), 'utf8'));
    const held = lock.packages?.['node_modules/jellyfin-apiclient'];
    if (!held) {
        throw new Error(`${checkout} resolves no node_modules/jellyfin-apiclient`);
    }
    if (held.version !== version || held.integrity !== integrity) {
        throw new Error(
            `${checkout} resolves jellyfin-apiclient ${held.version} ${held.integrity}, and reference/PINNED names ${version} ${integrity}`
        );
    }
}
