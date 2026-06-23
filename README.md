# dioxus-bootstrap-css

[![Crates.io](https://img.shields.io/crates/v/dioxus-bootstrap-css.svg)](https://crates.io/crates/dioxus-bootstrap-css)
[![License](https://img.shields.io/crates/l/dioxus-bootstrap-css.svg)](LICENSE)
[![CI](https://github.com/mik-tf/dioxus-bootstrap-css/actions/workflows/ci.yml/badge.svg)](https://github.com/mik-tf/dioxus-bootstrap-css/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/dioxus-bootstrap-css)](https://docs.rs/dioxus-bootstrap-css)

Bootstrap 5.3 components for Dioxus. The crate uses real Bootstrap CSS and
Bootstrap Icons, bundles assets for offline-first apps, and replaces Bootstrap
JavaScript behavior with Dioxus signals.

**Live showcase:** https://mik-tf.github.io/dioxus-bootstrap-css/

**API docs:** https://docs.rs/dioxus-bootstrap-css

**Primary repository:** https://forge.ourworld.tf/lhumina_code/dioxus-bootstrap-css

**GitHub mirror:** https://github.com/mik-tf/dioxus-bootstrap-css

Short name: `dbcss` means `dioxus-bootstrap-css`. The full crate name is long,
so issues and notes may use the abbreviation.

> Design rule: if Bootstrap does it, this crate should expose a typed Dioxus
> way to express it. If Bootstrap does not do it, this crate should not invent
> it.

## Why

- Pixel fidelity by using Bootstrap 5.3 CSS instead of a reimplementation.
- Zero Bootstrap JavaScript; interactions are driven by Dioxus state.
- Offline-first CSS and icon assets through `BootstrapHead`.
- Type-safe component props for color, size, state, slots, and variants.
- Migration tools for converting raw Bootstrap RSX into typed components.

## Quick Start

```sh
cargo add dioxus-bootstrap-css
```

Or pin it in `Cargo.toml`:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-bootstrap-css = "0.5"
```

```rust
use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let theme = use_signal(|| Theme::Dark);

    rsx! {
        ThemeProvider { theme }
        BootstrapHead {}
        Container { class: "py-4",
            Row { class: "g-3",
                Col { md: ColumnSize::Span(6),
                    Card {
                        header: rsx! { "Getting Started" },
                        body: rsx! {
                            p { "Bootstrap in Dioxus - fully offline, fully Rust." }
                            Button { color: Color::Primary, "Launch" }
                        },
                    }
                }
                Col { md: ColumnSize::Span(6),
                    Alert { color: Color::Success, "Everything works out of the box." }
                }
            }
        }
    }
}
```

## Components

Layout and head:

- `BootstrapHead`
- `ThemeProvider`, `ThemeToggle`
- `Container`, `Row`, `Col`

Content and data display:

- `Button`, `ButtonGroup`, `ButtonToolbar`
- `Card`, `Alert`, `Badge`, `Icon`, `Spinner`
- `Progress`, `ProgressBar`, `Placeholder`, `Figure`, `Ratio`
- `Table`, `ListGroup`, `Pagination`

Forms:

- `FormGroup`, `Input`, `Select`, `Textarea`
- `Checkbox`, `Radio`, `Switch`, `Range`
- `FloatingLabel`, `InputGroup`, `FormFeedback`, `FormText`

Interactive components:

- `Modal`, `Dropdown`, `Collapse`, `Tabs`, `TabList`
- `Accordion`, `Offcanvas`, `Toast`, `Carousel`
- `Tooltip`, `Popover`, `Scrollspy`
- `Navbar`, `NavbarToggler`, `NavbarCollapse`, `NavbarNav`, `Nav`, `Breadcrumb`

## Dark Mode

```rust
let theme = use_signal(|| Theme::Dark);

rsx! {
    ThemeProvider { theme }
    BootstrapHead {}
    ThemeToggle { theme }
}
```

`ThemeProvider` sets `data-bs-theme` on the document and lets Bootstrap handle
the rest.

## Migration From Bootstrap HTML

Convert Bootstrap HTML or raw Bootstrap RSX by replacing component classes with
typed components:

```rust
Button {
    color: Color::Secondary,
    outline: true,
    size: Size::Sm,
    class: "border-0",
    "Refresh"
}
```

Key principles:

- Use Bootstrap utility classes (`small`, `py-2`, `mb-0`) for layout and spacing.
- Use component props (`color`, `outline`, `size`, `header_class`, `responsive`)
  for component intent.
- Replace Bootstrap JavaScript with Dioxus signals.
- Treat `tools/check-no-raw-bootstrap.mjs` as a completeness gate, not a
  visual-fidelity proof.
- Let the converter map Bootstrap intent to props, flag unsafe cases instead of
  guessing, then prove fidelity with Playwright screenshots.

See [docs/MIGRATION.md](docs/MIGRATION.md) for the complete
workflow.

## Tools

```bash
node tools/migrate-bootstrap-rsx.mjs path/to/app/src
node tools/migrate-bootstrap-rsx.mjs --check path/to/app/src
node tools/migrate-bootstrap-rsx.mjs --write path/to/app/src

node tools/check-no-raw-bootstrap.mjs path/to/app/src
npm run lint:bootstrap
```

[tools/README.md](tools/README.md) documents the converter and migration gate.

## Examples

- `examples/showcase` demonstrates the component set.
- `examples/dashboard` is a compact admin dashboard example.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Apache-2.0.
