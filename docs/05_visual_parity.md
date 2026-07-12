# Visual parity and overlays

Removing raw Bootstrap classes proves *completeness*, not *fidelity*. This
chapter covers the layer that proves the converted UI still looks right: the
objective parity gate, and the overlay positioning math that is the most common
source of "the port looks slightly off" bugs.

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
worked examples, each surfaced by the parity gate above:

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
