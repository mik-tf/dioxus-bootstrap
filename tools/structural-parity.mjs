#!/usr/bin/env node
/**
 * structural-parity.mjs — deterministic element-by-element parity checker.
 *
 * The pixel scorer (visual-parity.mjs) is a coarse backstop: it works on a
 * cropped screenshot, so it is crop-dependent and blurs subtle deltas into
 * font anti-aliasing noise (grey-vs-dark text, one glyph vs another, a 2px
 * border difference). This checker is the PRIMARY gate: it walks the original
 * control and the converted control in lockstep and compares every element's
 * computed style, geometry, and text, then prints an exact list of every
 * property that differs. No crop to choose, no AA noise, no eyeballing — fix
 * the listed mismatches, re-run, and when the list is empty (minus intended
 * deltas) the control is identical BY CONSTRUCTION.
 *
 * ## Flow
 *   1. On the ORIGINAL page (headless browser), snapshot the control:
 *        node structural-parity.mjs --emit-js        # prints the snapshot fn
 *      then evaluate `(<fn>)('<root-selector>')` and save the JSON to golden.json
 *   2. On the CONVERTED page, same → candidate.json
 *   3. node structural-parity.mjs --a golden.json --b candidate.json \
 *        [--ignore-text 'Last checked'] [--only <css-prop,css-prop>]
 *
 * Exit code 0 when there are no (unignored) mismatches, 1 otherwise.
 *
 * ## Why computed style, not pixels
 * getComputedStyle resolves the ACTUAL rendered value (colour, font, padding,
 * border, radius, display) regardless of how it was authored (class, inline,
 * CSS var, fallback). Two controls that resolve to the same computed values
 * render identically; the pixels are a consequence. Comparing the cause is
 * deterministic where comparing the effect (pixels) is noisy.
 */

import { readFileSync } from 'node:fs';

// Browser-side snapshot function, kept as source so it can be injected verbatim
// into any headless browser / devtools. Returns a flat, document-order list of
// { path, tag, text, w, h, style{} } for the subtree rooted at `sel`. `path`
// encodes tree position (tag + child index) so a structural divergence between
// the two trees is itself detected, not silently mis-aligned.
export const SNAPSHOT_JS = String.raw`
(function snap(sel){
  var PROPS=['color','background-color','font-size','font-weight','font-family',
    'font-style','line-height','text-align','text-decoration-line','white-space',
    'padding-top','padding-right','padding-bottom','padding-left',
    'margin-top','margin-right','margin-bottom','margin-left',
    'border-top-width','border-right-width','border-bottom-width','border-left-width',
    'border-top-style','border-top-color','border-bottom-color',
    'border-top-left-radius','border-top-right-radius','display','opacity','box-sizing'];
  var out=[];
  function walk(el,path){
    var cs=getComputedStyle(el),st={};
    for(var i=0;i<PROPS.length;i++)st[PROPS[i]]=cs.getPropertyValue(PROPS[i]);
    var t='';for(var j=0;j<el.childNodes.length;j++){var n=el.childNodes[j];if(n.nodeType===3)t+=n.nodeValue;}
    t=t.replace(/\s+/g,' ').trim();
    var r=el.getBoundingClientRect();
    out.push({path:path,tag:el.tagName.toLowerCase(),text:t,w:Math.round(r.width),h:Math.round(r.height),style:st});
    var k=0;for(var c=0;c<el.children.length;c++){var ch=el.children[c];walk(ch,path+'/'+(k++)+ch.tagName.toLowerCase());}
  }
  var el=document.querySelector(sel);if(!el)return null;walk(el,el.tagName.toLowerCase());return out;
})
`;

function fail(msg) {
  console.error(`structural-parity: ${msg}`);
  process.exit(2);
}

function parseArgs(argv) {
  const args = {};
  for (let i = 0; i < argv.length; i++) {
    const k = argv[i];
    if (k === '--emit-js') { args.emitJs = true; continue; }
    if (!k.startsWith('--')) fail(`unexpected argument: ${k}`);
    args[k.slice(2)] = argv[++i];
  }
  return args;
}

function diff(a, b, { ignoreText, only }) {
  const propFilter = only ? new Set(only.split(',')) : null;
  const out = [];
  const n = Math.max(a.length, b.length);
  for (let i = 0; i < n; i++) {
    const x = a[i], y = b[i];
    if (!x || !y) {
      out.push({ path: (x || y).path, kind: 'structure', detail: `node only on side ${x ? 'A' : 'B'} (${(x || y).tag})` });
      continue;
    }
    if (x.path !== y.path || x.tag !== y.tag) {
      out.push({ path: x.path, kind: 'structure', detail: `A=${x.tag}@${x.path} vs B=${y.tag}@${y.path}` });
      continue;
    }
    if (x.w !== y.w) out.push({ path: x.path, kind: 'geometry', prop: 'width', a: x.w, b: y.w });
    if (x.h !== y.h) out.push({ path: x.path, kind: 'geometry', prop: 'height', a: x.h, b: y.h });
    const bothVolatile = ignoreText && ignoreText.test(x.text) && ignoreText.test(y.text);
    if (x.text !== y.text && !bothVolatile) out.push({ path: x.path, kind: 'text', a: x.text, b: y.text });
    for (const p in x.style) {
      if (propFilter && !propFilter.has(p)) continue;
      if (x.style[p] !== y.style[p]) out.push({ path: x.path, kind: 'style', prop: p, a: x.style[p], b: y.style[p] });
    }
  }
  return out;
}

const args = parseArgs(process.argv.slice(2));

if (args.emitJs) {
  process.stdout.write(SNAPSHOT_JS.trim() + '\n');
  process.exit(0);
}

if (!args.a || !args.b) fail('need --a <golden.json> --b <candidate.json> (or --emit-js)');

let a, b;
try { a = JSON.parse(readFileSync(args.a, 'utf8')); } catch (e) { fail(`cannot read --a: ${e.message}`); }
try { b = JSON.parse(readFileSync(args.b, 'utf8')); } catch (e) { fail(`cannot read --b: ${e.message}`); }
if (!Array.isArray(a) || !Array.isArray(b)) fail('snapshots must be JSON arrays (did the selector match?)');

const ignoreText = args['ignore-text'] ? new RegExp(args['ignore-text']) : null;
const mismatches = diff(a, b, { ignoreText, only: args.only });

if (mismatches.length === 0) {
  console.log(`structural-parity: PASS — ${a.length} nodes, 0 mismatches`);
  process.exit(0);
}

console.log(`structural-parity: ${mismatches.length} mismatch(es) across ${a.length}/${b.length} nodes:\n`);
for (const m of mismatches) {
  if (m.kind === 'structure') console.log(`  [structure] ${m.path}: ${m.detail}`);
  else if (m.kind === 'text') console.log(`  [text]      ${m.path}: A="${m.a}" B="${m.b}"`);
  else console.log(`  [${m.kind}]${' '.repeat(Math.max(1, 10 - m.kind.length))}${m.path} · ${m.prop}: A=${m.a} B=${m.b}`);
}
process.exit(1);
