// Bootstrap-to-dioxus-bootstrap-css parity data used by migration tooling.
//
// Rule: if Bootstrap supports a component shape, the crate should expose it
// typed. The converter consumes this map; it must not invent downstream
// workarounds for crate gaps.

export const COLORS = new Map([
  ['primary', 'Primary'],
  ['secondary', 'Secondary'],
  ['success', 'Success'],
  ['danger', 'Danger'],
  ['warning', 'Warning'],
  ['info', 'Info'],
  ['light', 'Light'],
  ['dark', 'Dark'],
]);

export const SIZES = new Map([
  ['sm', 'Sm'],
  ['lg', 'Lg'],
]);

export function colorProp(name) {
  const color = COLORS.get(name);
  return color ? `Color::${color}` : null;
}

export function sizeProp(name) {
  const size = SIZES.get(name);
  return size ? `Size::${size}` : null;
}

export const COMPONENTS = {
  Button: {
    tags: ['button', 'a'],
    requiredClass: 'btn',
    maps: ['btn-{color}', 'btn-outline-{color}', 'btn-sm', 'btn-lg', 'btn-link', 'active'],
  },
  Badge: {
    tags: ['span'],
    requiredClass: 'badge',
    maps: ['text-bg-{color}', 'bg-{color}', 'rounded-pill'],
  },
  Alert: {
    tags: ['div'],
    requiredClass: 'alert',
    maps: ['alert-{color}', 'alert-dismissible', 'fade', 'show'],
  },
  Card: {
    tags: ['div'],
    requiredClass: 'card',
    maps: ['card-header', 'card-body', 'card-footer'],
  },
  Spinner: {
    tags: ['div', 'span'],
    requiredClassAny: ['spinner-border', 'spinner-grow'],
    maps: ['spinner-border', 'spinner-grow', 'spinner-border-sm', 'spinner-grow-sm', 'text-{color}'],
  },
  Input: {
    tags: ['input'],
    requiredClass: 'form-control',
    maps: ['form-control-sm', 'form-control-lg'],
  },
  Select: {
    tags: ['select'],
    requiredClass: 'form-select',
    maps: ['form-select-sm', 'form-select-lg'],
  },
  Textarea: {
    tags: ['textarea'],
    requiredClass: 'form-control',
    maps: ['form-control-sm', 'form-control-lg'],
  },
  Table: {
    tags: ['table'],
    requiredClass: 'table',
    maps: [
      'table-striped',
      'table-striped-columns',
      'table-hover',
      'table-bordered',
      'table-borderless',
      'table-sm',
      'caption-top',
      'table-{color}',
    ],
  },
};
