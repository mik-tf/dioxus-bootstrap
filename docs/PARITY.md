# Bootstrap Parity Matrix

This document tracks the remaining Bootstrap 5.3 parity work for Tooltip,
Popover, and Scrollspy.

Parent tracker:
https://forge.ourworld.tf/lhumina_code/dioxus-bootstrap-css/issues/1

Official Bootstrap references:

- Tooltip: https://getbootstrap.com/docs/5.3/components/tooltips/
- Popover: https://getbootstrap.com/docs/5.3/components/popovers/
- Scrollspy: https://getbootstrap.com/docs/5.3/components/scrollspy/

## Rule

If Bootstrap does it, dbcss should expose a typed Dioxus way to express it. If
Bootstrap does not do it, dbcss should not invent it.

For JavaScript plugin behavior, dbcss should expose Dioxus state, props, and
callbacks instead of requiring `bootstrap.bundle.js`.

## Tracking Split

- #8 Overlay: add shared viewport-aware positioning core
- #9 Tooltip: implement Bootstrap trigger and positioning parity
- #10 Popover: implement Bootstrap trigger, dismiss, and positioning parity
- #11 Scrollspy: replace global eval with scoped observer parity
- #12 Migration/docs: finish overlay and scrollspy parity cleanup

Issue #7 is this parity spec.

## Shared Overlay API

Tooltip and Popover should share the same positioning foundation.

Proposed shared concepts:

- `OverlayPlacement`: `Auto`, `Top`, `Bottom`, `Start`, `End`
- `OverlayTrigger`: `Hover`, `Focus`, `Click`, `Manual`
- `OverlayDelay`: `show_ms`, `hide_ms`
- `OverlayOffset`: `skidding`, `distance`
- `OverlayBoundary`: viewport/body/selector-based boundary
- `OverlayContainer`: inline/body/selector-based render container

The exact names may change in #8, but the behavior should remain stable:

- measure trigger and overlay rectangles while visible
- choose the requested placement when it fits
- choose fallback placement when the requested placement would overflow
- keep the effective placement available to render Bootstrap orientation classes
- update positioning on scroll, resize, and relevant layout changes while open
- clean up listeners/observers when the overlay closes or unmounts

## Shared Overlay Out Of Scope

Bootstrap exposes several JavaScript plugin options that should not become raw
dbcss APIs:

- `popperConfig`: dbcss does not embed Popper or expose Popper config objects.
- `template`: dbcss owns the Bootstrap-compatible markup it renders.
- `selector`: delegated JavaScript plugin setup is not how Dioxus components are
  instantiated.
- `allowList`, `sanitize`, `sanitizeFn`: dbcss should prefer typed Dioxus
  `Element` or plain text APIs instead of accepting unsafe HTML strings.

Bootstrap plugin methods and events should map to Dioxus state and callbacks.
For example, manual show/hide/toggle should be represented through explicit
state/control props rather than runtime plugin instance methods.

## Tooltip Matrix

Current dbcss:

- `TooltipPlacement`: `Top`, `Bottom`, `Start`, `End`
- Props: `text`, `placement`, `class`, `children`
- Behavior: hover-only, CSS-positioned relative to the trigger wrapper

| Bootstrap behavior | dbcss target |
|---|---|
| default placement `top` | Keep default `Top`. |
| placement `auto`, `top`, `bottom`, `left`, `right` | Add `Auto`; map Bootstrap left/right to `Start`/`End` with RTL-aware class output where practical. |
| fallback placements | Add typed fallback placement list. |
| boundary / prevent overflow | Use shared overlay boundary logic. |
| container, especially body | Add typed container option without requiring Bootstrap JS. |
| offset default `[0, 6]` | Add typed offset with Bootstrap-compatible default. |
| trigger default `hover focus` | Add trigger set with default hover+focus. |
| click and manual triggers | Add typed trigger/control API. |
| delay show/hide | Add typed delay struct or millisecond pair. |
| custom class | Keep `class`, ensure it augments the tooltip element without losing Bootstrap classes. |
| zero-length title hidden | Empty `text` or empty rendered content should not show a tooltip. |
| HTML tooltip content | Prefer typed Dioxus content over an `html: bool` string API. Keep `text` as the safe simple path. |
| disabled elements need wrapper | Document wrapper guidance or add a small trigger wrapper helper. |
| accessibility | Preserve `role="tooltip"`, add stable `aria-describedby`, and support keyboard focus behavior. |
| plugin events/methods | Map to Dioxus state and optional callbacks after trigger/control API exists. |

Issue #9 owns Tooltip behavior after #8 lands.

## Popover Matrix

Current dbcss:

- `PopoverPlacement`: `Top`, `Bottom`, `Start`, `End`
- Props: `title`, `body`, `placement`, `class`, `children`
- Behavior: click toggles open, outside overlay closes, CSS-positioned relative
  to the trigger wrapper

| Bootstrap behavior | dbcss target |
|---|---|
| default placement `right` | Align default to `End` or document release migration if changed from current `Top`. |
| placement `auto`, `top`, `bottom`, `left`, `right` | Add `Auto`; map left/right to `Start`/`End`. |
| fallback placements | Add typed fallback placement list. |
| boundary / prevent overflow | Use shared overlay boundary logic. |
| container, especially body | Add typed container option without requiring Bootstrap JS. |
| offset default `[0, 8]` | Add typed offset with Bootstrap-compatible default. |
| trigger default `click` | Keep click default. |
| hover, focus, manual triggers | Add typed trigger/control API. |
| focus dismiss pattern | Support focus-dismiss behavior and document trigger requirements. |
| delay show/hide | Add typed delay struct or millisecond pair. |
| custom class | Keep `class`, ensure it augments the popover element without losing Bootstrap classes. |
| title/content empty handling | Preserve Bootstrap-compatible rendering of absent title/body sections. |
| HTML content | Already typed through `body: Element`; do not add unsafe HTML string API. |
| disabled elements need wrapper | Document wrapper guidance or add a small trigger wrapper helper. |
| accessibility | Preserve `role="tooltip"`, add stable `aria-describedby`, and document focus-order limits. |
| plugin events/methods | Map to Dioxus state and optional callbacks after trigger/control API exists. |

Issue #10 owns Popover behavior after #8 lands.

## Scrollspy Matrix

Current dbcss:

- Props: `target`, `active`, `offset`
- Behavior: document eval, document-wide `[id]` scan, shared
  `window.__dioxus_scrollspy_active`, scroll listener, signal update

Bootstrap 5.3 uses an IntersectionObserver-style model with `rootMargin`,
`threshold`, `target`, optional smooth scrolling, non-visible element handling,
and refresh.

| Bootstrap behavior | dbcss target |
|---|---|
| `target` points to nav/list/simple-anchor container | Add or correct typed `target` semantics to mean Bootstrap's target selector. |
| element being spied on is body or custom scroll container | Add explicit scroll container/root prop. |
| `rootMargin` default `0px 0px -25%` | Add `root_margin` prop. |
| `threshold` default `[0.1, 0.5, 1]` | Add typed threshold list. |
| deprecated `offset` compatibility | Keep compatibility if practical by mapping offset to root-margin semantics. |
| smooth scroll | Add `smooth_scroll` when dbcss can own click handling safely. |
| non-visible targets ignored | Observer logic must ignore invisible sections. |
| dynamic sections require refresh | Add refresh mechanism, likely a `refresh_key` prop or MutationObserver-backed refresh. |
| multiple instances | Remove shared global state; each instance must be scoped. |
| active nav/list/simple anchors | Expose active id through Dioxus state and provide enough info for apps/components to apply `.active`. |
| activation event | Map to signal update and optional callback. |

Issue #11 owns Scrollspy behavior. It should decide the compatibility plan for
the current `target` prop before implementation starts.

## Converter And Docs

Issue #12 owns follow-through after behavior lands:

- update component docs and examples
- remove completed caveats from `docs/DESIGN.md`
- map safe static tooltip/popover/scrollspy Bootstrap attributes in the
  converter
- flag dynamic or ambiguous data attributes for manual review
- keep the raw-Bootstrap gate aligned with the typed surface

## Done Criteria For Tracker

Issue #1 can close only after:

- #8 through #12 are complete
- local and Forge CI checks are green
- tooltip/popover viewport-edge behavior is covered by e2e tests
- scrollspy multi-instance and custom-container behavior is covered by tests
- caveats in `docs/DESIGN.md` reflect reality after implementation
