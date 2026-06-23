# Design

`dioxus-bootstrap-css` is a typed Dioxus layer over Bootstrap 5.3. It does not
rewrite Bootstrap, approximate Bootstrap, or invent a separate design system.

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
