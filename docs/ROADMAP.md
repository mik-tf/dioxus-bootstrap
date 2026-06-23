# Roadmap

## Current Status

The Bootstrap 5.3 component surface is implemented and covered by local tests,
doctests, migration tooling, the raw-Bootstrap gate, and Playwright e2e checks.

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

- `Navbar`, `NavbarToggler`, `NavbarCollapse`
- `Nav`, `NavItem`, `NavLink`
- `Breadcrumb`, `BreadcrumbItem`

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

## Maintenance Direction

- Keep Bootstrap parity in the crate API instead of asking downstream apps to
  keep raw Bootstrap component classes.
- Expand the converter when common static Bootstrap RSX can be mapped safely.
- Flag dynamic or ambiguous conversion cases for manual review.
- Keep the migration gate aligned with the component surface.
- Use screenshot tests for visual confidence whenever component output or
  migration behavior changes.

## Known Caveats

- Tooltip and popover positioning is CSS-based and relative to the trigger.
  Apps with viewport-edge requirements may need to choose placement explicitly.
- Scrollspy uses browser scroll observation through Dioxus document evaluation,
  so tests should allow the page to settle before asserting active sections.
