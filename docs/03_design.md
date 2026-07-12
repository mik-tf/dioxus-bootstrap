# Design

`dioxus-bootstrap-css` is a typed Dioxus layer over Bootstrap 5.3. It does not
rewrite Bootstrap, approximate Bootstrap, or invent a separate design system.

## Current Status

The Bootstrap 5.3 component surface is implemented and covered by local tests,
doctests, migration tooling, the raw-Bootstrap gate, and Playwright e2e checks.

## Parity Contract

If Bootstrap 5.3 supports a component state, class, structure, size, color,
wrapper, or interaction pattern, this crate should expose a typed Dioxus way to
express it.

If Bootstrap does not define the behavior, it belongs in the consuming
application, not in this crate.

## What Belongs In The Crate

- Type-safe RSX wrappers for Bootstrap components.
- Signal-driven replacements for Bootstrap JavaScript behavior.
- Bundled Bootstrap CSS and Bootstrap Icons for offline-first apps.
- Minimal escape hatches such as `class` and forwarded attributes for Bootstrap
  utilities and ordinary HTML behavior.

## What Does Not Belong In The Crate

- App-specific page layout, branding, gradients, or scroll offsets.
- Custom CSS beyond what Bootstrap provides.
- Opinionated defaults that Bootstrap does not define.
- Extra components that are not Bootstrap components.

## Rendering Model

The crate emits ordinary Bootstrap HTML structure and class names. Bootstrap CSS
does the styling. Dioxus signals replace Bootstrap JavaScript state machines for
interactive components such as modals, dropdowns, tabs, collapse, offcanvas,
toast, carousel, tooltip, popover, and scrollspy.

Every component follows the same pattern:

1. Props represent Bootstrap component intent.
2. Residual utility classes pass through with `class`.
3. Interactive state is explicit Dioxus state.
4. Output remains standard Bootstrap HTML.

Bootstrap's JavaScript conveniences are reproduced as typed props on the same
components: Modal and Offcanvas close on the Escape key and backdrop click
(`keyboard_close`, `backdrop_close`), Offcanvas exposes an `on_dismiss`
callback, and Toast supports `autohide` with a `delay_ms`. See the
Troubleshooting chapter for the defaults.

## Component Surface

Layout and head:

- `BootstrapHead`
- `ThemeProvider`, `ThemeToggle`
- `Container`, `Row`, `Col`

Content and data display:

- `Button`, `ButtonGroup`, `ButtonToolbar`
- `Card`, `Alert`, `Badge`, `Icon`, `Spinner`
- `Progress`, `ProgressBar`, `Placeholder`, `PlaceholderParagraph`
- `Figure`, `Ratio`, `Table`, `ListGroup`, `ListGroupItem`, `Pagination`

Forms:

- `FormGroup`, `Input`, `Select`, `Textarea`
- `Checkbox`, `Radio`, `Switch`, `Range`
- `FloatingLabel`, `InputGroup`, `InputGroupText`, `FormFeedback`, `FormText`

Interactive components:

- `Modal`, `Dropdown`, `Collapse`, `Tabs`, `Tab`, `TabList`
- `Accordion`, `AccordionItem`, `Offcanvas`, `Toast`, `ToastContainer`
- `Carousel`, `Tooltip`, `Popover`, `Scrollspy`

Navigation:

- `Navbar`, `NavbarToggler`, `NavbarCollapse`, `NavbarNav`
- `Nav`, `NavItem`, `NavLink`, `NavButton`
- `Breadcrumb`, `BreadcrumbItem`

## Migration Quality Bar

Migration is not complete merely because raw Bootstrap classes are gone. The
typed component must preserve the original Bootstrap intent.

The migration bar is:

- Convert safe static Bootstrap intent to typed props.
- Flag dynamic or ambiguous class strings instead of guessing.
- Reject raw Bootstrap component classes, CDN assets, and Bootstrap JavaScript
  with `tools/check-no-raw-bootstrap.mjs`.
- Prove visual fidelity with Playwright screenshots or explicit screenshot
  review.

This keeps the crate honest: parity gaps are fixed in the crate or converter,
not hidden as downstream Bootstrap workarounds.

## Maintenance Direction

- Keep Bootstrap parity in the crate API instead of asking downstream apps to
  keep raw Bootstrap component classes.
- Expand the converter when common static Bootstrap RSX can be mapped safely.
- Flag dynamic or ambiguous conversion cases for manual review.
- Keep the migration gate aligned with the component surface.
- Use screenshot tests for visual confidence whenever component output or
  migration behavior changes.

- Track new Bootstrap parity gaps as focused Forge issues with acceptance
  criteria, then update crate API, converter, tests, and docs together.

## Quality Gates

Changes should keep these checks green:

```bash
cargo +1.96 fmt --all -- --check
cargo +1.96 clippy --target wasm32-unknown-unknown -p dioxus-bootstrap-css -- -D warnings
cargo +1.96 clippy --target wasm32-unknown-unknown -p showcase -- -D warnings
cargo +1.96 check --target wasm32-unknown-unknown -p dioxus-bootstrap-css
cargo +1.96 check --target wasm32-unknown-unknown -p showcase
cargo +1.96 test -p dioxus-bootstrap-css
npm run test:migrate
npm run lint:bootstrap
npm run test:e2e -- --reporter=list
```

## Implementation Notes

- Tooltip and Popover use crate-owned viewport-aware positioning and render
  Bootstrap-compatible markup/classes without Bootstrap JavaScript or Popper.js.
- Scrollspy uses browser IntersectionObserver, MutationObserver, and scroll
  listeners through Dioxus document evaluation. As with Bootstrap's observer
  model, tests should allow the page to settle before asserting active sections.
