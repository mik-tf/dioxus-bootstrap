# dioxus-bootstrap-css — converting an Askama + Bootstrap UI to typed dbcss

This crate exists so a server-rendered Bootstrap page can become a Dioxus/WASM app
that looks and behaves identically with **zero Bootstrap JavaScript**. Converting a
UI is a repeatable, *measured* procedure — never trial-and-error against screenshots.

## The rule

Match the **original**, and prove it with a number. "Valid Bootstrap markup" is not
the bar; pixel- and behaviour-equivalence to the page you are replacing is. If the
crate cannot reproduce something Bootstrap does, **fix the crate — never hand-patch
the app.** A per-app workaround hides a gap that every other consumer then re-hits.

## The loop (run it per control)

1. **Read the original — in whichever form it lives:**
   - static Bootstrap markup,
   - declarative `data-bs-*` behaviour, or
   - **imperative JS / a web component** (`new bootstrap.Popover(el, {…})`).

   For the imperative form, read the JS to recover the Bootstrap options *and* the
   exact HTML it injects. "Convert the markup" alone misses this whole class — it is
   the reason JS-driven widgets look wrong after a naive port.
2. **Transfer to the typed component** — map intent to props; keep residual utility
   and inline styles; drop every `data-bs-*` and all Bootstrap JS.
3. **Measure — capture BOTH sides yourself** (never ask a human to be the differ):
   - golden = the original control, candidate = the converted one,
   - fixed viewport, deterministic open-state,
   - **primary — `tools/structural-parity.mjs`:** an element-by-element diff of
     computed style + geometry + text across the two control subtrees. It prints
     every property that differs. Fix the list, re-run, until empty.
   - **backstop — `tools/visual-parity.mjs`** (`compare -metric AE`): gross
     regressions only. A crop is crop-dependent and blurs subtle deltas (grey-vs-dark
     text, one glyph vs another, a 2px border, a missing `<tbody>`) into AA noise —
     the structural checker catches those, the pixel scorer does not.
4. **Classify every delta**: intended (font anti-aliasing, irreducible component
   swap) vs regression (wrong position / size / colour / missing element / mis-placed
   arrow). Zero AE is not the bar; **no regression** is.
5. **If the gap is the crate's, fix the crate + add a test, then re-measure.**

## Why structural, not pixels

`getComputedStyle` resolves the ACTUAL rendered value (colour, font, padding, border,
radius, display) no matter how it was authored — class, inline, CSS var, or fallback.
Two controls with the same computed values render identically; the pixels are a
consequence. Comparing the cause (computed style, element by element) is exhaustive
and deterministic; comparing the effect (a cropped screenshot) is crop-dependent and
noisy. Lead with the structural checker; keep the pixel scorer as a coarse backstop.

For an **imperative-JS / web-component** control there is no static markup to convert,
so the deterministic recipe is: **dump the original's live rendered DOM** (outerHTML +
computed styles) — that is the exact target — **reproduce it byte-identically in rsx**,
then let the structural checker confirm zero mismatch. No creativity, no eyeballing.

## Tools and worked examples

- `tools/check-no-raw-bootstrap.mjs` — completeness gate (no raw classes / CDN / JS).
  Lint-green here is necessary, not sufficient: it cannot see look.
- `tools/structural-parity.mjs` — the PRIMARY parity gate: element-by-element diff of
  computed style + geometry + text between the original and converted control.
  `--emit-js` prints the browser snapshot function; save a snapshot per side, then
  `--a golden.json --b candidate.json` prints every mismatch.
- `tools/visual-parity.mjs` — coarse pixel backstop (optional crop + `compare -metric AE`).
- `examples/popover_parity`, `examples/dropdown_parity` — capture harnesses that pin a
  control at a fixed point and force it open, so a golden-vs-candidate diff isolates
  the control from the page around it. Copy the pattern for new controls.
- `docs/MIGRATION.md` — the full method, the component map, and the overlay/Popper notes.

## Overlays are one family

Popover, Tooltip, and Dropdown are "overlays". Everything that tends to be wrong on a
JS-free port is **Popper parity** — placement near a viewport edge, arrow tracking,
precise anchoring. `src/overlay.rs` reproduces Popper's placement/arrow math; the
Popover and Tooltip apply it. Fix an overlay behaviour there once and every overlay
inherits it. (The Dropdown is CSS-anchored — no arrow, no run-time measurement — so
it is immune to that class by construction.)

## Releasing — the tag publishes; never `cargo publish` by hand

Publishing is automated and triggered by pushing a version **tag**. To cut a release:

1. Move the `CHANGELOG.md` `[Unreleased]` notes under a new `## [X.Y.Z]` heading.
2. Bump `version` in `crates/dioxus-bootstrap/Cargo.toml`.
3. Commit on a branch, squash-merge to `development`, push `development`.
4. Create and push the tag: `git tag -a vX.Y.Z -m "dioxus-bootstrap-css X.Y.Z"` then
   push it to `origin` (Forge).

Pushing the `vX.Y.Z` tag triggers the Forge **Release** workflow
(`.forgejo/workflows/release.yml`): it checks the tag matches the crate version, runs
the gates, and publishes to crates.io — **guarded by an existing-version check, so it
safely skips if that version is already published.** GitHub release + Pages are
automated mirror outputs of Forge (`sync-from-forge.yml`); Forge is the only publisher,
so you never push the GitHub remote by hand either.

**Do NOT run `cargo publish` manually.** The tag does it. A manual publish is at best
redundant (CI then skips the publish step) and at worst races the workflow. Full
step-by-step: `docs/RELEASE.md`.
