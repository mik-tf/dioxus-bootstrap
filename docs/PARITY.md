# Bootstrap Parity Matrix

This document records how `dioxus-bootstrap-css` (`dbcss`) maps Bootstrap 5.3
behavior into typed Dioxus APIs for Tooltip, Popover, and Scrollspy.

Parent tracker: https://forge.ourworld.tf/lhumina_code/dioxus-bootstrap-css/issues/1

Official Bootstrap references:

- Tooltip: https://getbootstrap.com/docs/5.3/components/tooltips/
- Popover: https://getbootstrap.com/docs/5.3/components/popovers/
- Scrollspy: https://getbootstrap.com/docs/5.3/components/scrollspy/

## Rule

If Bootstrap does it, dbcss should expose a typed Dioxus way to express it. If
Bootstrap does not do it, dbcss should not invent it.

For JavaScript plugin behavior, dbcss uses Dioxus state, props, and callbacks
instead of requiring `bootstrap.bundle.js`.

## Tracker Split

- #8 Overlay: shared viewport-aware positioning core
- #9 Tooltip: Bootstrap trigger and positioning parity
- #10 Popover: Bootstrap trigger, dismiss, and positioning parity
- #11 Scrollspy: scoped observer parity
- #12 Migration/docs: converter, docs, and migration-gate cleanup

## Shared Overlay

Tooltip and Popover share crate-owned overlay positioning. The shared behavior:

- measures trigger and overlay rectangles while visible
- chooses the requested placement when it fits
- falls back when the requested placement would overflow
- exposes Bootstrap-compatible placement classes
- supports Bootstrap-style offset defaults through typed `OverlayOffset`
- avoids Bootstrap JavaScript and Popper.js

Bootstrap options that accept raw HTML strings (`template`, `sanitize`,
`allowList`, `sanitizeFn`) are not copied as unsafe string APIs. dbcss should
prefer typed Dioxus elements or plain text.

## Tooltip

Current dbcss API:

- `TooltipPlacement`: `Auto`, `Top`, `Bottom`, `Start`, `End`
- `TooltipTriggers`: `HOVER_FOCUS`, `HOVER`, `FOCUS`, `CLICK`, `MANUAL`
- `TooltipDelay { show_ms, hide_ms }`
- Props: `text`, `placement`, `fallback_placements`, `trigger`, `delay`,
  `open`, `offset`, `boundary_padding`, `class`, `children`
- Helper: `TooltipDisabledTrigger`

Bootstrap mapping:

| Bootstrap behavior | dbcss expression |
| --- | --- |
| default placement `top` | `placement: TooltipPlacement::Top` default |
| `auto`, `top`, `bottom`, `left`, `right` | `Auto`, `Top`, `Bottom`, `Start`, `End` |
| fallback placements | `fallback_placements` |
| prevent overflow | shared viewport-aware overlay core |
| offset default `[0, 6]` | `OverlayOffset::TOOLTIP` default |
| trigger default `hover focus` | `TooltipTriggers::HOVER_FOCUS` default |
| click, focus, hover, manual | `TooltipTriggers` or controlled `open` |
| show/hide delay | `TooltipDelay` |
| custom class | `class` on rendered tooltip element |
| disabled controls | wrap with `TooltipDisabledTrigger` |
| accessibility | Bootstrap role/classes and stable `aria-describedby` |

Migration converter:

- Converts static `data-bs-toggle="tooltip"` with static `title` or
  `data-bs-title`.
- Preserves static placement, trigger, and custom class when supported.
- Flags dynamic or unsupported Bootstrap attributes for manual review.

## Popover

Current dbcss API:

- `PopoverPlacement`: `Auto`, `Top`, `Bottom`, `Start`, `End`
- `PopoverTriggers`: `CLICK`, `HOVER_FOCUS`, `HOVER`, `FOCUS`, `MANUAL`
- `PopoverDelay { show_ms, hide_ms }`
- Props: `title`, `body`, `placement`, `fallback_placements`, `trigger`,
  `delay`, `open`, `offset`, `boundary_padding`, `dismiss_on_outside_click`,
  `class`, `children`
- Helper: `PopoverDisabledTrigger`

Bootstrap mapping:

| Bootstrap behavior | dbcss expression |
| --- | --- |
| default placement `right` | `placement: PopoverPlacement::End` default |
| `auto`, `top`, `bottom`, `left`, `right` | `Auto`, `Top`, `Bottom`, `Start`, `End` |
| fallback placements | `fallback_placements` |
| prevent overflow | shared viewport-aware overlay core |
| offset default `[0, 8]` | `OverlayOffset::POPOVER` default |
| trigger default `click` | `PopoverTriggers::CLICK` default |
| hover, focus, manual | `PopoverTriggers` or controlled `open` |
| outside click dismiss | `dismiss_on_outside_click` |
| show/hide delay | `PopoverDelay` |
| custom class | `class` on rendered popover element |
| typed rich content | `body: Element` |
| disabled controls | wrap with `PopoverDisabledTrigger` |
| accessibility | Bootstrap role/classes and stable `aria-describedby` |

Migration converter:

- Converts static `data-bs-toggle="popover"` with static `data-bs-content`.
- Preserves static title, placement, trigger, and custom class when supported.
- Flags dynamic, raw-HTML, sanitize, template, container, boundary, offset, or
  fallback-placement attributes for manual review instead of guessing.

## Scrollspy

Current dbcss API:

- Props: `target`, `root`, `active`, `offset`, `root_margin`, `threshold`,
  `refresh_key`, `smooth_scroll`
- Defaults: `target: "body"`, `root: "body"`,
  `root_margin: "0px 0px -25%"`, `threshold: [0.1, 0.5, 1.0]`
- Behavior: scoped per-instance observer/listener state, body or custom scroll
  roots, Bootstrap target semantics for nav/list/simple anchor containers,
  non-visible section filtering, active link class updates, signal updates,
  dynamic refresh, and optional smooth scroll

Bootstrap mapping:

| Bootstrap behavior | dbcss expression |
| --- | --- |
| `data-bs-target` points nav/list/simple links | `target: "#nav"` |
| body scroll | `root: "body"` default |
| custom scroll container | `root: "#scroll-area"` |
| `rootMargin` | `root_margin` |
| `threshold` | `threshold` |
| deprecated `offset` compatibility | `offset` |
| dynamic sections | `refresh_key` plus MutationObserver refresh |
| active section id | app-owned `Signal<String>` via `active` |

Migration converter:

- Flags `data-bs-spy="scroll"` for manual review.
- The converter does not invent the required `Signal<String>`. Add the signal
  and `Scrollspy { target, root, active }` marker by hand, then remove raw
  `data-bs-spy` and `data-bs-target` attributes.

## Definition Of Done

Tracker work is done when:

- implementation issues are closed
- local checks and Forge CI are green
- tooltip/popover viewport-edge behavior is covered by E2E tests
- scrollspy body/custom-root/multi-instance behavior is covered by tests
- converter fixtures cover safe static mappings and manual-review cases
- `docs/DESIGN.md` describes current implementation behavior
