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
