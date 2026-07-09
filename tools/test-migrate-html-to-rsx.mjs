#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

import { transformHtml, transformHtmlToTyped } from './migrate-html-to-rsx.mjs';

const ROOT = path.dirname(fileURLToPath(import.meta.url));
const FIXTURES = path.join(ROOT, 'fixtures');

function read(name) {
  return fs.readFileSync(path.join(FIXTURES, name), 'utf8').replace(/\r\n/g, '\n');
}

function assertEqual(name, actual, expected) {
  if (actual !== expected) {
    console.error(`FAIL ${name}`);
    console.error('--- actual ---');
    console.error(actual);
    console.error('--- expected ---');
    console.error(expected);
    process.exit(1);
  }
}

// Deterministic conversions: output matches the golden .rs and nothing is flagged.
for (const name of ['card', 'card-link', 'table-for', 'if-block', 'interp', 'form', 'inline-ws']) {
  const result = transformHtml(read(`${name}.input.html`), `${name}.input.html`);
  assertEqual(`${name} conversion`, result.source, read(`${name}.expected.rs`));
  assertEqual(`${name} warnings`, JSON.stringify(result.warnings), '[]');
}

// table-for exercises {% for %} + {{ expr }}.
const tableFor = transformHtml(read('table-for.input.html'), 'table-for.input.html');
assertEqual('table-for has one for block', String(tableFor.stats.forBlocks), '1');
assertEqual('table-for has two exprs', String(tableFor.stats.exprs), '2');

// if-block exercises {% if %}/{% else %}.
const ifBlock = transformHtml(read('if-block.input.html'), 'if-block.input.html');
assertEqual('if-block has one if block', String(ifBlock.stats.ifBlocks), '1');

// handlers must be flagged, not guessed: inline onclick + <script>.
const handlers = transformHtml(read('handlers.input.html'), 'handlers.input.html');
assertEqual('handlers matches golden', handlers.source, read('handlers.expected.rs'));
assertEqual('handlers warning count', String(handlers.warnings.length), '2');
assertEqual('handlers warning kinds', handlers.warnings.every((w) => w.kind === 'manual_review') ? 'ok' : 'bad', 'ok');
if (!handlers.source.includes('// TODO(convert): onclick=')) {
  console.error('FAIL handlers must emit a TODO(convert) for the onclick handler');
  process.exit(1);
}
if (!handlers.warnings.some((w) => /script/.test(w.message))) {
  console.error('FAIL handlers must flag the inline <script>');
  process.exit(1);
}

// --pipe / transformHtmlToTyped runs both stages: the card fixture ends up as
// typed dbcss components with no raw Bootstrap component classes left behind.
const piped = transformHtmlToTyped(read('card.input.html'), 'card.input.html');
assertEqual('piped card warnings', JSON.stringify(piped.warnings), '[]');
if (!piped.source.includes('Card {') || !piped.source.includes('Button {')) {
  console.error('FAIL piped card must emit typed Card + Button');
  process.exit(1);
}
// A bare `card` / `btn` component class must be gone (Card/Button own it), but
// allowed content classes like `card-title` / `card-text` legitimately stay.
if (/class:\s*"(?:[^"]*\s)?(?:card|btn)(?:\s[^"]*)?"/.test(piped.source)) {
  console.error('FAIL piped card must not leave a bare card/btn component class');
  process.exit(1);
}

// A clickable whole-card link (<a class="card" href>) must become a typed Card
// in anchor mode: Card { href } with the residual classes preserved and no bare
// `card` class left behind (dbcss 0.5.12 Card gained the href/target props).
const pipedLink = transformHtmlToTyped(read('card-link.input.html'), 'card-link.input.html');
assertEqual('piped card-link warnings', JSON.stringify(pipedLink.warnings), '[]');
if (!pipedLink.source.includes('Card {') || !/href:\s*"\/docs\/hero_router"/.test(pipedLink.source)) {
  console.error('FAIL piped card-link must emit a typed Card in href anchor mode');
  console.error(pipedLink.source);
  process.exit(1);
}
if (/class:\s*"(?:[^"]*\s)?card(?:\s[^"]*)?"/.test(pipedLink.source)) {
  console.error('FAIL piped card-link must not leave a bare card component class');
  process.exit(1);
}

console.log('OK migrate-html-to-rsx fixtures passed');
