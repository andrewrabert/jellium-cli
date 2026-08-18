// The one reader of reference/provenance.tsv: the rows it holds, the sources
// they name, and the lines one construct's row spans, refused unless the
// checkout's own text digests to what the row recorded.

import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..', '..');

const HEADER = 'construct\tpath\tfirst\tlast\tsha256\tkind';

// The root a path names when it is not read from the jellyfin-web checkout.
const BUNDLED = 'jellyfin-apiclient:';

// The entry of the bundle's `sourcesContent` a `BUNDLED` path names.
const SOURCE = 'src/apiClient.js';

// Every row of reference/provenance.tsv, in file order: the construct it names,
// the path and the one-based line pair it spans, the digest it records and the
// kind it carries. Refuses a file that does not open with its header and a row
// that does not hold six fields.
export function rows() {
    const lines = readFileSync(join(root, 'reference', 'provenance.tsv'), 'utf8').split('\n');
    if (lines[0] !== HEADER) {
        throw new Error('reference/provenance.tsv does not open with its header');
    }
    return lines
        .slice(1)
        .filter((line) => line !== '')
        .map((line, offset) => {
            const fields = line.split('\t');
            if (fields.length !== 6) {
                throw new Error(
                    `reference/provenance.tsv:${offset + 2} holds ${fields.length} fields, and a row holds six`
                );
            }
            const [construct, path, first, last, sha256, kind] = fields;
            return {
                construct,
                path,
                first: Number(first),
                last: Number(last),
                sha256,
                kind
            };
        });
}

function bundled(checkout) {
    const map = join(
        checkout,
        'node_modules',
        'jellyfin-apiclient',
        'dist',
        'jellyfin-apiclient.js.map'
    );
    const document = JSON.parse(readFileSync(map, 'utf8'));
    const at = document.sources.findIndex((source) => source.endsWith(SOURCE));
    if (at < 0) {
        throw new Error(`${map} carries no ${SOURCE}`);
    }
    return document.sourcesContent[at];
}

// The text of one source of the checkout, read once and held; a path opening
// with `jellyfin-apiclient:` is answered from the bundle's own
// `sourcesContent`.
export function texts(checkout) {
    const read = new Map();
    return (path) => {
        if (!read.has(path)) {
            read.set(
                path,
                path.startsWith(BUNDLED) ? bundled(checkout) : readFileSync(join(checkout, path), 'utf8')
            );
        }
        return read.get(path);
    };
}

// The lines one construct's row spans, taken from the reader `texts` answers.
// Refuses a construct no row names, a source shorter than the row's last line,
// and a span whose text does not digest to the row's recorded sha256.
export function span(read, construct) {
    const row = rows().find((held) => held.construct === construct);
    if (!row) {
        throw new Error(`reference/provenance.tsv names no ${construct}`);
    }
    const lines = read(row.path).split('\n');
    if (row.last > lines.length) {
        throw new Error(`${row.path} is shorter than line ${row.last}`);
    }
    const text = lines.slice(row.first - 1, row.last).join('\n');
    const digest = createHash('sha256').update(text).digest('hex');
    if (digest !== row.sha256) {
        throw new Error(
            `${construct} spans ${row.path}:${row.first}-${row.last}, which digests to ${digest} and not to the ${row.sha256} its row records`
        );
    }
    return text;
}
