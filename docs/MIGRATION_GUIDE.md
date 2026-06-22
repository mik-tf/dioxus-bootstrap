# Bootstrap SSR → Dioxus SPA Migration Guide

Complete guide for converting Bootstrap server-side rendered apps (Tera, Askama, Jinja, Handlebars) to Dioxus WASM SPA using `dioxus-bootstrap-css`.

**Proven on:** Project Mycelium Marketplace — 14 pages, 65 templates, 13/14 pages at <1% perceptual diff.

**For AI agents:** This document is structured for both human developers and AI code assistants. The component mapping table and rules are machine-parseable.

---

## Prerequisites

- Rust with `wasm32-unknown-unknown` target
- Dioxus CLI (`cargo install dioxus-cli --locked`)
- Node.js (for pixelmatch visual testing)
- Chromium (for headless screenshots)

## Step 1: Assessment

Before starting, audit your SSR app:

```bash
# Count templates
find src/views -name "*.html" | wc -l

# Count Bootstrap class usages
grep -r "class=\"" src/views/ | grep -c "btn\|card\|modal\|table\|nav\|col-\|row"

# Check Bootstrap version
head -2 src/static/vendor/bootstrap/bootstrap.min.css
```

## Step 2: Setup

```toml
# Cargo.toml
[dependencies]
dioxus = { version = "0.7", features = ["web", "router"] }
dioxus-bootstrap-css = "0.3.1"
gloo-net = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### BootstrapHead configuration

```rust
use dioxus_bootstrap_css::prelude::*;

// New project (use bundled Bootstrap 5.3.3):
BootstrapHead {}

// Migrating from SSR (match existing Bootstrap version exactly):
BootstrapHead {
    css: BootstrapCss::Url("/static/vendor/bootstrap/bootstrap.min.css".into()),
    icons: BootstrapIcons::Url("/static/vendor/bootstrap-icons/font/bootstrap-icons.css".into()),
}

// Also load your app's custom CSS:
link { rel: "stylesheet", href: "/static/css/styles.css" }
```

### Static asset proxy

If the SPA is on a different domain/port than the SSR backend, proxy `/static/` requests:

```nginx
# nginx.conf
server {
    listen 80;
    server_name spa.example.com;
    location /static/ { proxy_pass http://backend:8000/static/; }
    location / { proxy_pass http://frontend:80; }
}
```

## Step 3: The Conversion Method

For EACH page:

### 3a. Dump the SSR DOM

```bash
chromium --headless --disable-gpu --virtual-time-budget=5000 \
  --dump-dom "https://ssr.example.com/page" > /tmp/ssr_page.html
```

### 3b. Read the DOM, write RSX

Match the HTML structure exactly — same classes, same nesting, same text.

### 3c. Use crate components for interactive elements

**NEVER use `data-bs-toggle` or any `data-bs-*` attributes.** Bootstrap JavaScript is NOT loaded in WASM.

### 3d. Take screenshots and compare

```bash
node tests/perceptual_diff.mjs
```

## Step 4: Component Mapping

### Interactive components (replace Bootstrap JS with Dioxus signals)

| Bootstrap HTML | Dioxus RSX | Signal |
|---------------|------------|--------|
| `<div class="accordion">` + `data-bs-toggle="collapse"` | `Accordion { open }` + `AccordionItem { index, title, open }` | `Signal<Option<usize>>` |
| `<div class="modal fade">` + `data-bs-toggle="modal"` | `Modal { show, title, body, footer }` | `Signal<bool>` |
| `<div class="dropdown">` + `data-bs-toggle="dropdown"` | `Dropdown { open, toggle, menu }` | `Signal<bool>` |
| `<div class="collapse navbar-collapse">` | `NavbarCollapse { collapsed }` | `Signal<bool>` |
| `<button class="navbar-toggler">` | `NavbarToggler { collapsed }` | `Signal<bool>` |
| `<div class="collapse">` | `Collapse { expanded }` | `Signal<bool>` |
| `<div class="offcanvas">` | `Offcanvas { show }` | `Signal<bool>` |
| `<div class="toast">` | `Toast { show }` | `Signal<bool>` |
| `<ul class="nav nav-tabs">` | `TabList` (alias `Tabs`) with `Vec<TabDef>` | `Signal<usize>` |

### CSS-only components (no signals needed)

| Bootstrap HTML | Dioxus RSX |
|---------------|------------|
| `<div class="card"><div class="card-body">` | `Card { body: rsx!{...}, header: rsx!{...}, footer: rsx!{...} }` |
| `<button class="btn btn-primary">` | `Button { color: Color::Primary, "Text" }` |
| `<button class="btn btn-sm btn-outline-danger">` | `Button { color: Color::Danger, size: Size::Sm, outline: true, "Text" }` |
| `<table class="table table-striped">` | `Table { striped: true, hover: true }` |
| `<div class="alert alert-danger">` | `Alert { color: Color::Danger, "Text" }` |
| `<span class="badge bg-success">` | `Badge { color: Color::Success, "Text" }` |
| `<div class="spinner-border">` | `Spinner { color: Some(Color::Primary) }` |
| `<div class="container">` | `Container {}` or `Container { fluid: true }` |
| `<div class="progress"><div class="progress-bar">` | `Progress { ProgressBar { value: 75 } }` |
| `<nav aria-label="breadcrumb">` | `Breadcrumb { BreadcrumbItem { "Home" } }` |

### Layout (use plain RSX)

```rust
// Grid
div { class: "container",
    div { class: "row",
        div { class: "col-md-6", "Left" }
        div { class: "col-md-6", "Right" }
    }
}

// Or use typed components:
Container {
    Row {
        Col { md: ColumnSize::Span(6), "Left" }
        Col { md: ColumnSize::Span(6), "Right" }
    }
}
```

## Step 5: Visual Testing

### Setup

```bash
cd tests
npm init -y
npm install pixelmatch pngjs
```

### Test script

The test script (`perceptual_diff.mjs`) uses:
1. **DOM canary wait** — waits for a specific element (e.g., footer text) before screenshotting
2. **3-run best score** — eliminates timing noise
3. **pixelmatch** — anti-aliasing aware comparison (ignores font rendering differences)

### Threshold

- **<1% perceptual diff = PASS** (industry standard for SSR→SPA)
- 0.01-0.05% = pixel-perfect (rendering engine floor)
- 0.05-0.5% = excellent (minor rendering differences)
- 0.5-1% = good (tiny structural differences)
- >1% = structural issue to fix

### Run

```bash
node tests/perceptual_diff.mjs
```

## Step 6: Enforce — the migration gate

### What "raw Bootstrap" means

Plain Bootstrap is used by hand-writing its class names (`<button class="btn btn-primary">`) and loading its CSS and JavaScript bundle. This crate replaces that with typed Rust components (`Button { color: Color::Primary }`) and CSS bundled into the binary. **"Raw Bootstrap" is anything that bypasses the crate**: hand-written component class strings, a remote stylesheet, or Bootstrap's JS. A finished migration contains none of it. The gate, `tools/check-no-raw-bootstrap.mjs`, makes that mechanical instead of a thing reviewers have to remember.

### Why it fails on three things

1. **CDN** — a remote `<link>`/`<script>` to Bootstrap. A sovereign or offline machine cannot reach it and the app renders unstyled. The crate bundles its assets through `BootstrapHead`, so no remote fetch is needed.
2. **JS** — Bootstrap's JavaScript (`data-bs-toggle`, `bootstrap.bundle.min.js`, `new bootstrap.Modal(...)`). That JS is never loaded in a WASM app, so a `data-bs-toggle="modal"` button *looks* right but does nothing. Interactive widgets must be the crate's signal-driven components.
3. **Raw classes** — a component class string like `class="card"` or `class="btn btn-primary"` instead of `Card {}` / `Button {}`. This loses the type safety and styling consistency the crate exists to provide.

### What stays raw (allowed)

Layout and utilities have no component and are meant to be written directly: `container`, `row`, `col-*`, spacing (`m-*`/`p-*`), flex (`d-flex`), text (`text-*`), `form-label`. Also allowed are *content* classes that live inside a component and have no typed equivalent: `card-title`, `card-text`, `modal-title`. The gate knows this difference, so it flags `card` but not `card-title`.

### How to run it

```bash
node tools/check-no-raw-bootstrap.mjs path/to/your-app/src   # any consumer crate, point at your src
npm run lint:bootstrap                                        # the bundled examples
```

It scans `.rs`, `.html`, and `.js` (not `.css`, where bundled Bootstrap legitimately contains these tokens, and not this crate's own `crates/`, which is what *emits* the classes). A clean port prints `OK` and exits 0. A violation prints each `file:line`, which rule tripped, and the offending line, then exits 1. It is wired into CI here next to clippy; wire it into the consumer's CI the same way. **A migration is done when this exits 0.**

## Common Mistakes

| Mistake | Consequence | Fix |
|---------|-------------|-----|
| Using `data-bs-toggle` | Clicks do nothing (no Bootstrap JS) | Use crate signal-based components |
| Hand-rolling modals with `div.modal.d-block` | No backdrop/escape/animation | Use `Modal { show, title, body, footer }` |
| Adding `fw-bold` to active nav link | Bolder than SSR | SSR uses `nav-link active` (brighter, not bold) |
| All nav items in one `<ul>` | Can't right-align | Use separate `<ul>` groups with `me-auto` |
| Using `BootstrapHead {}` with different Bootstrap version | Layout mismatches | Use `BootstrapHead { css: BootstrapCss::Url("...") }` |
| Not using `--virtual-time-budget` in screenshots | Blank WASM page | Always use 15000+ ms budget |
| Inventing designs instead of copying SSR DOM | Wastes iteration cycles | Dump DOM first, copy exactly |
| Changing navbar when fixing other pages | Regresses passing pages | Never change shared components without measuring all pages |

## Crate Audit Results

All 32 components audited against Bootstrap 5.3 HTML docs:
- **30 of 32 correct** — produce exact Bootstrap HTML
- **2 bugs fixed in v0.3.0** (NavbarCollapse wrapper, Navbar dark theme attribute)
- **v0.3.1** added configurable `BootstrapHead`

## AI Agent Prompt

For AI-assisted conversion, use this prompt:

```
Convert this Bootstrap HTML template to Dioxus RSX using dioxus-bootstrap-css 0.3.1:

[paste HTML here]

Rules:
1. Match the HTML structure exactly — same classes, same nesting, same text
2. Use dioxus-bootstrap-css components for interactive elements (see mapping table)
3. NEVER use data-bs-toggle or any data-bs-* attributes
4. Use plain div { class: "..." } for static Bootstrap layout
5. Load CSS via BootstrapHead { css: BootstrapCss::Url("...") } if migrating from SSR
6. All interactive state via Dioxus signals (use_signal)
7. The port is done only when `node tools/check-no-raw-bootstrap.mjs <src>` exits 0 (see Step 6)
```
