#!/usr/bin/env node
// check-no-raw-bootstrap.mjs — migration completeness gate for dioxus-bootstrap-css consumers.
//
// Fails (exit 1) when consumer code reintroduces the three things this crate exists to remove:
//   1. CDN      — Bootstrap CSS/JS/icons fetched from a remote host (breaks offline-first).
//   2. JS       — Bootstrap's JavaScript (data-bs-* attributes, bootstrap.bundle/min.js, new bootstrap.*).
//   3. RAW      — raw Bootstrap *component* classes (btn, card, modal, ...) instead of the typed components.
//
// Layout/utility classes (container, row, col-*, d-flex, m-*, p-*, text-*, form-label, ...) are allowed:
// the guide says to use plain `div { class: "..." }` for static layout. Only component classes that have a
// typed equivalent in this crate are forbidden, matching docs/MIGRATION.md.
//
// Usage:
//   node tools/check-no-raw-bootstrap.mjs [dir ...]     # default: examples
//   node tools/check-no-raw-bootstrap.mjs ../my-app/src # point it at any consumer crate
//
// Scans .rs (RSX), .html and .js (bundled assets). Does NOT scan .css (bundled Bootstrap legitimately
// contains these tokens) or this crate's own crates/ source (the components are what *emit* the classes).

import fs from 'fs';
import path from 'path';

const ROOTS = process.argv.slice(2);
const TARGETS = ROOTS.length ? ROOTS : ['examples'];

// Component classes with a typed equivalent in the crate (docs/MIGRATION.md).
//
// Two match modes, because some roots own a whole family (every `dropdown-*` is rendered by Dropdown)
// while others have *content* sub-classes with no typed equivalent (`card-title`/`card-text` live inside a
// Card's body and stay raw — there is no CardTitle component). Getting this split right is what keeps the
// gate from flagging idiomatic code; it is calibrated so the crate's own examples and a clean consumer pass.

// Whole family forbidden: token === base OR token starts with "<base>-".
const FORBID_PREFIX = [
  'btn', 'dropdown', 'accordion', 'offcanvas', 'toast', 'carousel',
  'list-group', 'breadcrumb', 'spinner-border', 'spinner-grow', 'tooltip', 'popover',
  'bs-tooltip', 'bs-popover',
  'navbar-toggler', 'navbar-collapse',
];
// Only these exact structural classes forbidden; their content siblings (card-title, modal-title,
// alert-heading, ...) have no typed component and stay raw.
const FORBID_EXACT = new Set([
  'card', 'card-body', 'card-header', 'card-footer', 'card-group',
  'modal', 'modal-dialog', 'modal-content', 'modal-header', 'modal-body', 'modal-footer',
  'alert', 'badge', 'table', 'collapse',
  'nav-tabs', 'nav-pills', 'navbar-nav', 'navbar-nav-scroll',
  'progress', 'progress-bar', 'pagination', 'page-item', 'page-link',
  'popover-header', 'popover-body',
  // Form fields with typed equivalents (Input / Select / Textarea). Exact match
  // only: residual size/variant modifiers (form-control-sm, form-control-color,
  // form-select-lg) ride on the typed component's own class and stay allowed.
  'form-control', 'form-select',
]);
// Allowed even though a FORBID_PREFIX base would otherwise catch them (utilities / state, no component).
const ALLOW_EXACT = new Set(['btn-close', 'collapsed', 'collapsing']);

const SKIP_DIRS = new Set(['target', 'node_modules', '.git', 'dist', 'app_dist', 'pkg', '.dx']);
const SCAN_EXT = new Set(['.rs', '.html', '.htm', '.js', '.mjs']);

const CDN_RE = /(https?:)?\/\/[^"'`\s)]*\b(cdn\.jsdelivr\.net|bootstrapcdn\.com|unpkg\.com|stackpath\.[^/"'`\s]+|cdnjs\.cloudflare\.com)\/[^"'`\s)]*bootstrap/i;
// `data-bs-*` attributes drive Bootstrap's JavaScript (toggle, target, dismiss,
// ride, spy, ...) — forbidden, since that JS is not loaded in WASM. The sole
// exception is `data-bs-theme`: it is the CSS-only color-mode hook (Bootstrap's
// stylesheet reads it, no JS involved), and this crate's own ThemeProvider emits
// it via `documentElement.setAttribute('data-bs-theme', …)`. Exempt it so
// consumers can set the color mode the same idiomatic way.
const JS_DATA_RE = /\bdata-bs-(?!theme\b)[a-z-]+/g;
const JS_API_RE = /\b(bootstrap\.bundle(?:\.min)?\.js|bootstrap(?:\.min)?\.js|new\s+bootstrap\.[A-Z]\w*|bootstrap\.(Modal|Dropdown|Collapse|Offcanvas|Tab|Toast|Tooltip|Popover|Carousel|ScrollSpy)\b)/g;
// class string literals: RSX `class: "..."` and HTML `class="..."`
const CLASS_RE = /\bclass\s*[:=]\s*"([^"]*)"/g;
// conditional class attrs: RSX `class: if cond { "a" } else { "b" }` / `class: match … { … }`.
// The literal CLASS_RE above misses these because the quote isn't adjacent to `class:`.
// Match from the `class:` up to the next attribute (`, name:`) or end of line, then check
// every string literal inside. Only triggers on `if`/`match` so it never touches plain
// `class: "..."` (already covered) — keeps false positives near zero.
const COND_CLASS_RE = /\bclass\s*[:=]\s*(?:if|match)\b.*?(?=,\s*[a-zA-Z_][\w-]*\s*[:=]|$)/g;

const violations = [];
function tokenForbidden(tok) {
  if (!tok || ALLOW_EXACT.has(tok)) return false;
  if (FORBID_EXACT.has(tok)) return true;
  for (const base of FORBID_PREFIX) {
    if (tok === base || tok.startsWith(base + '-')) return true;
  }
  return false;
}

function scanFile(file) {
  let text;
  try { text = fs.readFileSync(file, 'utf8'); } catch { return; }
  const lines = text.split('\n');
  lines.forEach((line, i) => {
    const at = (msg) => violations.push({ file, line: i + 1, msg, src: line.trim().slice(0, 140) });
    if (CDN_RE.test(line)) at('CDN: remote Bootstrap asset (use bundled BootstrapHead, keep offline-first)');
    const dataBs = line.match(JS_DATA_RE);
    if (dataBs) at(`JS: ${[...new Set(dataBs)].join(', ')} (Bootstrap JS is not loaded; use the signal-driven component)`);
    const jsApi = line.match(JS_API_RE);
    if (jsApi) at(`JS: ${[...new Set(jsApi)].join(', ')} (Bootstrap JavaScript is not loaded in WASM)`);
    let m;
    CLASS_RE.lastIndex = 0;
    while ((m = CLASS_RE.exec(line)) !== null) {
      // strip RSX interpolation braces so `{foo}` inside a class string is ignored
      const tokens = m[1].replace(/\{[^}]*\}/g, ' ').split(/\s+/).filter(Boolean);
      const bad = tokens.filter(tokenForbidden);
        if (bad.length) at(`RAW: raw Bootstrap component class "${bad.join(' ')}" (use the typed component, see docs/MIGRATION.md)`);
    }
    // conditional class attrs: check every string literal in `class: if/match { … }`
    let c;
    COND_CLASS_RE.lastIndex = 0;
    while ((c = COND_CLASS_RE.exec(line)) !== null) {
      for (const lit of c[0].match(/"[^"]*"/g) || []) {
        const tokens = lit.slice(1, -1).replace(/\{[^}]*\}/g, ' ').split(/\s+/).filter(Boolean);
        const bad = tokens.filter(tokenForbidden);
      if (bad.length) at(`RAW: raw Bootstrap component class "${bad.join(' ')}" in a conditional class (use the typed component, see docs/MIGRATION.md)`);
      }
      if (c.index === COND_CLASS_RE.lastIndex) COND_CLASS_RE.lastIndex++; // guard zero-width
    }
  });
}

function walk(dir) {
  let entries;
  try { entries = fs.readdirSync(dir, { withFileTypes: true }); } catch { return; }
  for (const e of entries) {
    if (e.isDirectory()) { if (!SKIP_DIRS.has(e.name)) walk(path.join(dir, e.name)); }
    else if (SCAN_EXT.has(path.extname(e.name))) scanFile(path.join(dir, e.name));
  }
}

let scannedRoots = [];
for (const t of TARGETS) {
  if (fs.existsSync(t)) { scannedRoots.push(t); walk(t); }
  else console.error(`warning: target not found, skipped: ${t}`);
}

if (!scannedRoots.length) { console.error('error: no scannable targets'); process.exit(2); }

if (violations.length === 0) {
  console.log(`OK  no raw Bootstrap / CDN / JS found in: ${scannedRoots.join(', ')}`);
  process.exit(0);
}

console.error(`FAIL  ${violations.length} migration violation(s):\n`);
for (const v of violations) {
  console.error(`  ${v.file}:${v.line}`);
  console.error(`      ${v.msg}`);
  console.error(`      > ${v.src}`);
}
console.error(`\nFix: replace with typed components (docs/MIGRATION.md), drop CDN links (use BootstrapHead), and drive interactive state with Dioxus signals.`);
process.exit(1);
