# Bootstrap Dioxus Migration

This guide covers migrating raw Bootstrap HTML or RSX to
`dioxus-bootstrap-css` typed components.

The goal is not only to remove raw classes. The goal is visual and behavioral
equivalence: the same Bootstrap intent represented as typed Dioxus props,
signal-driven interactions, bundled assets, and screenshot-verified output.

## Setup

```toml
[dependencies]
dioxus = { version = "0.7", features = ["web", "router"] }
dioxus-bootstrap-css = "0.5"
```

```rust
use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn app() -> Element {
    rsx! {
        BootstrapHead {}
        // your app
    }
}
```

`BootstrapHead {}` loads bundled Bootstrap CSS and Bootstrap Icons. When an
existing server-rendered app used a different Bootstrap build, point
`BootstrapHead` at local assets while migrating:

```rust
BootstrapHead {
    css: BootstrapCss::Url("/static/vendor/bootstrap/bootstrap.min.css".into()),
    icons: BootstrapIcons::Url("/static/vendor/bootstrap-icons/font/bootstrap-icons.css".into()),
}
```

Do not load Bootstrap from a CDN in the migrated app. The crate is designed for
offline-first bundled assets.

## Fidelity Rule

`tools/check-no-raw-bootstrap.mjs` is a completeness gate, not a
visual-fidelity proof.

It proves raw Bootstrap component classes, remote Bootstrap assets, and
Bootstrap JavaScript are gone. It cannot prove the replacement typed component
received equivalent props. A conversion can pass the gate and still look wrong
if it drops intent such as color, outline, size, href, state, or slot structure.

Use the three-layer migration system:

1. **Converter:** maps Bootstrap intent to typed component props.
2. **Gate:** rejects remaining raw Bootstrap, CDN assets, and Bootstrap JS.
3. **Visual regression:** verifies the converted UI still looks the same.

Lint-green is not the same as pixel-identical.

## Converter Rule

The converter maps Bootstrap intent to the crate API. It must preserve residual
utility classes, inline styles, event handlers, keys, children, and nested
layout while turning component classes into typed components.

The converter follows four rules:

1. **Bootstrap parity lives in the crate.** If Bootstrap supports component
   state, class, structure, size, color, wrapper, or interaction pattern,
   `dioxus-bootstrap-css` should expose a typed way to express it.
2. **Static intent is converted deterministically.** Common static RSX gets
   rewritten to typed components and props.
3. **Unsafe intent is flagged, not guessed.** Conditional or dynamic class
   strings, ambiguous nesting, and unsupported component shapes are reported for
   manual review.
4. **Limitations feed back upstream.** Classify every failed conversion as a
   crate parity gap, converter gap, or manual-review case. Do not leave
   downstream projects with permanent Bootstrap workarounds.

Run the converter in dry-run mode first:

```bash
node tools/migrate-bootstrap-rsx.mjs path/to/app/src
```

Check mode fails when safe rewrites are available:

```bash
node tools/migrate-bootstrap-rsx.mjs --check path/to/app/src
```

Write mode applies safe conversions:

```bash
node tools/migrate-bootstrap-rsx.mjs --write path/to/app/src
```

Supported static mappings include:

- `button.btn` / `a.btn` -> `Button`
- `div.card` with `card-header` / `card-body` / `card-footer` slots -> `Card`
- `span.badge` -> `Badge`
- `div.alert` -> `Alert`
- `div.spinner-*` / `span.spinner-*` -> `Spinner`
- `input.form-control` -> `Input` (including `list` for `<datalist>`
  autocomplete and `onfocus` / `onblur` focus handlers)
- `select.form-select` -> `Select`
- `textarea.form-control` -> `Textarea` (including `onfocus` / `onblur`)
- `table.table` -> `Table`
- static `data-bs-toggle="tooltip"` + `title`/`data-bs-title` -> `Tooltip`
- static `data-bs-toggle="popover"` + `data-bs-content` -> `Popover`

Scrollspy is intentionally manual review in the converter: raw Bootstrap
`data-bs-spy="scroll"` does not name the app-owned `Signal<String>` required by
`Scrollspy { target, root, active }`. Add the signal and typed `Scrollspy`
marker by hand, then remove `data-bs-spy`/`data-bs-target`.

Example:

```rust
// Raw Bootstrap
button {
    class: "btn btn-sm btn-outline-secondary border-0",
    onclick: move |_| refresh(),
    "Refresh"
}

// Converted
Button {
    size: Size::Sm,
    color: Color::Secondary,
    outline: true,
    class: "border-0",
    onclick: move |_| refresh(),
    "Refresh"
}
```

Bare neutral buttons are explicit:

```rust
button { class: "btn btn-sm", "Cancel" }

// becomes
Button { plain: true, size: Size::Sm, "Cancel" }
```

Dynamic component classes are manual-review cases:

```rust
button {
    class: if active { "btn btn-primary" } else { "btn btn-secondary" },
    "Save"
}
```

The converter should report this instead of guessing which typed props to emit.

## Component Mapping

### Interactive Components

Bootstrap JavaScript is not loaded. Replace JS attributes with Dioxus state.

| Bootstrap shape | Dioxus shape | State |
| --- | --- | --- |
| `data-bs-toggle="modal"` | `Modal { show, ... }` | `Signal<bool>` |
| dropdown + `data-bs-toggle` | `Dropdown { open, ... }` | `Signal<bool>` |
| `<button class="dropdown-item">` | `DropdownItem { .. }` | — |
| `<a class="dropdown-item" href=... target=...>` (link menu item) | `DropdownItem { href, target?, .. }` | — |
| `.collapse` | `Collapse { expanded, ... }` | `Signal<bool>` |
| navbar collapse | `NavbarToggler` + `NavbarCollapse` + `NavbarNav` | `Signal<bool>` |
| tabs | `TabList` / `Tabs` | `Signal<usize>` |
| `<a class="nav-link">` used for SPA/JS nav (must not follow `#`) | `NavLink { prevent_default: true, onclick, .. }` | app signal |
| `<button class="nav-link">` (JS-toggled nav item) | `NavButton { active, onclick, .. }` | app signal |
| accordion | `Accordion` / `AccordionItem` | `Signal<Option<usize>>` |
| offcanvas | `Offcanvas { show, ... }` | `Signal<bool>` |
| toast | `Toast { show, ... }` | `Signal<bool>` |
| tooltip + static `title` | `Tooltip { text, placement?, trigger?, children }` | owned by component or `open` |
| popover + static `data-bs-content` | `Popover { title?, body, placement?, trigger?, children }` | owned by component or `open` |
| `data-bs-spy="scroll"` | `Scrollspy { target, root, active }` | `Signal<String>` |

Never keep `data-bs-*`, `bootstrap.bundle.js`, or `new bootstrap.*` in a Dioxus
WASM app.

### CSS-Only Components

| Bootstrap shape | Dioxus shape |
| --- | --- |
| `btn btn-primary` | `Button { color: Color::Primary }` |
| `btn btn-outline-danger btn-sm` | `Button { color: Color::Danger, outline: true, size: Size::Sm }` |
| bare `btn` | `Button { plain: true }` |
| `card` + header/body/footer | `Card { header, body, footer }` |
| `alert alert-warning` | `Alert { color: Color::Warning }` |
| `badge text-bg-success` | `Badge { color: Color::Success }` |
| `spinner-border text-primary` | `Spinner { color: Some(Color::Primary) }` |
| `table table-striped table-hover` | `Table { striped: true, hover: true }` |

Layout and utility classes stay ordinary class strings unless a typed component
has a dedicated prop. Keep `container`, `row`, `col-*`, `d-flex`, spacing
utilities, text utilities, `form-label`, `card-title`, and similar classes where
they are pure layout or text styling.

## Workflow

1. Dump or inspect the existing DOM.
2. Run the converter in dry-run mode.
3. Review warnings. Fix crate parity gaps upstream; handle manual-review cases
   intentionally.
4. Run the converter with `--write`.
5. Run `cargo check` or the consumer app's normal build.
6. Run the no-raw-Bootstrap gate:

```bash
node tools/check-no-raw-bootstrap.mjs path/to/app/src
```

7. Run Playwright visual regression, or manually compare screenshots when no
   golden baseline exists yet.

## Visual Regression

Screenshot checks are the only layer that proves visual fidelity. They catch
wrong colors, missing outlines, changed spacing, font differences, and slot
structure mistakes that the converter and gate cannot prove.

Recommended checks:

- Wait for a stable DOM canary before taking screenshots.
- Compare light and dark themes when the app supports them.
- Use a small threshold for anti-aliasing noise.
- Treat deliberate visual changes as explicit test fixture updates.

## Definition Of Done

A migration is done when:

- the converter rewrites common static RSX cases deterministically.
- unsafe cases are reported with file and line, not silently transformed.
- the migrated app passes `cargo check` or its normal build.
- `tools/check-no-raw-bootstrap.mjs` exits clean.
- visual regression confirms typed output matches the raw Bootstrap baseline, or
  an intentional visual change is documented and accepted.

## AI Agent Prompt

Use this when asking an AI coding agent to migrate a page:

```text
Convert Bootstrap HTML/RSX to Dioxus RSX using dioxus-bootstrap-css 0.5.

Rules:
1. Match the existing DOM structure unless a typed component requires a wrapper.
2. Replace Bootstrap JS behavior with Dioxus signals.
3. Never keep data-bs-* attributes or Bootstrap JavaScript.
4. Map component intent to typed props; do not drop color, outline, size, href,
   target, state, or slot information.
5. Preserve residual utility classes, layout, and spacing.
6. Flag dynamic or ambiguous class strings and Bootstrap attributes instead of guessing.
7. Run the converter, no-raw-Bootstrap gate, cargo checks, and screenshot
   comparison before calling the migration done.
```
