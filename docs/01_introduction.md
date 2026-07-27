# Introduction

`dioxus-bootstrap-css` (`dbcss`) is a typed Dioxus layer over Bootstrap 5.3. It
lets applications express Bootstrap UI as type-safe Rust components instead of
raw class strings, while emitting ordinary Bootstrap HTML and using real
Bootstrap CSS for the styling.

The crate is developed on GitHub and published to crates.io.

- **Crate:** https://crates.io/crates/dioxus-bootstrap-css
- **API docs:** https://docs.rs/dioxus-bootstrap-css
- **Live showcase:** https://mik-tf.github.io/dioxus-bootstrap-css/
- **Repository:** https://github.com/mik-tf/dioxus-bootstrap-css

## Why dbcss

- **Pixel fidelity** by using Bootstrap 5.3 CSS directly instead of a
  reimplementation.
- **Zero Bootstrap JavaScript** — interactive components are driven by Dioxus
  signals, not Bootstrap's JS state machines.
- **Offline-first** CSS and icon assets bundled through `BootstrapHead`.
- **Type-safe props** for color, size, state, slots, and variants.
- **Migration tooling** that converts raw Bootstrap RSX into typed components
  and gates against raw Bootstrap classes reappearing.

## Design Rule

> If Bootstrap does it, `dbcss` should expose a typed Dioxus way to express it.
> If Bootstrap does not do it, `dbcss` should not invent it.

Parity gaps are fixed in the crate API, converter, migration gate, examples, or
documentation — never normalized as downstream raw Bootstrap workarounds.

## How This Booklet Is Organized

- **Getting Started** — install the crate and render your first typed component.
- **Design** — the parity contract, rendering model, and component surface.
- **Migration** — converting Bootstrap HTML and raw RSX to typed components.
- **Visual parity and overlays** — proving the conversion looks right, and the
  overlay positioning math behind tooltips, popovers, and dropdowns.
- **Migrating with an AI agent** — handing the conversion to a coding agent.
- **Governance** — where development happens and maintainer expectations.
- **Release** — the release checklist.
- **Adoption** — what has been assessed from downstream forks, adopted or declined.
- **Troubleshooting** — common issues and how to resolve them.

Throughout, the short name `dbcss` stands for `dioxus-bootstrap-css`; the full
crate name is long, so issues and notes often use the abbreviation.
