#!/usr/bin/env node

// Reads one playbackInfoBody request as JSON on stdin and writes the pinned
// reference's answer as JSON on stdout. getPlaybackInfo reads only its
// arguments, so this needs no browser.
//
//     node tools/reference/parity.mjs

import { readFileSync } from 'node:fs';
import { playbackInfoBody } from '../../jellium-web/reference/jellyfin-web.mjs';

process.stdout.write(JSON.stringify(playbackInfoBody(JSON.parse(readFileSync(0, 'utf8')))));
