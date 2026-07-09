#!/usr/bin/env node
// migrate-html-to-rsx.mjs — front-end converter: plain / Askama HTML template -> Dioxus `rsx!` markup.
//
// This is the first stage of the HTML -> RSX -> typed-dbcss pipeline. It does the deterministic
// structural translation (tags, attributes, text, nesting, comments, and the common Askama control
// flow) that has no typed-component opinion, and FLAGS anything it cannot translate safely with a
// `// TODO(convert): ...` comment plus a manual-review warning. The output is ordinary Dioxus rsx!
// that you then feed into `migrate-bootstrap-rsx.mjs`, which turns the Bootstrap component classes
// into typed components.
//
// Philosophy (same as migrate-bootstrap-rsx.mjs):
//   - Convert safe/static cases deterministically.
//   - Flag dynamic/ambiguous cases for manual review instead of guessing.
//   - If a shape has no clean representation, flag it here and fix upstream — never a per-site hack.
//
// Usage:
//   node tools/migrate-html-to-rsx.mjs page.html                 # print rsx! to stdout
//   node tools/migrate-html-to-rsx.mjs page.html -o page.rs      # write to a file (- = stdout)
//   node tools/migrate-html-to-rsx.mjs --write templates/        # write <name>.rs next to each .html
//   node tools/migrate-html-to-rsx.mjs --check templates/        # parse only, exit 2 if anything flagged
//   node tools/migrate-html-to-rsx.mjs --json page.html          # machine-readable summary
//   node tools/migrate-html-to-rsx.mjs --pipe page.html          # run both stages: emit typed-dbcss rsx

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

import { transformSource } from './migrate-bootstrap-rsx.mjs';

const SKIP_DIRS = new Set(['.git', 'target', 'node_modules', '.dx', 'dist', 'pkg', 'app_dist']);

// HTML void elements — no closing tag, never have children.
const VOID_ELEMENTS = new Set([
  'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input',
  'link', 'meta', 'param', 'source', 'track', 'wbr',
]);

// Elements whose body is raw text (not markup). We flag script/style because their content must be
// hand-ported to Dioxus event handlers / a stylesheet asset.
const RAW_TEXT_ELEMENTS = new Set(['script', 'style']);

// Attribute names that collide with Rust keywords and need the raw-identifier form in rsx.
const RUST_KEYWORD_ATTRS = new Set([
  'for', 'type', 'loop', 'as', 'async', 'await', 'ref', 'move', 'use', 'in',
  'mut', 'self', 'fn', 'let', 'static', 'dyn', 'impl', 'match', 'else', 'while',
  'struct', 'enum', 'trait', 'mod', 'pub', 'crate', 'super', 'where', 'box',
]);

// Valueless HTML attributes that map to a boolean `true` in Dioxus.
const BOOLEAN_ATTRS = new Set([
  'disabled', 'checked', 'selected', 'readonly', 'required', 'multiple',
  'autofocus', 'hidden', 'novalidate', 'defer', 'open', 'autoplay', 'controls',
  'loop', 'muted', 'default', 'reversed', 'ismap', 'nomodule',
]);

function lineCol(source, index) {
  const prefix = source.slice(0, index);
  const lines = prefix.split('\n');
  return { line: lines.length, column: lines[lines.length - 1].length + 1 };
}

// --- HTML entity decoding (common named + numeric) -------------------------------------------------

const NAMED_ENTITIES = new Map([
  ['amp', '&'], ['lt', '<'], ['gt', '>'], ['quot', '"'], ['apos', "'"],
  ['nbsp', ' '], ['copy', '©'], ['reg', '®'], ['hellip', '…'],
  ['mdash', '—'], ['ndash', '–'], ['times', '×'], ['middot', '·'],
  ['laquo', '«'], ['raquo', '»'], ['deg', '°'], ['trade', '™'],
]);

function decodeEntities(text) {
  return text.replace(/&(#x?[0-9a-fA-F]+|[a-zA-Z][a-zA-Z0-9]*);/g, (whole, body) => {
    if (body[0] === '#') {
      const code = body[1] === 'x' || body[1] === 'X'
        ? parseInt(body.slice(2), 16)
        : parseInt(body.slice(1), 10);
      return Number.isFinite(code) ? String.fromCodePoint(code) : whole;
    }
    return NAMED_ENTITIES.has(body) ? NAMED_ENTITIES.get(body) : whole;
  });
}

// --- Tokenizer -------------------------------------------------------------------------------------
// Recognizes: HTML comments, Askama comments {# #}, Askama expr {{ }}, Askama stmt {% %}, open/close
// tags (with attribute parsing), raw-text elements (script/style), and plain text.

function stripTrimMarkers(inner) {
  return inner.replace(/^-/, '').replace(/-$/, '').trim();
}

function tokenize(source) {
  const tokens = [];
  let i = 0;
  const n = source.length;

  while (i < n) {
    // HTML comment
    if (source.startsWith('<!--', i)) {
      const end = source.indexOf('-->', i + 4);
      const stop = end === -1 ? n : end + 3;
      tokens.push({ kind: 'comment', value: source.slice(i + 4, end === -1 ? n : end).trim(), off: i });
      i = stop;
      continue;
    }
    // DOCTYPE / processing — drop it
    if (source.startsWith('<!', i)) {
      const end = source.indexOf('>', i);
      i = end === -1 ? n : end + 1;
      continue;
    }
    // Askama comment {# #}
    if (source.startsWith('{#', i)) {
      const end = source.indexOf('#}', i + 2);
      const stop = end === -1 ? n : end + 2;
      tokens.push({ kind: 'comment', value: source.slice(i + 2, end === -1 ? n : end).trim(), off: i });
      i = stop;
      continue;
    }
    // Askama expr {{ }}
    if (source.startsWith('{{', i)) {
      const end = source.indexOf('}}', i + 2);
      const stop = end === -1 ? n : end + 2;
      tokens.push({ kind: 'expr', raw: stripTrimMarkers(source.slice(i + 2, end === -1 ? n : end)), off: i });
      i = stop;
      continue;
    }
    // Askama stmt {% %}
    if (source.startsWith('{%', i)) {
      const end = source.indexOf('%}', i + 2);
      const stop = end === -1 ? n : end + 2;
      tokens.push({ kind: 'stmt', value: stripTrimMarkers(source.slice(i + 2, end === -1 ? n : end)), off: i });
      i = stop;
      continue;
    }
    // Close tag </name>
    if (source.startsWith('</', i)) {
      const end = source.indexOf('>', i);
      const stop = end === -1 ? n : end + 1;
      const name = source.slice(i + 2, end === -1 ? n : end).trim().toLowerCase();
      tokens.push({ kind: 'close', name, off: i });
      i = stop;
      continue;
    }
    // Open tag <name ...>
    if (source[i] === '<' && /[a-zA-Z]/.test(source[i + 1] || '')) {
      const parsed = parseOpenTag(source, i);
      tokens.push(parsed.token);
      i = parsed.next;
      // Raw-text element: capture body verbatim to its close tag.
      if (!parsed.token.selfClose && RAW_TEXT_ELEMENTS.has(parsed.token.name)) {
        const closeRe = new RegExp(`</${parsed.token.name}\\s*>`, 'i');
        const rest = source.slice(i);
        const m = closeRe.exec(rest);
        const bodyEnd = m ? i + m.index : n;
        tokens.push({ kind: 'rawtext', name: parsed.token.name, value: source.slice(i, bodyEnd), off: i });
        i = m ? bodyEnd + m[0].length : n;
        tokens.push({ kind: 'close', name: parsed.token.name, off: i });
      }
      continue;
    }
    // Plain text — up to the next special sequence.
    let j = i;
    while (j < n) {
      if (source[j] === '<' && (source[j + 1] === '/' || source[j + 1] === '!' || /[a-zA-Z]/.test(source[j + 1] || ''))) break;
      if (source.startsWith('{{', j) || source.startsWith('{%', j) || source.startsWith('{#', j)) break;
      j += 1;
    }
    if (j === i) j += 1; // stray '<' or '{' — consume one char so we make progress
    tokens.push({ kind: 'text', value: source.slice(i, j), off: i });
    i = j;
  }
  return tokens;
}

function parseOpenTag(source, start) {
  const n = source.length;
  let i = start + 1;
  const nameMatch = /^[a-zA-Z][a-zA-Z0-9:-]*/.exec(source.slice(i));
  const name = nameMatch[0].toLowerCase();
  i += nameMatch[0].length;
  const attrs = [];
  const dynAttrs = []; // askama-driven attribute regions we cannot map cleanly
  let selfClose = false;

  while (i < n) {
    while (/\s/.test(source[i] || '')) i += 1;
    if (source[i] === '>') { i += 1; break; }
    if (source.startsWith('/>', i)) { selfClose = true; i += 2; break; }
    if (i >= n) break;

    // Askama statement / expr sitting between attributes → cannot map to a static attribute.
    if (source.startsWith('{%', i)) {
      const end = source.indexOf('%}', i + 2);
      const stop = end === -1 ? n : end + 2;
      dynAttrs.push({ raw: source.slice(i, stop), off: i });
      i = stop;
      continue;
    }
    if (source.startsWith('{{', i)) {
      const end = source.indexOf('}}', i + 2);
      const stop = end === -1 ? n : end + 2;
      dynAttrs.push({ raw: source.slice(i, stop), off: i });
      i = stop;
      continue;
    }

    const attrNameMatch = /^[^\s=/>]+/.exec(source.slice(i));
    if (!attrNameMatch) { i += 1; continue; }
    const attrName = attrNameMatch[0];
    const attrOff = i;
    i += attrName.length;
    while (/\s/.test(source[i] || '')) i += 1;
    if (source[i] === '=') {
      i += 1;
      while (/\s/.test(source[i] || '')) i += 1;
      let value = '';
      if (source[i] === '"' || source[i] === "'") {
        const quote = source[i];
        const end = source.indexOf(quote, i + 1);
        value = source.slice(i + 1, end === -1 ? n : end);
        i = end === -1 ? n : end + 1;
      } else {
        const vm = /^[^\s>]+/.exec(source.slice(i));
        value = vm ? vm[0] : '';
        i += value.length;
      }
      attrs.push({ name: attrName, value, hasValue: true, off: attrOff });
    } else {
      attrs.push({ name: attrName, value: '', hasValue: false, off: attrOff });
    }
  }

  const isVoid = VOID_ELEMENTS.has(name);
  return {
    token: { kind: 'open', name, attrs, dynAttrs, selfClose: selfClose || isVoid, void: isVoid, off: start },
    next: i,
  };
}

// --- Tree builder ----------------------------------------------------------------------------------

const IF_STARTERS = /^if\b/;
const FOR_STARTERS = /^for\b/;
const MATCH_STARTERS = /^match\b/;
const BLOCK_STARTERS = /^block\b/;

function classifyStmt(value) {
  const head = value.split(/\s+/)[0];
  return head;
}

function buildTree(tokens, source, warnings) {
  const root = { type: 'root', children: [] };
  const frames = [{ kind: 'root', target: root.children }];
  const top = () => frames[frames.length - 1];
  const append = (node) => top().target.push(node);
  const warn = (off, message) => warnings.push({ kind: 'manual_review', off, line: lineCol(source, off).line, message });

  for (const tok of tokens) {
    if (tok.kind === 'text') { append({ type: 'text', value: tok.value, off: tok.off }); continue; }
    if (tok.kind === 'expr') { append({ type: 'expr', raw: tok.raw, off: tok.off }); continue; }
    if (tok.kind === 'comment') { append({ type: 'comment', value: tok.value, off: tok.off }); continue; }
    if (tok.kind === 'rawtext') {
      warn(tok.off, `inline <${tok.name}> must be ported by hand (Dioxus event handlers + signals for scripts, a stylesheet asset for styles)`);
      append({ type: 'flag', off: tok.off, message: `<${tok.name}> block dropped — port it by hand`, raw: `<${tok.name}> ... </${tok.name}>` });
      continue;
    }

    if (tok.kind === 'open') {
      const el = { type: 'el', tag: tok.name, attrs: tok.attrs, dynAttrs: tok.dynAttrs, children: [], off: tok.off };
      if (tok.dynAttrs.length) {
        warn(tok.off, `<${tok.name}> has Askama-driven attributes (${tok.dynAttrs.map((d) => d.raw).join(' ')}); wire them as Dioxus attributes/signals by hand`);
      }
      if (tok.selfClose) { append(el); }
      else { append(el); frames.push({ kind: 'el', tag: tok.name, node: el, target: el.children }); }
      continue;
    }

    if (tok.kind === 'close') {
      // Pop until we find the matching element frame (tolerate malformed / optional-close markup).
      for (let d = frames.length - 1; d >= 1; d -= 1) {
        if (frames[d].kind === 'el' && frames[d].tag === tok.name) {
          frames.length = d;
          break;
        }
      }
      continue;
    }

    if (tok.kind === 'stmt') {
      const head = classifyStmt(tok.value);
      const rest = tok.value.slice(head.length).trim();

      if (IF_STARTERS.test(tok.value)) {
        const node = { type: 'if', off: tok.off, branches: [{ cond: rest, children: [] }], elseChildren: null };
        append(node);
        frames.push({ kind: 'if', node, target: node.branches[0].children });
        continue;
      }
      if (FOR_STARTERS.test(tok.value)) {
        const node = { type: 'for', off: tok.off, head: rest, children: [], elseChildren: null };
        append(node);
        frames.push({ kind: 'for', node, target: node.children });
        continue;
      }
      if (BLOCK_STARTERS.test(tok.value)) {
        const node = { type: 'block', off: tok.off, name: rest, children: [] };
        append(node);
        frames.push({ kind: 'block', node, target: node.children });
        continue;
      }
      if (MATCH_STARTERS.test(tok.value)) {
        warn(tok.off, `Askama {% match ${rest} %} needs manual conversion to a Dioxus match {} expression (arms are emitted below as flat, flagged fragments)`);
        const node = { type: 'match', off: tok.off, expr: rest, arms: [] };
        append(node);
        // Content between `{% match %}` and the first `{% when %}` is (whitespace) noise in valid
        // Askama; route it to a throwaway buffer until the first arm opens.
        frames.push({ kind: 'match', node, target: [] });
        continue;
      }
      if (head === 'elif' || (head === 'else' && rest.startsWith('if'))) {
        const frame = top();
        if (frame.kind === 'if') {
          const cond = head === 'elif' ? rest : rest.slice(2).trim();
          const branch = { cond, children: [] };
          frame.node.branches.push(branch);
          frame.target = branch.children;
        }
        continue;
      }
      if (head === 'else') {
        const frame = top();
        if (frame.kind === 'if') { frame.node.elseChildren = []; frame.target = frame.node.elseChildren; }
        else if (frame.kind === 'for') { frame.node.elseChildren = []; frame.target = frame.node.elseChildren; }
        continue;
      }
      if (head === 'when') {
        const frame = top();
        if (frame.kind === 'match') {
          const arm = { pat: rest, children: [] };
          frame.node.arms.push(arm);
          frame.target = arm.children;
        }
        continue;
      }
      if (head === 'endif' || head === 'endfor' || head === 'endblock' || head === 'endmatch') {
        // Pop the nearest matching control frame.
        const wantKind = { endif: 'if', endfor: 'for', endblock: 'block', endmatch: 'match' }[head];
        for (let d = frames.length - 1; d >= 1; d -= 1) {
          if (frames[d].kind === wantKind) { frames.length = d; break; }
        }
        continue;
      }

      // Everything else (extends / include / import / macro / call / let / set / filter / ...) is flagged.
      warn(tok.off, `Askama {% ${tok.value} %} has no automatic RSX equivalent; convert by hand (template inheritance -> component composition, let/set -> Rust bindings, macro/call -> components)`);
      append({ type: 'flag', off: tok.off, message: `{% ${tok.value} %}`, raw: `{% ${tok.value} %}` });
      continue;
    }
  }
  return root;
}

// --- Renderer --------------------------------------------------------------------------------------

const IND = '    ';

function cleanExpr(raw, off, warnings, context) {
  const parts = raw.split('|').map((s) => s.trim());
  const expr = parts[0];
  const badFilters = parts.slice(1).filter((f) => f && f !== 'safe');
  if (badFilters.length && warnings) {
    warnings.push({
      kind: 'manual_review', off, line: null,
      message: `expression "${raw}" uses filter(s) ${badFilters.map((f) => `|${f}`).join(' ')}; apply the transform in Rust before rendering`,
    });
  }
  const simple = /^[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*$/.test(expr);
  if (!simple && context === 'format' && warnings) {
    warnings.push({
      kind: 'manual_review', off, line: null,
      message: `interpolated expression "{${expr}}" is not a simple path; a Dioxus format string may not accept it — bind it to a variable first`,
    });
  }
  return expr;
}

function escapeText(s) {
  return s.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/\{/g, '{{').replace(/\}/g, '}}');
}

// Render a string-valued attribute or a formatted-string text run. `pieces` is an array of
// { text } / { expr } fragments; returns a quoted Dioxus string literal.
function renderFormatString(pieces, off, warnings) {
  let out = '';
  for (const p of pieces) {
    if (p.text !== undefined) out += escapeText(decodeEntities(p.text));
    else out += `{${cleanExpr(p.expr, p.off ?? off, warnings, 'format')}}`;
  }
  return `"${out}"`;
}

// Convert an attribute value string (may contain {{ }} and {% %}) into format-string pieces.
function attrValuePieces(value, off, warnings) {
  let v = value;
  if (/\{%/.test(v)) {
    warnings.push({
      kind: 'manual_review', off, line: lineCol('', 0).line,
      message: `attribute value "${value}" contains Askama control flow; use a Dioxus conditional attribute (e.g. class: if cond { .. } else { .. })`,
    });
    v = v.replace(/\{%[^]*?%\}/g, ''); // drop the delimiters, keep inner text as a static fallback
  }
  const pieces = [];
  const re = /\{\{([^]*?)\}\}/g;
  let last = 0; let m;
  while ((m = re.exec(v)) !== null) {
    if (m.index > last) pieces.push({ text: v.slice(last, m.index) });
    pieces.push({ expr: stripTrimMarkers(m[1]), off });
    last = re.lastIndex;
  }
  if (last < v.length) pieces.push({ text: v.slice(last) });
  return pieces;
}

function mapAttrName(name) {
  const lower = name.toLowerCase();
  if (/[-:]/.test(name)) return { key: `"${name}"`, raw: false };
  if (RUST_KEYWORD_ATTRS.has(lower)) return { key: `r#${lower}`, raw: true };
  return { key: lower, raw: false };
}

function isEventAttr(name) {
  return /^on[a-z]+$/i.test(name);
}

function renderAttrLines(el, indent, warnings) {
  const lines = [];
  for (const attr of el.attrs) {
    if (isEventAttr(attr.name)) {
      warnings.push({
        kind: 'manual_review', off: attr.off, line: lineCol('', 0).line,
        message: `${el.tag} has ${attr.name}="${attr.value}" — inline handlers must become a Dioxus event handler (${attr.name.toLowerCase()}: move |_| { .. }) driven by signals`,
      });
      lines.push(`${indent}// TODO(convert): ${attr.name}="${attr.value}" -> Dioxus ${attr.name.toLowerCase()} handler + signal`);
      continue;
    }
    const { key } = mapAttrName(attr.name);
    if (!attr.hasValue) {
      const value = BOOLEAN_ATTRS.has(attr.name.toLowerCase()) ? 'true' : '"true"';
      lines.push(`${indent}${key}: ${value},`);
      continue;
    }
    const pieces = attrValuePieces(attr.value, attr.off, warnings);
    lines.push(`${indent}${key}: ${renderFormatString(pieces, attr.off, warnings)},`);
  }
  return lines;
}

// Split children into runs of inline nodes (text/expr) and standalone nodes, so adjacent text and
// interpolations collapse into a single Dioxus string / dyn-node.
function isInline(node) {
  return node.type === 'text' || node.type === 'expr';
}

function renderChildren(children, indent, warnings) {
  const lines = [];
  let run = [];
  const flush = () => {
    if (!run.length) return;
    const rendered = renderInlineRun(run, indent, warnings);
    for (const l of rendered) lines.push(l);
    run = [];
  };
  for (const child of children) {
    if (isInline(child)) { run.push(child); continue; }
    flush();
    for (const l of renderNode(child, indent, warnings)) lines.push(l);
  }
  flush();
  return lines;
}

function renderInlineRun(run, indent, warnings) {
  // Collapse whitespace; drop a run that is only whitespace text.
  const exprs = run.filter((r) => r.type === 'expr');
  const hasText = run.some((r) => r.type === 'text' && decodeEntities(r.value).trim() !== '');
  if (!exprs.length && !hasText) return [];

  // A single interpolation with no surrounding text -> a bare dyn node `{expr}` (accepts any expr).
  if (exprs.length === 1 && !hasText) {
    return [`${indent}{${cleanExpr(exprs[0].raw, exprs[0].off, warnings, 'node')}}`];
  }

  // Otherwise build one formatted string literal, normalizing internal whitespace.
  const pieces = [];
  for (const node of run) {
    if (node.type === 'text') {
      const norm = decodeEntities(node.value).replace(/\s+/g, ' ');
      if (norm) pieces.push({ text: norm });
    } else {
      pieces.push({ expr: node.raw, off: node.off });
    }
  }
  // Trim leading/trailing whitespace on the joined text edges.
  if (pieces.length && pieces[0].text !== undefined) pieces[0].text = pieces[0].text.replace(/^\s+/, '');
  const lastP = pieces[pieces.length - 1];
  if (lastP && lastP.text !== undefined) lastP.text = lastP.text.replace(/\s+$/, '');
  const cleaned = pieces.filter((p) => p.expr !== undefined || p.text !== '');
  if (!cleaned.length) return [];
  return [`${indent}${renderFormatString(cleaned, run[0].off, warnings)}`];
}

function renderNode(node, indent, warnings) {
  switch (node.type) {
    case 'comment': {
      return node.value.split('\n').map((l) => `${indent}// ${l.trim()}`);
    }
    case 'flag': {
      return [`${indent}// TODO(convert): ${node.message}`];
    }
    case 'el': {
      const attrLines = renderAttrLines(node, `${indent}${IND}`, warnings);
      const childLines = renderChildren(node.children, `${indent}${IND}`, warnings);
      if (!attrLines.length && !childLines.length) return [`${indent}${node.tag} {}`];
      return [`${indent}${node.tag} {`, ...attrLines, ...childLines, `${indent}}`];
    }
    case 'if': {
      const lines = [];
      node.branches.forEach((branch, idx) => {
        const kw = idx === 0 ? 'if' : '} else if';
        lines.push(`${indent}${kw} ${branch.cond} {`);
        for (const l of renderChildren(branch.children, `${indent}${IND}`, warnings)) lines.push(l);
      });
      if (node.elseChildren) {
        lines.push(`${indent}} else {`);
        for (const l of renderChildren(node.elseChildren, `${indent}${IND}`, warnings)) lines.push(l);
      }
      lines.push(`${indent}}`);
      return lines;
    }
    case 'for': {
      const lines = [`${indent}for ${node.head} {`];
      for (const l of renderChildren(node.children, `${indent}${IND}`, warnings)) lines.push(l);
      lines.push(`${indent}}`);
      if (node.elseChildren && node.elseChildren.length) {
        lines.push(`${indent}// TODO(convert): {% else %} (empty-loop branch) — render this only when the collection is empty`);
        for (const l of renderChildren(node.elseChildren, indent, warnings)) lines.push(l);
      }
      return lines;
    }
    case 'block': {
      const lines = [`${indent}// block: ${node.name}`];
      for (const l of renderChildren(node.children, indent, warnings)) lines.push(l);
      return lines;
    }
    case 'match': {
      const lines = [`${indent}// TODO(convert): match ${node.expr} { .. } — wire the arms below into a Dioxus match`];
      for (const arm of node.arms) {
        lines.push(`${indent}// when ${arm.pat} =>`);
        for (const l of renderChildren(arm.children, indent, warnings)) lines.push(l);
      }
      return lines;
    }
    default:
      return [];
  }
}

export function transformHtml(source, file = '<memory>') {
  const normalized = source.replace(/\r\n/g, '\n');
  const warnings = [];
  const tokens = tokenize(normalized);
  const tree = buildTree(tokens, normalized, warnings);
  const bodyLines = renderChildren(tree.children, IND, warnings);
  const out = `rsx! {\n${bodyLines.join('\n')}\n}\n`;

  // Fill in real line numbers for warnings raised deep in the renderer (they carry an off).
  for (const w of warnings) {
    if (w.line == null || w.line === 1) w.line = lineCol(normalized, w.off ?? 0).line;
  }
  warnings.sort((a, b) => (a.off ?? 0) - (b.off ?? 0));

  const stats = {
    elements: countNodes(tree, 'el'),
    ifBlocks: countNodes(tree, 'if'),
    forBlocks: countNodes(tree, 'for'),
    exprs: countNodes(tree, 'expr'),
    flagged: warnings.length,
  };
  return { source: out, file, warnings, stats };
}

// Full pipeline in one call: HTML/Askama template -> plain rsx -> typed dbcss
// components. Warnings from both stages are merged so nothing is hidden.
export function transformHtmlToTyped(source, file = '<memory>') {
  const front = transformHtml(source, file);
  const typed = transformSource(front.source, file);
  return {
    source: typed.source,
    file,
    warnings: [...front.warnings, ...typed.warnings],
    stats: front.stats,
  };
}

function countNodes(node, type) {
  let count = node.type === type ? 1 : 0;
  const kids = node.children || [];
  for (const k of kids) count += countNodes(k, type);
  if (node.branches) for (const b of node.branches) for (const k of b.children) count += countNodes(k, type);
  if (node.elseChildren) for (const k of node.elseChildren) count += countNodes(k, type);
  if (node.arms) for (const a of node.arms) for (const k of a.children) count += countNodes(k, type);
  return count;
}

// --- CLI -------------------------------------------------------------------------------------------

function walkTargets(targets) {
  const files = [];
  function walk(p) {
    if (!fs.existsSync(p)) return;
    const st = fs.statSync(p);
    if (st.isDirectory()) {
      if (SKIP_DIRS.has(path.basename(p))) return;
      for (const entry of fs.readdirSync(p)) walk(path.join(p, entry));
    } else if (p.endsWith('.html') || p.endsWith('.htm')) {
      files.push(p);
    }
  }
  for (const target of targets) walk(target);
  return files;
}

function parseArgs(argv) {
  const args = { write: false, check: false, json: false, pipe: false, out: null, targets: [] };
  for (let k = 0; k < argv.length; k += 1) {
    const arg = argv[k];
    if (arg === '--write') args.write = true;
    else if (arg === '--check') args.check = true;
    else if (arg === '--json') args.json = true;
    else if (arg === '--pipe' || arg === '--chain') args.pipe = true;
    else if (arg === '-o' || arg === '--out') { args.out = argv[k + 1]; k += 1; }
    else args.targets.push(arg);
  }
  return args;
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  if (!args.targets.length) {
    console.error('usage: node tools/migrate-html-to-rsx.mjs [--write|--check|--json] [--pipe] [-o out.rs] <file.html|dir> ...');
    process.exit(2);
  }

  const inputs = [];
  for (const t of args.targets) {
    if (fs.existsSync(t) && fs.statSync(t).isDirectory()) inputs.push(...walkTargets([t]));
    else inputs.push(t);
  }

  const results = [];
  for (const file of inputs) {
    const before = fs.readFileSync(file, 'utf8');
    const result = args.pipe ? transformHtmlToTyped(before, file) : transformHtml(before, file);
    let outPath = null;
    if (args.out && inputs.length === 1) outPath = args.out;
    else if (args.write) outPath = file.replace(/\.html?$/, '.rs');

    if (args.check) {
      // parse-only
    } else if (outPath && outPath !== '-') {
      fs.writeFileSync(outPath, result.source);
    } else if (!args.json) {
      process.stdout.write(result.source);
    }
    results.push({ file, out: outPath, warnings: result.warnings, stats: result.stats });
  }

  const allWarnings = results.flatMap((r) => r.warnings.map((w) => ({ ...w, file: r.file })));
  if (args.json) {
    console.log(JSON.stringify({ files: results.map((r) => ({ file: r.file, out: r.out, stats: r.stats, warnings: r.warnings })) }, null, 2));
  } else {
    for (const r of results) {
      if (r.out && r.out !== '-') console.error(`wrote ${r.out}  (elements ${r.stats.elements}, if ${r.stats.ifBlocks}, for ${r.stats.forBlocks}, expr ${r.stats.exprs}, flagged ${r.stats.flagged})`);
    }
    for (const w of allWarnings) {
      console.error(`${w.file}:${w.line}: ${w.kind}: ${w.message}`);
    }
  }

  if (args.check && allWarnings.length) process.exit(2);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  main();
}
