# Getting Started

## Install

Add the crate with Cargo:

```sh
cargo add dioxus-bootstrap-css
```

Or pin it in `Cargo.toml` alongside Dioxus:

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web"] }
dioxus-bootstrap-css = "0.5"
```

## First Component

Bring the prelude into scope, mount `BootstrapHead` so the bundled Bootstrap CSS
and icons load, then compose typed components:

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

## Dark Mode

`ThemeProvider` sets `data-bs-theme` on the document and lets Bootstrap handle
the rest. Pair it with `ThemeToggle` for a switch:

```rust
let theme = use_signal(|| Theme::Dark);

rsx! {
    ThemeProvider { theme }
    BootstrapHead {}
    ThemeToggle { theme }
}
```

## Offline Assets

`BootstrapHead` bundles Bootstrap CSS and Bootstrap Icons into the app, so no
CDN request is made at runtime. This keeps applications offline-first and avoids
remote asset dependencies — the migration gate (see the Migration chapter)
enforces that CDN links do not creep back in.

## Examples

- `examples/showcase` demonstrates the full component set and is deployed as the
  live showcase.
- `examples/dashboard` is a compact admin dashboard example.

Run an example locally with the Dioxus CLI (`dx serve`) from its crate
directory, or browse the hosted showcase linked in the Introduction.

## Next Steps

- Read **Design** for the parity contract and the full component surface.
- Read **Migration** if you are converting an existing Bootstrap or raw-RSX app.
