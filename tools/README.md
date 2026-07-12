# tools/

Developer tooling for `dioxus-bootstrap-css`. These tools are not shipped in
the published crate.

## The HTML -> RSX -> typed-dbcss pipeline

Migrating a server-rendered Bootstrap app (plain HTML or Askama templates) to
typed `dioxus-bootstrap-css` components is a three-stage pipeline. Each stage is
deterministic where it safely can be and flags the rest for manual review:

1. **`migrate-html-to-rsx.mjs`** — turns an `.html` template into a Dioxus
   `rsx!` block: tags, attributes, text, nesting, comments, and the common
   Askama control flow. It has no Bootstrap opinion; it just gets you from HTML
   into RSX. Flags inline scripts/handlers and ambiguous Askama for manual work.
2. **`migrate-bootstrap-rsx.mjs`** — takes that RSX (or hand-written RSX) and
   rewrites Bootstrap component classes into typed components with typed props.
3. **`check-no-raw-bootstrap.mjs`** — the gate: fails if any raw Bootstrap
   component class, CDN asset, or Bootstrap JS survives.

Then verify with visual regression. Lint-green is not pixel-identical.

```bash
node tools/migrate-html-to-rsx.mjs page.html -o page.rs   # stage 1: HTML -> RSX
node tools/migrate-bootstrap-rsx.mjs --write page.rs       # stage 2: RSX -> typed components
node tools/check-no-raw-bootstrap.mjs page.rs              # stage 3: gate
```

## migrate-html-to-rsx.mjs - HTML/Askama -> RSX front-end

Translates a plain or Askama `.html` template into a Dioxus `rsx!` block, so the
output can be fed straight into `migrate-bootstrap-rsx.mjs`. It is the
deterministic-80%-plus-flag-the-rest front-end for the pipeline above.

Deterministic mappings:

- **Elements / nesting / void tags** (`<br>`, `<img>`, `<input>`, ...).
- **Attributes**: `class` is kept verbatim as `class: "..."` so the downstream
  converter can type it; Rust-keyword names get the raw form (`for` ->
  `r#for`, `type` -> `r#type`); hyphenated names become string keys
  (`data-bs-target` -> `"data-bs-target"`, `aria-*` likewise); valueless HTML
  boolean attributes (`disabled`, `selected`, ...) become `name: true`; inline
  `style` is preserved.
- **Text and interpolation**: text nodes become string literals with HTML
  entities decoded; `{{ expr }}` becomes `{expr}` inside a formatted string, or
  a bare `{expr}` dyn node when it stands alone; `{{ expr|safe }}` drops the
  `|safe` filter (identity for a Rust string).
- **Askama control flow**: `{% if c %}` / `{% elif %}` / `{% else %}` /
  `{% endif %}` -> `if c { .. } else if .. { .. } else { .. }`; `{% for x in xs %}`
  -> `for x in xs { .. }`; `{% block name %}` -> inlined children under a
  `// block: name` marker. (Output uses Dioxus 0.7 bare-block control flow inside
  the single `rsx!` — no nested `rsx!{}` per arm, which is the version-correct,
  compilable form and is exactly what `migrate-bootstrap-rsx.mjs` then walks.)

Flagged (emitted as a `// TODO(convert): ...` comment plus a `manual_review`
warning, never guessed):

- inline `onclick=` / `on*=` handlers (must become Dioxus event handlers +
  signals) and inline `<script>` / `<style>` blocks;
- `{% match %}` / `{% when %}` (arms are emitted flat and flagged);
- `{% extends %}`, `{% include %}`, `{% import %}`, `{% macro %}`, `{% call %}`,
  `{% let %}`, `{% set %}`, `{% filter %}` (no automatic RSX equivalent);
- Askama control flow embedded inside an attribute value (e.g. a conditional
  `class`), Askama statements sitting between attributes, and non-`safe`
  expression filters or non-trivial interpolated expressions.

Usage (mirrors `migrate-bootstrap-rsx.mjs`):

```bash
node tools/migrate-html-to-rsx.mjs page.html                # print rsx! to stdout
node tools/migrate-html-to-rsx.mjs page.html -o page.rs     # write to a file (- = stdout)
node tools/migrate-html-to-rsx.mjs --write templates/       # write <name>.rs next to each .html
node tools/migrate-html-to-rsx.mjs --check templates/       # parse only; exit 2 if anything is flagged
node tools/migrate-html-to-rsx.mjs --json page.html         # machine-readable stats + warnings
npm run test:migrate:html
```

The converter never invents typed components — that is stage 2's job. Its output
is intentionally plain RSX with raw Bootstrap classes intact.

## check-no-raw-bootstrap.mjs - migration gate

This gate checks that consumer code uses typed components instead of falling
back to raw Bootstrap. It exists because prose rules are easy to forget, while
a failing build is hard to miss.

Raw Bootstrap means:

- remote Bootstrap stylesheet or script tags
- Bootstrap JavaScript (`data-bs-*`, `bootstrap.bundle.js`, `new bootstrap.*`)
- component class strings with typed equivalents (`btn`, `card`, `modal`,
  `alert`, `badge`, `table`, `dropdown`, `accordion`, `offcanvas`, `toast`,
  `tooltip`, `popover`, `pagination`, related variants)

Allowed raw classes are layout utilities and content classes without typed
components: `container`, `row`, `col-*`, `d-flex`, spacing utilities, text
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

Important: the gate is necessary but not sufficient. It proves no raw Bootstrap
component classes, CDN assets, or Bootstrap JavaScript are left. It does not
prove the replacement typed component received equivalent props. Dropping a
button color or outline during conversion still changes UI while the gate
passes. Use the converter first, then the gate, then visual regression.

## migrate-bootstrap-rsx.mjs - converter

The converter maps Bootstrap intent to typed component props. It preserves
residual utility classes, inline styles, event handlers, keys, children, and
nested layout while turning component classes into typed components.

Converter rule:

- Convert safe static cases deterministically.
- Preserve semantic props: color, outline, size, href, target, slots, spinner
  style, table/form variants, static tooltip/popover text/body, placement, and
  trigger.
- Flag dynamic or ambiguous class strings or Bootstrap attributes for manual
  review instead of guessing. Scrollspy is manual review because typed
  `Scrollspy` requires an app-owned `Signal<String>`.
- If a Bootstrap component shape has no typed representation, fix the crate or
  converter before downstream projects adopt a workaround.

Usage:

```bash
node tools/migrate-bootstrap-rsx.mjs path/to/app/src
node tools/migrate-bootstrap-rsx.mjs --check path/to/app/src
node tools/migrate-bootstrap-rsx.mjs --write path/to/app/src
```

## install-dioxus-cli.sh - Dioxus CLI pin

Installs the `dioxus-cli` version matching the resolved `dioxus` package from
Cargo metadata. Run before `dx build`, `dx serve`, or Playwright checks when
Dioxus dependencies change.

```bash
tools/install-dioxus-cli.sh
```

## Tuning

The migration gate's forbidden list is calibrated against the crate's component
set in `tools/check-no-raw-bootstrap.mjs` (`FORBID_PREFIX`, `FORBID_EXACT`,
`ALLOW_EXACT`). When a new component replaces a class people would otherwise
write by hand, update the forbidden list so the gate enforces the new typed
surface. See `docs/content/04_migration.md` for the full migration workflow definition of
done.
