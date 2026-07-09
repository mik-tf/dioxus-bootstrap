#!/usr/bin/env node
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

import { colorProp, sizeProp } from './bootstrap-parity.mjs';

const RAW_TAGS = new Set(['button', 'a', 'span', 'div', 'input', 'select', 'textarea', 'table', 'ul', 'li']);
const SKIP_DIRS = new Set(['.git', 'target', 'node_modules', '.dx', 'dist', 'pkg']);
const SLOT_CLASSES = new Map([
  ['card-header', 'header'],
  ['card-body', 'body'],
  ['card-footer', 'footer'],
]);
const DYNAMIC_COMPONENT_EXACT = new Set([
  'btn',
  'badge',
  'card',
  'card-body',
  'card-header',
  'card-footer',
  'card-group',
  'alert',
  'table',
  'modal',
  'modal-dialog',
  'modal-content',
  'modal-header',
  'modal-body',
  'modal-footer',
  'collapse',
  'progress',
  'progress-bar',
  'pagination',
  'page-item',
  'page-link',
]);
const DYNAMIC_COMPONENT_PREFIXES = [
  'btn-',
  'alert-',
  'table-',
  'spinner-',
  'form-control',
  'form-select',
  'dropdown',
  'navbar',
  'offcanvas',
  'toast',
  'accordion',
  'breadcrumb',
  'list-group',
  'carousel',
  'nav-',
];

function isWord(ch) {
  return /[A-Za-z0-9_-]/.test(ch || '');
}

function lineCol(source, index) {
  const prefix = source.slice(0, index);
  const lines = prefix.split('\n');
  return { line: lines.length, column: lines[lines.length - 1].length + 1 };
}

function skipRustRawString(source, index) {
  if (source[index] !== 'r') return null;
  let i = index + 1;
  let hashes = '';
  while (source[i] === '#') {
    hashes += '#';
    i += 1;
  }
  if (source[i] !== '"') return null;
  const close = `"${hashes}`;
  const end = source.indexOf(close, i + 1);
  return end === -1 ? source.length : end + close.length;
}

function skipString(source, index) {
  const quote = source[index];
  let i = index + 1;
  while (i < source.length) {
    if (source[i] === '\\') {
      i += 2;
    } else if (source[i] === quote) {
      return i + 1;
    } else {
      i += 1;
    }
  }
  return source.length;
}

function skipLineComment(source, index) {
  const end = source.indexOf('\n', index + 2);
  return end === -1 ? source.length : end + 1;
}

function skipBlockComment(source, index) {
  const end = source.indexOf('*/', index + 2);
  return end === -1 ? source.length : end + 2;
}

function skipIgnored(source, index) {
  const rawEnd = skipRustRawString(source, index);
  if (rawEnd !== null) return rawEnd;
  if (source[index] === '"' || source[index] === "'" || source[index] === '`') {
    return skipString(source, index);
  }
  if (source[index] === '/' && source[index + 1] === '/') {
    return skipLineComment(source, index);
  }
  if (source[index] === '/' && source[index + 1] === '*') {
    return skipBlockComment(source, index);
  }
  return null;
}

function findMatchingBrace(source, open) {
  let depth = 0;
  for (let i = open; i < source.length; i += 1) {
    const ignoredEnd = skipIgnored(source, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    if (source[i] === '{') {
      depth += 1;
    } else if (source[i] === '}') {
      depth -= 1;
      if (depth === 0) return i;
    }
  }
  return -1;
}

function findRsxBlocks(source) {
  const blocks = [];
  for (let i = 0; i < source.length; i += 1) {
    const ignoredEnd = skipIgnored(source, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    if (source.startsWith('rsx!', i)) {
      const open = source.indexOf('{', i + 4);
      if (open !== -1) {
        const close = findMatchingBrace(source, open);
        if (close !== -1) blocks.push({ open, close });
        i = close === -1 ? i : close;
      }
    }
  }
  return blocks;
}

function collectElements(body) {
  const out = [];
  for (let i = 0; i < body.length; i += 1) {
    const ignoredEnd = skipIgnored(body, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    const match = /^[A-Za-z][A-Za-z0-9_-]*/.exec(body.slice(i));
    if (!match || isWord(body[i - 1])) continue;
    const tag = match[0];
    let j = i + tag.length;
    while (/\s/.test(body[j] || '')) j += 1;
    if (body[j] !== '{') continue;
    const close = findMatchingBrace(body, j);
    if (close === -1) continue;
    out.push({ tag, start: i, open: j, close, end: close + 1 });
    i = close;
  }
  return out;
}

function readAttrEnd(content, index) {
  let depth = 0;
  for (let i = index; i < content.length; i += 1) {
    const ignoredEnd = skipIgnored(content, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
    } else if (content[i] === '{' || content[i] === '(' || content[i] === '[') {
      depth += 1;
    } else if (content[i] === '}' || content[i] === ')' || content[i] === ']') {
      if (depth === 0) return i;
      depth -= 1;
    } else if (depth === 0 && content[i] === ',') {
      return i + 1;
    }
  }
  return content.length;
}

function findTopLevelClassAttr(content) {
  let depth = 0;
  for (let i = 0; i < content.length; i += 1) {
    const ignoredEnd = skipIgnored(content, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    if (content[i] === '{' || content[i] === '(' || content[i] === '[') {
      depth += 1;
      continue;
    }
    if (content[i] === '}' || content[i] === ')' || content[i] === ']') {
      depth = Math.max(0, depth - 1);
      continue;
    }
    if (depth !== 0 || !content.startsWith('class', i) || isWord(content[i - 1]) || isWord(content[i + 5])) {
      continue;
    }
    let j = i + 5;
    while (/\s/.test(content[j] || '')) j += 1;
    if (content[j] !== ':' && content[j] !== '=') continue;
    j += 1;
    while (/\s/.test(content[j] || '')) j += 1;
    if (content[j] !== '"') {
      const end = readAttrEnd(content, j);
      return { dynamic: true, start: i, end, expression: content.slice(j, end) };
    }
    const valueStart = j + 1;
    const valueEnd = skipString(content, j) - 1;
    let end = valueEnd + 1;
    while (/[ \t]/.test(content[end] || '')) end += 1;
    if (content[end] === ',') end += 1;
    return { dynamic: false, start: i, end, value: content.slice(valueStart, valueEnd) };
  }
  return null;
}

function removeRanges(content, ranges) {
  let out = content;
  for (const range of ranges.sort((a, b) => b.start - a.start)) {
    out = `${out.slice(0, range.start)}${range.replacement ?? ''}${out.slice(range.end)}`;
  }
  return out;
}

function removeClassAttr(content, attr) {
  return removeRanges(content, [attr]);
}

function findTopLevelAttr(content, names) {
  const wanted = new Set(names);
  let depth = 0;
  for (let i = 0; i < content.length; i += 1) {
    if (depth === 0 && content[i] === '"') {
      const nameEnd = skipString(content, i);
      const name = content.slice(i + 1, nameEnd - 1);
      if (!wanted.has(name)) {
        i = nameEnd - 1;
        continue;
      }
      let j = nameEnd;
      while (/\s/.test(content[j] || '')) j += 1;
      if (content[j] !== ':' && content[j] !== '=') {
        i = nameEnd - 1;
        continue;
      }
      j += 1;
      while (/\s/.test(content[j] || '')) j += 1;
      if (content[j] !== '"') {
        const end = readAttrEnd(content, j);
        return { name, dynamic: true, start: i, end, expression: content.slice(j, end) };
      }
      const valueStart = j + 1;
      const valueEnd = skipString(content, j) - 1;
      let end = valueEnd + 1;
      while (/[ \t]/.test(content[end] || '')) end += 1;
      if (content[end] === ',') end += 1;
      return { name, dynamic: false, start: i, end, value: content.slice(valueStart, valueEnd) };
    }

    const ignoredEnd = skipIgnored(content, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    if (content[i] === '{' || content[i] === '(' || content[i] === '[') {
      depth += 1;
      continue;
    }
    if (content[i] === '}' || content[i] === ')' || content[i] === ']') {
      depth = Math.max(0, depth - 1);
      continue;
    }
    if (depth !== 0 || isWord(content[i - 1])) continue;
    const match = /^(?:r#)?[A-Za-z_][A-Za-z0-9_-]*/.exec(content.slice(i));
    if (!match || !wanted.has(match[0])) continue;
    let j = i + match[0].length;
    while (/\s/.test(content[j] || '')) j += 1;
    if (content[j] !== ':' && content[j] !== '=') continue;
    j += 1;
    while (/\s/.test(content[j] || '')) j += 1;
    if (content[j] !== '"') {
      const end = readAttrEnd(content, j);
      return { name: match[0], dynamic: true, start: i, end, expression: content.slice(j, end) };
    }
    const valueStart = j + 1;
    const valueEnd = skipString(content, j) - 1;
    let end = valueEnd + 1;
    while (/[ \t]/.test(content[end] || '')) end += 1;
    if (content[end] === ',') end += 1;
    return { name: match[0], dynamic: false, start: i, end, value: content.slice(valueStart, valueEnd) };
  }
  return null;
}

function findStaticAttr(content, names) {
  const attr = findTopLevelAttr(content, names);
  return attr && !attr.dynamic ? attr : null;
}

function removeTopLevelAttrs(content, attrs) {
  return removeRanges(content, attrs.filter(Boolean));
}

function rustString(value) {
  return JSON.stringify(value);
}

function mapOverlayPlacement(value, enumName) {
  const normalized = value.trim().toLowerCase();
  const variants = new Map([
    ['auto', 'Auto'],
    ['top', 'Top'],
    ['bottom', 'Bottom'],
    ['left', 'Start'],
    ['start', 'Start'],
    ['right', 'End'],
    ['end', 'End'],
  ]);
  const variant = variants.get(normalized);
  return variant ? `${enumName}::${variant}` : null;
}

function mapOverlayTrigger(value, typeName) {
  const tokens = new Set(value.trim().toLowerCase().split(/\s+/).filter(Boolean));
  const hover = tokens.has('hover');
  const focus = tokens.has('focus');
  const click = tokens.has('click');
  const manual = tokens.has('manual');
  if (manual && tokens.size === 1) return `${typeName}::MANUAL`;
  if (hover && focus && !click && tokens.size === 2) return `${typeName}::HOVER_FOCUS`;
  if (hover && !focus && !click && tokens.size === 1) return `${typeName}::HOVER`;
  if (!hover && focus && !click && tokens.size === 1) return `${typeName}::FOCUS`;
  if (!hover && !focus && click && tokens.size === 1) return `${typeName}::CLICK`;
  if (!manual && (hover || focus || click) && tokens.size === Number(hover) + Number(focus) + Number(click)) {
    return `${typeName} { hover: ${hover}, focus: ${focus}, click: ${click} }`;
  }
  return null;
}

function trueString(value) {
  return value.trim().toLowerCase() === 'true';
}

function quoteTopLevelAttr(content, name) {
  const ranges = [];
  let depth = 0;
  for (let i = 0; i < content.length; i += 1) {
    const ignoredEnd = skipIgnored(content, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    if (content[i] === '{' || content[i] === '(' || content[i] === '[') {
      depth += 1;
      continue;
    }
    if (content[i] === '}' || content[i] === ')' || content[i] === ']') {
      depth = Math.max(0, depth - 1);
      continue;
    }
    if (depth !== 0 || !content.startsWith(name, i) || isWord(content[i - 1]) || isWord(content[i + name.length])) {
      continue;
    }
    let j = i + name.length;
    while (/\s/.test(content[j] || '')) j += 1;
    if (content[j] === ':' || content[j] === '=') {
      ranges.push({ start: i, end: i + name.length, replacement: `"${name}"` });
    }
  }
  return removeRanges(content, ranges);
}

function normalizeBlock(content) {
  const trimmed = content.trim();
  if (!trimmed) return '';
  const lines = trimmed.split('\n');
  const firstIndent = (/^[ \t]*/.exec(lines[0]) || [''])[0].length;
  if (firstIndent === 0 && lines.length > 1) {
    const restIndents = lines
      .slice(1)
      .filter((line) => line.trim())
      .map((line) => (/^[ \t]*/.exec(line) || [''])[0].length);
    const restMin = restIndents.length ? Math.min(...restIndents) : 0;
    if (restMin > 0) {
      return [lines[0], ...lines.slice(1).map((line) => line.slice(Math.min(restMin, line.length)))].join('\n');
    }
  }
  const indents = lines
    .filter((line) => line.trim())
    .map((line) => (/^[ \t]*/.exec(line) || [''])[0].length);
  const min = indents.length ? Math.min(...indents) : 0;
  return lines.map((line) => line.slice(Math.min(min, line.length))).join('\n');
}

function lineIndent(source, index) {
  const lineStart = source.lastIndexOf('\n', index) + 1;
  const match = /^[ \t]*/.exec(source.slice(lineStart, index));
  return match ? match[0] : '';
}

function residual(tokens, consumed) {
  return tokens.filter((t) => !consumed.has(t));
}

function tokenSet(classValue) {
  return classValue.split(/\s+/).filter(Boolean);
}

function isBootstrapComponentToken(token) {
  return DYNAMIC_COMPONENT_EXACT.has(token) || DYNAMIC_COMPONENT_PREFIXES.some((prefix) => token.startsWith(prefix));
}

function dynamicClassHasComponentToken(expression) {
  for (let i = 0; i < expression.length; i += 1) {
    const ignoredEnd = skipIgnored(expression, i);
    if (ignoredEnd === null || expression[i] !== '"') {
      if (ignoredEnd !== null) i = ignoredEnd - 1;
      continue;
    }
    const valueEnd = skipString(expression, i) - 1;
    const value = expression.slice(i + 1, valueEnd);
    if (tokenSet(value).some(isBootstrapComponentToken)) return true;
    i = valueEnd;
  }
  return false;
}

function hasTopLevelAttrs(content) {
  let depth = 0;
  for (let i = 0; i < content.length; i += 1) {
    const ignoredEnd = skipIgnored(content, i);
    if (ignoredEnd !== null) {
      i = ignoredEnd - 1;
      continue;
    }
    if (content[i] === '{' || content[i] === '(' || content[i] === '[') {
      depth += 1;
      continue;
    }
    if (content[i] === '}' || content[i] === ')' || content[i] === ']') {
      depth = Math.max(0, depth - 1);
      continue;
    }
    if (depth !== 0 || isWord(content[i - 1])) continue;
    const match = /^(?:r#)?[A-Za-z_][A-Za-z0-9_-]*/.exec(content.slice(i));
    if (!match) continue;
    let j = i + match[0].length;
    while (/\s/.test(content[j] || '')) j += 1;
    if (content[j] === ':' || content[j] === '=') return true;
  }
  return false;
}

function mapButton(tag, tokens) {
  if (tag !== 'button' && tag !== 'a') return null;
  if (!tokens.includes('btn')) return null;
  const consumed = new Set(['btn']);
  const props = [];
  let hasVariant = false;

  for (const token of tokens) {
    if (token === 'btn-link') {
      props.push('link: true');
      consumed.add(token);
      hasVariant = true;
    } else if (token === 'active') {
      props.push('active: true');
      consumed.add(token);
    } else if (token === 'btn-sm' || token === 'btn-lg') {
      props.push(`size: ${sizeProp(token.slice(4))}`);
      consumed.add(token);
    } else if (token.startsWith('btn-outline-')) {
      const color = colorProp(token.slice('btn-outline-'.length));
      if (color) {
        props.push(`color: ${color}`);
        props.push('outline: true');
        consumed.add(token);
        hasVariant = true;
      }
    } else if (token.startsWith('btn-')) {
      const color = colorProp(token.slice(4));
      if (color) {
        props.push(`color: ${color}`);
        consumed.add(token);
        hasVariant = true;
      }
    }
  }

  if (!hasVariant) props.unshift('plain: true');
  return { component: 'Button', props, residual: residual(tokens, consumed) };
}

function mapBadge(tag, tokens) {
  if (tag !== 'span' || !tokens.includes('badge')) return null;
  const consumed = new Set(['badge']);
  const props = [];
  let hasColor = false;
  for (const token of tokens) {
    if (token === 'rounded-pill') {
      props.push('pill: true');
      consumed.add(token);
    } else if (token.startsWith('text-bg-')) {
      const color = colorProp(token.slice('text-bg-'.length));
      if (color) {
        props.push(`color: ${color}`);
        consumed.add(token);
        hasColor = true;
      }
    } else if (token.startsWith('bg-')) {
      const color = colorProp(token.slice(3));
      if (color) {
        props.push(`color: ${color}`);
        consumed.add(token);
        hasColor = true;
      }
    }
  }
  return hasColor ? { component: 'Badge', props, residual: residual(tokens, consumed) } : null;
}

function mapAlert(tag, tokens) {
  if (tag !== 'div' || !tokens.includes('alert')) return null;
  const consumed = new Set(['alert']);
  const props = [];
  let hasColor = false;
  let dismissible = false;
  for (const token of tokens) {
    if (token === 'alert-dismissible') {
      dismissible = true;
      props.push('dismissible: true');
      consumed.add(token);
    } else if (token.startsWith('alert-')) {
      const color = colorProp(token.slice('alert-'.length));
      if (color) {
        props.push(`color: ${color}`);
        consumed.add(token);
        hasColor = true;
      }
    }
  }
  if (dismissible) {
    consumed.add('fade');
    consumed.add('show');
  }
  return hasColor ? { component: 'Alert', props, residual: residual(tokens, consumed) } : null;
}

function mapSpinner(tag, tokens) {
  if (tag !== 'div' && tag !== 'span') return null;
  const base = tokens.includes('spinner-grow') ? 'spinner-grow' : tokens.includes('spinner-border') ? 'spinner-border' : null;
  if (!base) return null;
  const consumed = new Set([base]);
  const props = [];
  if (base === 'spinner-grow') props.push('style: SpinnerStyle::Grow');
  for (const token of tokens) {
    if (token === `${base}-sm`) {
      props.push('size: Size::Sm');
      consumed.add(token);
    } else if (token.startsWith('text-')) {
      const color = colorProp(token.slice(5));
      if (color) {
        props.push(`color: Some(${color})`);
        consumed.add(token);
      }
    }
  }
  return { component: 'Spinner', props, residual: residual(tokens, consumed) };
}

function mapForm(tag, tokens) {
  const base = tag === 'select' ? 'form-select' : tag === 'input' || tag === 'textarea' ? 'form-control' : null;
  if (!base || !tokens.includes(base)) return null;
  const consumed = new Set([base]);
  const props = [];
  for (const token of tokens) {
    if (token === `${base}-sm` || token === `${base}-lg`) {
      props.push(`size: ${sizeProp(token.slice(`${base}-`.length))}`);
      consumed.add(token);
    }
  }
  const component = tag === 'select' ? 'Select' : tag === 'textarea' ? 'Textarea' : 'Input';
  return { component, props, residual: residual(tokens, consumed) };
}

function mapTable(tag, tokens) {
  if (tag !== 'table' || !tokens.includes('table')) return null;
  const consumed = new Set(['table']);
  const props = [];
  const bools = new Map([
    ['table-striped', 'striped'],
    ['table-striped-columns', 'striped_columns'],
    ['table-hover', 'hover'],
    ['table-bordered', 'bordered'],
    ['table-borderless', 'borderless'],
    ['caption-top', 'caption_top'],
  ]);
  for (const token of tokens) {
    if (bools.has(token)) {
      props.push(`${bools.get(token)}: true`);
      consumed.add(token);
    } else if (token === 'table-sm') {
      props.push('size: Size::Sm');
      consumed.add(token);
    } else if (token.startsWith('table-')) {
      const color = colorProp(token.slice(6));
      if (color) {
        props.push(`color: Some(${color})`);
        consumed.add(token);
      }
    }
  }
  return { component: 'Table', props, residual: residual(tokens, consumed) };
}

function mapNavbarNav(tag, tokens) {
  if (tag !== 'ul' || !tokens.includes('navbar-nav')) return null;
  const consumed = new Set(['navbar-nav']);
  const props = [];
  if (tokens.includes('navbar-nav-scroll')) {
    props.push('scroll: true');
    consumed.add('navbar-nav-scroll');
  }
  return { component: 'NavbarNav', props, residual: residual(tokens, consumed) };
}

function mapNav(tag, tokens) {
  // `ul.nav` (pills/tabs/underline). `ul.navbar-nav` is a different component
  // (mapNavbarNav), so exclude it here.
  if (tag !== 'ul' || !tokens.includes('nav') || tokens.includes('navbar-nav')) return null;
  const consumed = new Set(['nav']);
  const props = [];
  const flag = (token, prop) => {
    if (tokens.includes(token)) {
      props.push(`${prop}: true`);
      consumed.add(token);
    }
  };
  flag('nav-pills', 'pills');
  flag('nav-tabs', 'tabs');
  flag('nav-underline', 'underline');
  flag('nav-fill', 'fill');
  flag('nav-justified', 'justified');
  flag('flex-column', 'vertical');
  return { component: 'Nav', props, residual: residual(tokens, consumed) };
}

function mapNavItem(tag, tokens) {
  if (tag !== 'li' || !tokens.includes('nav-item')) return null;
  return { component: 'NavItem', props: [], residual: residual(tokens, new Set(['nav-item'])) };
}

function mapNavLink(tag, tokens) {
  if (tag !== 'a' || !tokens.includes('nav-link')) return null;
  const consumed = new Set(['nav-link']);
  const props = [];
  if (tokens.includes('active')) {
    props.push('active: true');
    consumed.add('active');
  }
  if (tokens.includes('disabled')) {
    props.push('disabled: true');
    consumed.add('disabled');
  }
  return { component: 'NavLink', props, residual: residual(tokens, consumed) };
}

function mapCard(tag, tokens) {
  // `card` sits on a <div> normally, or on an <a> for the whole-card-as-link
  // pattern (mirrors mapButton's button/a). For the <a> form the element's
  // href/target survive as kept attributes and land on Card's href/target props.
  if ((tag !== 'div' && tag !== 'a') || !tokens.includes('card')) return null;
  const consumed = new Set(['card']);
  return { component: 'Card', props: [], residual: residual(tokens, consumed), card: true };
}

function unsupportedOverlayAttr(content, names, component) {
  const attr = findTopLevelAttr(content, names);
  if (!attr) return null;
  const attrName = attr.name;
  return `${component} uses ${attrName}; converter cannot safely map that Bootstrap option yet`;
}

function staticOverlayAttr(content, names, component) {
  const attr = findTopLevelAttr(content, names);
  if (!attr) return { attr: null, warning: null };
  if (attr.dynamic) {
    return {
      attr,
      warning: `${component} has dynamic ${attr.name}; converter cannot safely map Bootstrap intent`,
    };
  }
  return { attr, warning: null };
}

function mapTooltipDataElement(tag, content, toggleAttr) {
  const title = staticOverlayAttr(content, ['data-bs-title', 'title'], 'tooltip');
  if (title.warning) return { warning: title.warning };
  if (!title.attr) {
    return { warning: 'tooltip has no static title/data-bs-title; converter cannot safely map text' };
  }

  const removeAttrs = [toggleAttr, title.attr];
  for (const names of [
    ['data-bs-delay'],
    ['data-bs-html'],
    ['data-bs-offset'],
    ['data-bs-boundary'],
    ['data-bs-container'],
    ['data-bs-template'],
    ['data-bs-fallback-placements'],
  ]) {
    const unsupported = unsupportedOverlayAttr(content, names, 'tooltip');
    if (unsupported) {
      const attr = findStaticAttr(content, names);
      if (attr?.name === 'data-bs-html' && !trueString(attr.value)) {
        removeAttrs.push(attr);
        continue;
      }
      return { warning: unsupported };
    }
  }

  const props = [`text: ${rustString(title.attr.value)}`];

  const placement = staticOverlayAttr(content, ['data-bs-placement'], 'tooltip');
  if (placement.warning) return { warning: placement.warning };
  if (placement.attr) {
    const mapped = mapOverlayPlacement(placement.attr.value, 'TooltipPlacement');
    if (!mapped) return { warning: `tooltip has unsupported data-bs-placement "${placement.attr.value}"` };
    props.push(`placement: ${mapped}`);
    removeAttrs.push(placement.attr);
  }

  const trigger = staticOverlayAttr(content, ['data-bs-trigger'], 'tooltip');
  if (trigger.warning) return { warning: trigger.warning };
  if (trigger.attr) {
    const mapped = mapOverlayTrigger(trigger.attr.value, 'TooltipTriggers');
    if (!mapped) return { warning: `tooltip has unsupported data-bs-trigger "${trigger.attr.value}"` };
    props.push(`trigger: ${mapped}`);
    removeAttrs.push(trigger.attr);
  }

  const customClass = staticOverlayAttr(content, ['data-bs-custom-class'], 'tooltip');
  if (customClass.warning) return { warning: customClass.warning };
  if (customClass.attr) {
    props.push(`class: ${rustString(customClass.attr.value)}`);
    removeAttrs.push(customClass.attr);
  }

  return { component: 'Tooltip', props, residual: [], removeAttrs, wrap: true };
}

function mapPopoverDataElement(tag, content, toggleAttr) {
  const body = staticOverlayAttr(content, ['data-bs-content'], 'popover');
  if (body.warning) return { warning: body.warning };
  if (!body.attr) {
    return { warning: 'popover has no static data-bs-content; converter cannot safely map body' };
  }

  const removeAttrs = [toggleAttr, body.attr];
  for (const names of [
    ['data-bs-delay'],
    ['data-bs-html'],
    ['data-bs-offset'],
    ['data-bs-boundary'],
    ['data-bs-container'],
    ['data-bs-template'],
    ['data-bs-fallback-placements'],
    ['data-bs-sanitize'],
    ['data-bs-allow-list'],
    ['data-bs-sanitize-fn'],
  ]) {
    const unsupported = unsupportedOverlayAttr(content, names, 'popover');
    if (unsupported) {
      const attr = findStaticAttr(content, names);
      if ((attr?.name === 'data-bs-html' || attr?.name === 'data-bs-sanitize') && !trueString(attr.value)) {
        removeAttrs.push(attr);
        continue;
      }
      return { warning: unsupported };
    }
  }

  const props = [`body: rsx! { ${rustString(body.attr.value)} }`];

  const title = staticOverlayAttr(content, ['data-bs-title', 'title'], 'popover');
  if (title.warning) return { warning: title.warning };
  if (title.attr) {
    props.unshift(`title: ${rustString(title.attr.value)}`);
    removeAttrs.push(title.attr);
  }

  const placement = staticOverlayAttr(content, ['data-bs-placement'], 'popover');
  if (placement.warning) return { warning: placement.warning };
  if (placement.attr) {
    const mapped = mapOverlayPlacement(placement.attr.value, 'PopoverPlacement');
    if (!mapped) return { warning: `popover has unsupported data-bs-placement "${placement.attr.value}"` };
    props.push(`placement: ${mapped}`);
    removeAttrs.push(placement.attr);
  }

  const trigger = staticOverlayAttr(content, ['data-bs-trigger'], 'popover');
  if (trigger.warning) return { warning: trigger.warning };
  if (trigger.attr) {
    const mapped = mapOverlayTrigger(trigger.attr.value, 'PopoverTriggers');
    if (!mapped) return { warning: `popover has unsupported data-bs-trigger "${trigger.attr.value}"` };
    props.push(`trigger: ${mapped}`);
    removeAttrs.push(trigger.attr);
  }

  const customClass = staticOverlayAttr(content, ['data-bs-custom-class'], 'popover');
  if (customClass.warning) return { warning: customClass.warning };
  if (customClass.attr) {
    props.push(`class: ${rustString(customClass.attr.value)}`);
    removeAttrs.push(customClass.attr);
  }

  return { component: 'Popover', props, residual: [], removeAttrs, wrap: true };
}

function mapBootstrapDataElement(tag, content) {
  const spy = findTopLevelAttr(content, ['data-bs-spy']);
  if (spy) {
    if (spy.dynamic) {
      return { warning: `${tag} has dynamic data-bs-spy; converter cannot safely map Scrollspy intent` };
    }
    if (spy.value.trim().toLowerCase() === 'scroll') {
      return {
        warning:
          'scrollspy requires manual review: create a Signal<String>, add Scrollspy { target, root, active }, and remove data-bs-spy/data-bs-target attributes',
      };
    }
  }

  const toggle = findTopLevelAttr(content, ['data-bs-toggle']);
  if (!toggle) return null;
  if (toggle.dynamic) {
    return { warning: `${tag} has dynamic data-bs-toggle; converter cannot safely map Bootstrap intent` };
  }

  const value = toggle.value.trim().toLowerCase();
  if (value === 'tooltip') return mapTooltipDataElement(tag, content, toggle);
  if (value === 'popover') return mapPopoverDataElement(tag, content, toggle);
  return null;
}

function mapElement(tag, classValue) {
  const tokens = tokenSet(classValue);
  return (
    mapButton(tag, tokens) ||
    mapBadge(tag, tokens) ||
    mapAlert(tag, tokens) ||
    mapSpinner(tag, tokens) ||
    mapForm(tag, tokens) ||
    mapTable(tag, tokens) ||
    mapNavbarNav(tag, tokens) ||
    mapNav(tag, tokens) ||
    mapNavItem(tag, tokens) ||
    mapNavLink(tag, tokens) ||
    mapCard(tag, tokens)
  );
}

function classifyUnmapped(tag, classValue) {
  const tokens = tokenSet(classValue);
  if (tag === 'span' && tokens.includes('badge')) {
    return 'badge has no color class; Badge currently defaults to primary, so this needs review';
  }
  if (tag === 'div' && tokens.includes('alert')) {
    return 'alert has no color class; Alert currently defaults to primary, so this needs review';
  }
  return null;
}

function importsForMapping(mapping) {
  const imports = new Set([mapping.component]);
  for (const name of mapping.imports || []) imports.add(name);
  for (const prop of mapping.props) {
    if (prop.includes('Color::')) imports.add('Color');
    if (prop.includes('Size::')) imports.add('Size');
    if (prop.includes('SpinnerStyle::')) imports.add('SpinnerStyle');
    if (prop.includes('TooltipPlacement::')) imports.add('TooltipPlacement');
    if (prop.includes('TooltipTriggers')) imports.add('TooltipTriggers');
    if (prop.includes('PopoverPlacement::')) imports.add('PopoverPlacement');
    if (prop.includes('PopoverTriggers')) imports.add('PopoverTriggers');
  }
  if (mapping.childMapping) {
    for (const name of importsForMapping(mapping.childMapping)) imports.add(name);
  }
  return imports;
}

function mergePreludeImports(existing, required) {
  return [...new Set([...existing, ...required])].sort((a, b) => a.localeCompare(b));
}

function renderPreludeImport(names, multiline) {
  if (!multiline && names.length <= 6 && names.join(', ').length <= 96) {
    return `use dioxus_bootstrap_css::prelude::{${names.join(', ')}};`;
  }
  return `use dioxus_bootstrap_css::prelude::{\n${names.map((name) => `    ${name},`).join('\n')}\n};`;
}

function addPreludeImports(source, required) {
  if (!required.size || source.includes('use dioxus_bootstrap_css::prelude::*;')) return source;

  const grouped = /use\s+dioxus_bootstrap_css::prelude::\{([\s\S]*?)\};/m;
  const match = grouped.exec(source);
  if (match) {
    const existing = [...match[1].matchAll(/\b[A-Z][A-Za-z0-9_]*\b/g)].map((m) => m[0]);
    const names = mergePreludeImports(existing, required);
    const multiline = match[0].includes('\n');
    return `${source.slice(0, match.index)}${renderPreludeImport(names, multiline)}${source.slice(match.index + match[0].length)}`;
  }

  const importLine = renderPreludeImport([...required].sort((a, b) => a.localeCompare(b)), false);
  const dioxusPrelude = /use\s+dioxus::prelude::\*;\n?/m.exec(source);
  if (dioxusPrelude) {
    const insert = dioxusPrelude.index + dioxusPrelude[0].length;
    return `${source.slice(0, insert)}${importLine}\n${source.slice(insert)}`;
  }
  return `${importLine}\n${source}`;
}

function pushLines(lines, indent, block) {
  if (!block) return;
  for (const line of block.split('\n')) lines.push(`${indent}${line.trimEnd()}`);
}

function buildRsxProp(lines, propIndent, name, content) {
  if (!content) {
    lines.push(`${propIndent}${name}: rsx! {},`);
    return;
  }
  lines.push(`${propIndent}${name}: rsx! {`);
  pushLines(lines, `${propIndent}    `, content);
  lines.push(`${propIndent}},`);
}

function buildPlainElement(sourceBody, element, mapping, content) {
  const classAttr = findTopLevelClassAttr(content);
  let keptContent = removeClassAttr(content, classAttr);
  if (mapping.component === 'Spinner') keptContent = quoteTopLevelAttr(keptContent, 'style');
  const kept = normalizeBlock(keptContent);
  const indent = lineIndent(sourceBody, element.start);
  const propIndent = `${indent}    `;
  const props = [...mapping.props];
  if (mapping.residual.length) props.push(`class: "${mapping.residual.join(' ')}"`);

  if (!props.length && !kept) return `${mapping.component} {}`;

  const lines = [`${mapping.component} {`];
  for (const prop of props) lines.push(`${propIndent}${prop},`);
  pushLines(lines, propIndent, kept);
  lines.push(`${indent}}`);
  return lines.join('\n');
}

function buildWrappedElement(sourceBody, element, mapping, content, file, lineBase, warnings) {
  let childContent = removeTopLevelAttrs(content, mapping.removeAttrs || []);
  const classAttr = findTopLevelClassAttr(childContent);
  let child = null;

  if (classAttr?.dynamic && dynamicClassHasComponentToken(classAttr.expression)) {
    const loc = lineCol(sourceBody, element.start);
    warnings.push({
      kind: 'manual_review',
      file,
      line: lineBase + loc.line - 1,
      message: `${element.tag} has dynamic Bootstrap component class; converter cannot safely map Bootstrap intent`,
    });
  } else if (classAttr) {
    const childMapping = mapElement(element.tag, classAttr.value);
    if (childMapping && !childMapping.card) {
      mapping.childMapping = childMapping;
      child = buildPlainElement(sourceBody, element, childMapping, childContent);
    }
  }

  if (!child) child = rebuildWithContent(sourceBody, element, childContent);

  const indent = lineIndent(sourceBody, element.start);
  const propIndent = `${indent}    `;
  const lines = [`${mapping.component} {`];
  for (const prop of mapping.props) lines.push(`${propIndent}${prop},`);
  pushLines(lines, propIndent, normalizeBlock(child));
  lines.push(`${indent}}`);
  return lines.join('\n');
}

function slotFromElement(content, child) {
  if (child.tag !== 'div') return null;
  const childContent = content.slice(child.open + 1, child.close);
  const classAttr = findTopLevelClassAttr(childContent);
  if (!classAttr || classAttr.dynamic) return null;
  const tokens = tokenSet(classAttr.value);
  const slotClasses = tokens.filter((token) => SLOT_CLASSES.has(token));
  if (slotClasses.length === 0) return null;
  if (slotClasses.length > 1) {
    return { error: `card section has multiple slot classes: ${slotClasses.join(' ')}` };
  }
  const slot = SLOT_CLASSES.get(slotClasses[0]);
  const withoutClass = removeClassAttr(childContent, classAttr);
  if (hasTopLevelAttrs(withoutClass)) {
    return { error: `card ${slot} has attributes that Card cannot preserve` };
  }
  const consumed = new Set([slotClasses[0]]);
  return {
    slot,
    className: residual(tokens, consumed).join(' '),
    content: normalizeBlock(withoutClass),
  };
}

function buildCardElement(sourceBody, element, mapping, content, file, lineBase, warnings) {
  const classAttr = findTopLevelClassAttr(content);
  const children = collectElements(content);
  const slots = new Map();
  const remove = [{ start: classAttr.start, end: classAttr.end }];

  for (const child of children) {
    const slot = slotFromElement(content, child);
    if (!slot) continue;
    const loc = lineCol(sourceBody, element.start);
    if (slot.error) {
      warnings.push({
        kind: 'manual_review',
        file,
        line: lineBase + loc.line - 1,
        message: slot.error,
      });
      return null;
    }
    if (slots.has(slot.slot)) {
      warnings.push({
        kind: 'manual_review',
        file,
        line: lineBase + loc.line - 1,
        message: `card has multiple ${slot.slot} sections; converter will not guess how to merge them`,
      });
      return null;
    }
    slots.set(slot.slot, slot);
    remove.push({ start: child.start, end: child.end });
  }

  const kept = normalizeBlock(removeRanges(content, remove));
  const indent = lineIndent(sourceBody, element.start);
  const propIndent = `${indent}    `;
  const lines = ['Card {'];
  if (mapping.residual.length) lines.push(`${propIndent}class: "${mapping.residual.join(' ')}",`);

  for (const name of ['header', 'body', 'footer']) {
    const slot = slots.get(name);
    if (!slot) continue;
    if (slot.className) lines.push(`${propIndent}${name}_class: "${slot.className}",`);
    buildRsxProp(lines, propIndent, name, slot.content);
  }

  pushLines(lines, propIndent, kept);
  if (lines.length === 1) return 'Card {}';
  lines.push(`${indent}}`);
  return lines.join('\n');
}

function rebuildWithContent(sourceBody, element, content) {
  return `${sourceBody.slice(element.start, element.open + 1)}${content}${sourceBody.slice(element.close, element.end)}`;
}

function transformRsxBody(body, file, lineBase, warnings, imports) {
  const elements = collectElements(body).sort((a, b) => b.start - a.start);
  let out = body;

  for (const element of elements) {
    const content = out.slice(element.open + 1, element.close);
    const childLineBase = lineBase + lineCol(out, element.open + 1).line - 1;
    const transformedContent = transformRsxBody(content, file, childLineBase, warnings, imports);
    let rewritten = null;

    if (RAW_TAGS.has(element.tag)) {
      const dataMapping = mapBootstrapDataElement(element.tag, transformedContent);
      if (dataMapping?.warning) {
        const loc = lineCol(out, element.start);
        warnings.push({
          kind: 'manual_review',
          file,
          line: lineBase + loc.line - 1,
          message: dataMapping.warning,
        });
      } else if (dataMapping?.wrap) {
        rewritten = buildWrappedElement(out, element, dataMapping, transformedContent, file, lineBase, warnings);
        for (const name of importsForMapping(dataMapping)) imports.add(name);
      }

      if (!rewritten) {
        const classAttr = findTopLevelClassAttr(transformedContent);
        if (classAttr?.dynamic) {
          if (dynamicClassHasComponentToken(classAttr.expression)) {
            const loc = lineCol(out, element.start);
            warnings.push({
              kind: 'manual_review',
              file,
              line: lineBase + loc.line - 1,
              message: `${element.tag} has dynamic Bootstrap component class; converter cannot safely map Bootstrap intent`,
            });
          }
        } else if (classAttr) {
          const mapping = mapElement(element.tag, classAttr.value);
          if (mapping?.card) {
            rewritten = buildCardElement(out, element, mapping, transformedContent, file, lineBase, warnings);
            if (rewritten) for (const name of importsForMapping(mapping)) imports.add(name);
          } else if (mapping) {
            rewritten = buildPlainElement(out, element, mapping, transformedContent);
            for (const name of importsForMapping(mapping)) imports.add(name);
          } else {
            const message = classifyUnmapped(element.tag, classAttr.value);
            if (message) {
              const loc = lineCol(out, element.start);
              warnings.push({ kind: 'manual_review', file, line: lineBase + loc.line - 1, message });
            }
          }
      }
      }
    }

    if (!rewritten && transformedContent !== content) {
      rewritten = rebuildWithContent(out, element, transformedContent);
    }
    if (rewritten) {
      out = `${out.slice(0, element.start)}${rewritten}${out.slice(element.end)}`;
    }
  }
  return out;
}

export function transformSource(source, file = '<memory>') {
  const warnings = [];
  const imports = new Set();
  let out = source;
  const blocks = findRsxBlocks(source).sort((a, b) => b.open - a.open);
  for (const block of blocks) {
    const shift = source.length - out.length;
    const open = block.open - shift;
    const close = block.close - shift;
    const body = out.slice(open + 1, close);
    const lineBase = lineCol(out, open + 1).line;
    const next = transformRsxBody(body, file, lineBase, warnings, imports);
    out = `${out.slice(0, open + 1)}${next}${out.slice(close)}`;
  }
  out = addPreludeImports(out, imports);
  return { source: out, changed: out !== source, warnings };
}

function walkTargets(targets) {
  const files = [];
  function walk(p) {
    if (!fs.existsSync(p)) return;
    const st = fs.statSync(p);
    if (st.isDirectory()) {
      if (SKIP_DIRS.has(path.basename(p))) return;
      for (const entry of fs.readdirSync(p)) walk(path.join(p, entry));
    } else if (p.endsWith('.rs')) {
      files.push(p);
    }
  }
  for (const target of targets) walk(target);
  return files;
}

function parseArgs(argv) {
  const args = { write: false, check: false, json: false, targets: [] };
  for (const arg of argv) {
    if (arg === '--write') args.write = true;
    else if (arg === '--check') args.check = true;
    else if (arg === '--json') args.json = true;
    else args.targets.push(arg);
  }
  if (!args.targets.length) args.targets.push('examples');
  return args;
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  const files = walkTargets(args.targets);
  const results = [];
  for (const file of files) {
    const before = fs.readFileSync(file, 'utf8');
    const result = transformSource(before, file);
    if (args.write && result.changed) fs.writeFileSync(file, result.source);
    results.push({ file, changed: result.changed, warnings: result.warnings });
  }

  const changed = results.filter((r) => r.changed).map((r) => r.file);
  const warnings = results.flatMap((r) => r.warnings);
  if (args.json) {
    console.log(JSON.stringify({ changed, warnings }, null, 2));
  } else {
    for (const file of changed) console.log(`${args.write ? 'rewrote' : 'would rewrite'} ${file}`);
    for (const warning of warnings) {
      console.error(`${warning.file}:${warning.line}: ${warning.kind}: ${warning.message}`);
    }
    if (!changed.length && !warnings.length) {
      console.log(`OK no Bootstrap RSX migrations found in: ${args.targets.join(', ')}`);
    }
  }

  if (warnings.length) process.exit(2);
  if (args.check && changed.length) process.exit(1);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  main();
}
