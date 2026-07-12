# Bootstrap Dioxus Migration

This guide covers migrating raw Bootstrap HTML or RSX to
`dioxus-bootstrap-css` typed components.

The goal is not only to remove raw classes. The goal is visual and behavioral
equivalence: the same Bootstrap intent represented as typed Dioxus props,
signal-driven interactions, bundled assets, and screenshot-verified output.

## Reading the original: three source forms

Step one of any conversion is recovering what the original actually does. Bootstrap
behaviour reaches a page in one of three forms, and the third is the one naive ports
miss:

1. **Static markup** — `class="btn btn-primary"`, `class="card"`. Read the classes,
   map them to typed props. The converter handles most of this deterministically.
2. **Declarative behaviour** — `data-bs-toggle="popover"`, `data-bs-content="…"`. The
   attributes name the behaviour and its content; map them to the typed interactive
   component (`Popover`, `Modal`, …) plus a signal.
3. **Imperative JS / web components** — `new bootstrap.Popover(el, { placement: 'bottom',
   html: true, content: () => renderHTML() })`, often inside a custom element. Here the
   markup tells you almost nothing; the *JavaScript* is the source of truth. Read it to
   recover the Bootstrap options (placement, trigger, container) **and the exact HTML
   the script injects**, then reproduce both with the typed component.

If you convert only the markup of a form-3 control, it renders but behaves and often
looks wrong — the placement, the arrow, the injected body all lived in the JS. Always
identify the form first.

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
- `div.card` / `a.card` with `card-header` / `card-body` / `card-footer` slots -> `Card`
  (the `<a>` form is a whole-card link; its `href` / `target` map to `Card`'s props,
  exactly as `a.btn` maps to `Button { href }`)
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

## Starting from HTML or Askama templates

`migrate-bootstrap-rsx.mjs` converts *RSX*. If you are migrating a
server-rendered Bootstrap app whose source is plain HTML or Askama templates,
run the HTML front-end first — the conversion is a two-stage pipeline:

1. **`migrate-html-to-rsx.mjs`** — turns an `.html` template into a Dioxus
   `rsx!` block (tags, attributes, text, nesting, comments, and the common
   Askama control flow). It has **no Bootstrap opinion**; it just gets you from
   HTML into RSX. `class` strings are kept verbatim so the downstream converter
   can type them. Inline `on*=` handlers, `<script>`/`<style>` blocks, and
   Askama shapes with no RSX equivalent (`{% match %}`, `{% extends %}`,
   `{% include %}`, `{% macro %}`, a conditional in an attribute value) are
   **flagged** with a `// TODO(convert): …` comment plus a manual-review
   warning — never guessed.
2. **`migrate-bootstrap-rsx.mjs`** — takes that RSX (or hand-written RSX) and
   rewrites Bootstrap component classes into typed components, exactly as above.

```bash
node tools/migrate-html-to-rsx.mjs page.html -o page.rs   # stage 1: HTML -> RSX
node tools/migrate-bootstrap-rsx.mjs --write page.rs      # stage 2: RSX -> typed components
node tools/check-no-raw-bootstrap.mjs page.rs             # stage 3: gate
```

The front-end mirrors the RSX converter's flags: `--check` (parse only, exit 2
if anything is flagged), `--write` (write `<name>.rs` next to each `.html`), and
`--json` (machine-readable summary). `--pipe page.html` runs both stages in one
shot, emitting typed-dbcss RSX directly. As with the RSX converter, flagged
cases are for you to resolve deliberately — the gate and visual regression below
still apply to the result.

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
| `<a class="card" href=...>` (whole card is a link) | `Card { href, target?, header, body, footer }` |
| `alert alert-warning` | `Alert { color: Color::Warning }` |
| `badge text-bg-success` | `Badge { color: Color::Success }` |
| `spinner-border text-primary` | `Spinner { color: Some(Color::Primary) }` |
| `table table-striped table-hover` | `Table { striped: true, hover: true }` |

Layout and utility classes stay ordinary class strings unless a typed component
has a dedicated prop. Keep `container`, `row`, `col-*`, `d-flex`, spacing
utilities, text utilities, `form-label`, `card-title`, and similar classes where
they are pure layout or text styling.

## Overlays: reproducing Popper without Popper

Popover, Tooltip, and Dropdown are *overlays* — floating layers positioned relative
to a trigger. In stock Bootstrap, Popper.js does their placement: it flips them to a
side that fits, clamps them inside the viewport, and slides the arrow so it keeps
pointing at the trigger. This crate loads no JavaScript, so it reproduces that math
itself in `src/overlay.rs` (`calculate_overlay_position`). Popover and Tooltip call
it; the Dropdown is CSS-anchored instead (no arrow, no run-time measurement), so it is
immune to this whole class by construction.

Almost every "the port looks slightly off" bug on an overlay is a Popper-parity gap.
Fix it once in the shared overlay layer and every overlay inherits the fix. Three
worked examples, each surfaced by the gate above:

- **Arrow tracking.** When the box is clamped to a viewport edge, the arrow must
  offset along the box to keep pointing at the trigger. `calculate_overlay_position`
  returns the arrow's cross-axis centre; `Popover`/`Tooltip` apply it as an inline
  `position: absolute; left/top` on `.popover-arrow` / `.tooltip-arrow` — Bootstrap
  only makes the arrow absolutely positioned via Popper, so without this both the
  offset and Bootstrap's own edge rules are no-ops.
- **Trigger anchoring.** The overlay anchors to its trigger *wrapper*. An
  `inline-block` wrapper carries line-box leading, so its measured box extends below
  the trigger and the overlay lands low. The wrappers are `inline-flex`, which has no
  line box and hugs the trigger.
- **Dropdown spacer (tolerance).** Popper offsets a dropdown menu 2px off the toggle
  (`--bs-dropdown-spacer`); the CSS-only menu sits flush. A within-tolerance cosmetic
  delta, called out here so it stays a *known* intended difference, not a silent one.

When you hit a new overlay gap: reproduce it in a `*_parity` example, add the math to
`src/overlay.rs` with a unit test, apply it in the component, and re-run the gate.

## Workflow

1. Dump or inspect the existing DOM — and identify each control's source form
   (static markup / declarative `data-bs-*` / imperative JS), per "Reading the
   original" above.
2. If the source is HTML or Askama templates rather than RSX, run
   `migrate-html-to-rsx.mjs` first (see "Starting from HTML or Askama
   templates") so the rest of this workflow has RSX to work on.
3. Run the converter in dry-run mode.
4. Review warnings. Fix crate parity gaps upstream; handle manual-review cases
   intentionally.
5. Run the converter with `--write`.
6. Run `cargo check` or the consumer app's normal build.
7. Run the no-raw-Bootstrap gate:

```bash
node tools/check-no-raw-bootstrap.mjs path/to/app/src
```

8. Run the objective parity gate (above): capture golden + candidate yourself,
   compare geometry and `tools/visual-parity.mjs` AE, and classify every delta.

## The objective parity gate

Screenshot checks are the only layer that proves visual fidelity — but "look at the
two and decide" is not a gate, it is guessing, and it is slow. Make it objective:
capture both sides yourself and reduce the comparison to numbers.

**Never ask a human to be the differ.** Both the original and the converted control
are drivable in a headless browser; drive them.

### Procedure

1. **Capture the golden first**, from the original control, before or beside the
   conversion. Use a fixed viewport (e.g. 1440x900), a deterministic open-state
   (click / force `open`), and a **solid background** injected before the shot
   (`document.body.style.background = '#808080'`) so translucent chrome and wallpaper
   stop being pixel noise.
2. **Capture the candidate** from the converted control with identical steps. A tiny
   harness that pins the control at a fixed point and forces it open keeps the capture
   deterministic and isolates the control from the page — see `examples/popover_parity`
   and `examples/dropdown_parity`.
3. **Measure — structural first, pixels as a backstop:**
   - **Structural (primary) — `tools/structural-parity.mjs`.** Snapshot the control
     subtree on both sides (`--emit-js` prints the browser snapshot function; evaluate
     `(<fn>)('<root-selector>')` and save the JSON), then
     `node tools/structural-parity.mjs --a golden.json --b candidate.json
     [--ignore-text 'Last checked']`. It walks both trees in lockstep and prints
     **every** element whose computed style, geometry, text, or tree position differs.
     This is the gate that ends the eyeball loop: it is exhaustive (no crop to choose),
     deterministic (computed values, no anti-aliasing noise), and actionable (it hands
     you the exact property list to fix). Fix the list, re-run, repeat until empty.
   - **Pixels (backstop) — `tools/visual-parity.mjs`.** `--golden g.png --candidate
     c.png [--crop WxH+X+Y] [--fuzz 2%] [--threshold N]` runs ImageMagick
     `compare -metric AE`. Use it as a coarse final check (box moved, element missing,
     gross colour loss). Do NOT rely on it as the primary gate: a crop is
     crop-dependent and pixel diffs blur subtle deltas (grey-vs-dark text, one glyph vs
     another, a 2px border) into font anti-aliasing noise. The structural checker
     catches those; the pixel scorer does not.
4. **Classify every delta.** The gate is **not** "AE must be 0". A component swap has
   an irreducible few-pixel delta (font hinting, sub-pixel placement, a different
   Bootstrap build's anti-aliasing). The number exists to catch **gross** regressions
   — wrong size, lost colour, missing element, mis-placed arrow — and to force every
   real delta to be named *intended* or *regression* instead of drifting silently.
   Pick a `--threshold` from a known-good baseline pair.

Compare light and dark themes when the app supports them. Treat a deliberate visual
change as an explicit, documented baseline update — never a silent one.

## Definition Of Done

A migration is done when:

- the converter rewrites common static RSX cases deterministically.
- unsafe cases are reported with file and line, not silently transformed.
- the migrated app passes `cargo check` or its normal build.
- `tools/check-no-raw-bootstrap.mjs` exits clean.
- the objective parity gate confirms the converted control matches the original:
  geometry aligns and the `visual-parity.mjs` AE shows no gross regression, with
  every real delta classified intended vs regression.
- any gap that was the crate's (Bootstrap does it, the crate could not) was fixed in
  the crate with a test — not worked around in the app.

## AI Agent Prompt

Use this when asking an AI coding agent to migrate a page:

```text
Convert Bootstrap HTML/RSX to Dioxus RSX using dioxus-bootstrap-css 0.5.

Rules:
1. First identify each control's source form: static markup, declarative data-bs-*,
   or imperative JS / a web component. For the imperative form, read the JS to
   recover the Bootstrap options AND the exact HTML it injects — the markup alone
   is not enough.
2. Match the existing DOM structure unless a typed component requires a wrapper.
3. Replace Bootstrap JS behavior with Dioxus signals. Never keep data-bs-*
   attributes or Bootstrap JavaScript.
4. Map component intent to typed props; do not drop color, outline, size, href,
   target, state, or slot information. Preserve residual utility/layout classes.
5. Flag dynamic or ambiguous class strings and Bootstrap attributes instead of guessing.
6. Prove it objectively: capture the original (golden) and the converted (candidate)
   yourself in a headless browser at a fixed viewport with a solid background, then
   compare geometry (getBoundingClientRect) AND pixels (tools/visual-parity.mjs).
   Never ask a human to diff screenshots. Classify every delta intended vs regression.
7. If the crate cannot reproduce a Bootstrap behaviour, fix the crate and add a test.
   Never hand-patch the app.
```
