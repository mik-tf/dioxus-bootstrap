# tools/

Developer tooling for dioxus-bootstrap-css. These are not shipped in the published crate.

## check-no-raw-bootstrap.mjs — the migration gate

A mechanical check that a UI actually uses this crate instead of falling back to plain
Bootstrap. It exists because the crate's promise (typed components, bundled CSS, no
JavaScript) is only kept if nothing reintroduces "raw Bootstrap" by hand. Prose rules in
the migration guide rely on memory; this fails the build instead.

**Raw Bootstrap** = bypassing the typed components: hand-written component class strings,
a remote CDN stylesheet/script, or Bootstrap's JS bundle. The gate **fails (exit 1)** on any
of three things:

| Check | Fails on | Why |
| ----- | -------- | --- |
| CDN   | a remote `<link>`/`<script>` to Bootstrap | offline-first: bundle assets via `BootstrapHead`, never fetch them |
| JS    | `data-bs-*` attributes, `bootstrap.bundle.js`, `new bootstrap.*` | Bootstrap JS is not loaded in WASM, so those widgets are dead; use the signal-driven components |
| Raw   | component class strings (`btn`, `card`, `modal`, `alert`, `badge`, `table`, `dropdown`, `accordion`, `offcanvas`, `toast`, `pagination`, ...) | use the typed component instead |

**Allowed** (these have no component and stay raw): layout and utilities — `container`, `row`,
`col-*`, `d-flex`, `m-*`/`p-*`, `text-*`, `form-label` — and content classes that live inside a
component, like `card-title`, `card-text`, `modal-title`.

### Usage

```bash
node tools/check-no-raw-bootstrap.mjs path/to/your-app/src   # any consumer crate
node tools/check-no-raw-bootstrap.mjs                        # defaults to ./examples
npm run lint:bootstrap                                        # same, via package.json
```

Scans `.rs`, `.html`, and `.js`. It does not scan `.css` (bundled Bootstrap legitimately
contains these tokens) or this crate's own `crates/` (the components are what emit the
classes). Output is `OK` + exit 0 when clean, or a `file:line` + rule + offending line per
violation + exit 1.

### Tuning

The forbidden list is calibrated against the crate's component set in
`tools/check-no-raw-bootstrap.mjs` (`FORBID_PREFIX`, `FORBID_EXACT`, `ALLOW_EXACT`). When a
new component lands that replaces a class people would otherwise write by hand, add its class
there. See `docs/MIGRATION_GUIDE.md` Step 6 for the full rationale.
