#!/usr/bin/env node
/**
 * visual-parity.mjs — objective screenshot A/B scorer for migration parity.
 *
 * The no-raw-bootstrap gate proves the markup is clean; it cannot prove the
 * converted control still LOOKS like the original. This scorer closes that gap
 * with a number instead of an eyeball: it crops two screenshots to the same
 * region and runs ImageMagick `compare -metric AE` (absolute count of differing
 * pixels). Capture both sides yourself — a golden from the original control and
 * a candidate from the converted one — at a fixed viewport, then score here.
 *
 * The gate is NOT "AE must be 0". A component swap has an irreducible few-pixel
 * delta (font hinting, sub-pixel placement). The number exists to catch GROSS
 * regressions (wrong size, lost colour, missing element, mis-placed arrow) and
 * to force every real delta to be classified intended-vs-regression rather than
 * silently drifting. Pick a --threshold from a known-good baseline pair.
 *
 * Usage:
 *   node tools/visual-parity.mjs --golden g.png --candidate c.png \
 *       [--crop WxH+X+Y] [--fuzz 2%] [--threshold 4000] [--out diff.png]
 *
 * Exit code: 0 when AE <= threshold (or no threshold given), 1 when AE exceeds
 * it, 2 on a tooling error. Prints the AE count on stdout.
 */

import { spawnSync } from 'node:child_process';
import { mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

function parseArgs(argv) {
  const args = {};
  for (let i = 0; i < argv.length; i += 2) {
    const key = argv[i];
    if (!key.startsWith('--')) fail(`unexpected argument: ${key}`);
    args[key.slice(2)] = argv[i + 1];
  }
  return args;
}

function fail(msg) {
  console.error(`visual-parity: ${msg}`);
  process.exit(2);
}

function which(bin) {
  const r = spawnSync('sh', ['-c', `command -v ${bin}`], { encoding: 'utf8' });
  return r.status === 0 ? r.stdout.trim() : null;
}

// ImageMagick 6 ships `compare`/`convert`; IM7 ships `magick compare`. Detect.
const IM6 = { compare: which('compare'), convert: which('convert') };
const HAS_IM6 = IM6.compare && IM6.convert;
const HAS_IM7 = which('magick');
if (!HAS_IM6 && !HAS_IM7) fail('ImageMagick not found (need `compare`/`convert` or `magick`).');

function im(tool, extra) {
  return HAS_IM6 ? [IM6[tool], ...extra] : ['magick', tool, ...extra];
}

function crop(src, geom, dst) {
  const [bin, ...rest] = im('convert', [src, '-crop', geom, '+repage', dst]);
  const r = spawnSync(bin, rest, { encoding: 'utf8' });
  if (r.status !== 0) fail(`crop failed for ${src}: ${r.stderr}`);
}

const args = parseArgs(process.argv.slice(2));
if (!args.golden || !args.candidate) fail('need --golden and --candidate');

const fuzz = args.fuzz || '1%';
const out = args.out || join(mkdtempSync(join(tmpdir(), 'vparity-')), 'diff.png');

let golden = args.golden;
let candidate = args.candidate;
if (args.crop) {
  const dir = mkdtempSync(join(tmpdir(), 'vparity-'));
  golden = join(dir, 'g.png');
  candidate = join(dir, 'c.png');
  crop(args.golden, args.crop, golden);
  crop(args.candidate, args.crop, candidate);
}

// `compare -metric AE` prints the pixel count to stderr and exits 1 when the
// images differ — so a non-zero exit is normal, not an error. Only status 2 is.
const [bin, ...rest] = im('compare', ['-metric', 'AE', '-fuzz', fuzz, golden, candidate, out]);
const r = spawnSync(bin, rest, { encoding: 'utf8' });
if (r.status === 2 || r.status === null) fail(`compare failed: ${r.stderr || r.error}`);

const ae = parseInt((r.stderr || '').trim().split(/\s+/)[0], 10);
if (Number.isNaN(ae)) fail(`could not parse AE from: ${JSON.stringify(r.stderr)}`);

const threshold = args.threshold != null ? parseInt(args.threshold, 10) : null;
const verdict = threshold == null ? 'no-threshold' : ae <= threshold ? 'PASS' : 'FAIL';
console.log(`AE=${ae}${threshold == null ? '' : ` threshold=${threshold} ${verdict}`} diff=${out}`);
process.exit(threshold != null && ae > threshold ? 1 : 0);
