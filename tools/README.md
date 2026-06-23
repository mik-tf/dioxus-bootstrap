# tools/

Developer tooling for `dioxus-bootstrap-css`. These tools are not shipped in the
published crate.

## check-no-raw-bootstrap.mjs - migration gate

This gate checks that consumer code uses typed components instead of falling
back to raw Bootstrap. It exists because prose rules are easy to forget, while a
failing build is hard to miss.

Raw Bootstrap means any of these:

- remote Bootstrap stylesheet or script tags
- Bootstrap JavaScript (`data-bs-*`, `bootstrap.bundle.js`, `new bootstrap.*`)
- component class strings with typed equivalents (`btn`, `card`, `modal`,
  `alert`, `badge`, `table`, `dropdown`, `accordion`, `offcanvas`, `toast`,
  `pagination`, and related variants)

Allowed raw classes are layout utilities and content classes without a typed
component: `container`, `row`, `col-*`, `d-flex`, spacing utilities, text
utilities, `form-label`, `card-title`, `card-text`, `modal-title`, and similar
styling helpers.

Usage:

```bash
node tools/check-no-raw-bootstrap.mjs path/to/your-app/src
node tools/check-no-raw-bootstrap.mjs
npm run lint:bootstrap
```

The gate scans `.rs`, `.html`, `.htm`, `.js`, and `.mjs`. It skips `.css`,
`target`, `node_modules`, `.git`, `dist`, `app_dist`, `pkg`, and `.dx`.

Important: this gate is necessary but not sufficient. It proves no raw
Bootstrap component classes, CDN assets, or Bootstrap JavaScript are left. It
does not prove the replacement typed component received equivalent props.
Dropping button color or outline during conversion can still change the UI while
the gate passes.

Use the converter first, then the gate, then visual regression.

## migrate-bootstrap-rsx.mjs - converter

The converter maps Bootstrap intent to typed component props. It preserves
residual utility classes, inline styles, event handlers, keys, children, and
nested layout while turning component classes into typed components.

Converter rule:

- Convert safe static cases deterministically.
- Preserve semantic props: color, outline, size, href, target, slots, spinner
  style, and table/form variants.
- Flag dynamic or ambiguous class strings for manual review instead of guessing.
- If a Bootstrap component shape has no typed representation, fix the crate or
  converter before downstream projects adopt a workaround.

Usage:

```bash
node tools/migrate-bootstrap-rsx.mjs path/to/app/src
node tools/migrate-bootstrap-rsx.mjs --check path/to/app/src
node tools/migrate-bootstrap-rsx.mjs --write path/to/app/src
```

## install-dioxus-cli.sh - Dioxus CLI pin

Installs `dioxus-cli` version matching resolved `dioxus` package in
`Cargo.lock`. Run before `dx build`, `dx serve`, or Playwright checks when
lockfile changes.

```bash
tools/install-dioxus-cli.sh
```

## Tuning

The migration gate's forbidden list is calibrated against the crate's component
set in `tools/check-no-raw-bootstrap.mjs` (`FORBID_PREFIX`, `FORBID_EXACT`,
`ALLOW_EXACT`). When a new component replaces a class people would otherwise
write by hand, update the forbidden list so the gate enforces the new typed
surface.

See `docs/MIGRATION.md` for the full migration workflow and definition of
done.
