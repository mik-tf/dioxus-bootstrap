# Troubleshooting

## Bootstrap styles do not apply

Make sure `BootstrapHead {}` is mounted once near the top of the app. It injects
the bundled Bootstrap CSS and Bootstrap Icons; without it, components render the
correct HTML and class names but no stylesheet is present to style them.

## Dark mode does not switch

Dark mode is driven by `data-bs-theme`, which `ThemeProvider` sets on the
document. Confirm a single `ThemeProvider { theme }` wraps the app and that the
`theme` signal is the one being toggled. Two providers fighting over the same
attribute will produce inconsistent results.

## Interactive component does not open or close

Interactive components (`Modal`, `Dropdown`, `Collapse`, `Tabs`, `Offcanvas`,
`Toast`, `Carousel`, `Tooltip`, `Popover`, `Scrollspy`) are driven by Dioxus
signals, not Bootstrap JavaScript. Do **not** add `bootstrap.bundle.js` or
`data-bs-*` toggle attributes — they are unnecessary and the migration gate
rejects them. Drive open/close state through the component's props and signals.

## A modal, offcanvas, or toast closes (or does not close) on its own

These built-in dismissals are typed props, matching Bootstrap's options:

- `Modal { keyboard_close, backdrop_close }` — close on the Escape key and on a
  backdrop click. Both default to `true`; set either to `false` to keep the
  modal open in that case.
- `Offcanvas { keyboard_close, on_dismiss }` — Escape closes by default;
  `on_dismiss` fires whenever the offcanvas is dismissed (close button, backdrop,
  or Escape), the typed equivalent of `hidden.bs.offcanvas`.
- `Toast { autohide, delay_ms }` — `autohide` is `false` by default; set it to
  `true` to auto-dismiss after `delay_ms` (default `5000`).

If a modal disappears when you press Escape and you did not want that, set
`keyboard_close: false`. If a toast vanishes unexpectedly, check whether
`autohide` is on. All of these drive the same `show`/state signal you own, so the
signal stays the single source of truth.

## The migration gate reports raw Bootstrap classes

`tools/check-no-raw-bootstrap.mjs` fails when consumer code reintroduces raw
Bootstrap component classes, remote CDN assets, or Bootstrap JavaScript. Replace
the flagged class with the matching typed component (see the Migration chapter),
drop CDN links in favor of `BootstrapHead`, and drive interactive state with
Dioxus signals. The gate is a completeness check, not a visual-fidelity proof —
confirm the rendered result with the showcase or Playwright screenshots.

## Scrollspy or tooltip positioning seems off in tests

Scrollspy uses browser `IntersectionObserver`, `MutationObserver`, and scroll
listeners; tooltips and popovers use crate-owned viewport-aware positioning. In
tests, allow the page to settle before asserting active sections or positions,
just as Bootstrap's observer model requires.

## A Bootstrap feature has no typed component

Per the design rule, that is a parity gap to fix in the crate, not to work
around downstream. Open a focused Forge issue with acceptance criteria on the
primary repository so the crate API, converter, tests, and docs can be updated
together.
