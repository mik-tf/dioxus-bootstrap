# Changelog

All notable changes to dioxus-bootstrap-css are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [0.5.0] — Theming fidelity, forward-compatible construction, tested examples

### Changed

- Derived theme tokens (`-text-emphasis`, `-bg-subtle`, `-border-subtle`) now use Bootstrap 5.3's exact Sass weights (light: shade 60% / tint 80% / tint 60%; dark: tint 40% / shade 80% / shade 40%), so `BootstrapThemeProvider` output matches a native Bootstrap build.

### Added

- `SemanticColorScale` builder methods: `with_rgb`, `with_text_emphasis`, `with_bg_subtle`, `with_border_subtle`, for forward-compatible construction without struct literals.
- Docs distinguishing `ThemeProvider` (light/dark mode) from `BootstrapThemeProvider` (color overrides), and a `BootstrapHead` recipe for apps embedded in a host that already loads Bootstrap.

### Fixed

- Component doc examples now compile as doctests, and CI runs `cargo test`, so the examples stay correct.

## [0.4.0] — Runtime CSS Variable Theming

### Added

- `BootstrapThemeProvider` and the `theme_vars` module (`BootstrapTheme`, `ThemeColors`, `SurfaceColors`) for overriding Bootstrap 5.3 CSS variables at runtime, with separate light and dark tokens. Set custom brand colors and surfaces without rebuilding the CSS. Thanks to @enaut for the contribution.

### Fixed

- Removed a redundant clone in `Toast` that failed clippy on recent toolchains.

## [0.3.1] — Configurable Bootstrap CSS Loading

### Added

- `BootstrapCss` enum: `Bundled` (default), `Url(String)`, `Cdn(String)`, `None`
- `BootstrapIcons` enum: `Bundled` (default), `Url(String)`, `None`
- `BootstrapHead` now accepts `css` and `icons` props for full control over CSS loading
- Default behavior unchanged — `BootstrapHead {}` still loads bundled Bootstrap 5.3.3

### Use cases

- **New projects**: `BootstrapHead {}` — zero config, bundled CSS (same as before)
- **SSR migration**: `BootstrapHead { css: BootstrapCss::Url("/static/vendor/bootstrap.min.css".into()) }` — pixel-perfect match with existing SSR
- **CDN**: `BootstrapHead { css: BootstrapCss::Cdn("5.3.3".into()) }` — load from jsDelivr
- **Full control**: `BootstrapHead { css: BootstrapCss::None }` — user loads CSS themselves

## [0.3.0] — Breaking Changes

### Breaking

- **NavbarCollapse** no longer wraps children in `<ul class="navbar-nav me-auto">`. Children render directly inside the collapse div. Users must provide their own `<ul>` elements for left/right navbar grouping. This matches Bootstrap 5.3's documented HTML structure where the collapse div contains multiple `<ul>` groups.

**Before (0.2.x):**
```rust
NavbarCollapse { collapsed,
    // All items wrapped in one <ul class="me-auto"> — can't right-align
    li { Link { to: "/", "Home" } }
    li { Link { to: "/login", "Login" } }
}
```

**After (0.3.0):**
```rust
NavbarCollapse { collapsed,
    ul { class: "navbar-nav me-auto",
        li { Link { to: "/", "Home" } }
    }
    ul { class: "navbar-nav",
        li { Link { to: "/login", "Login" } }
    }
}
```

### Fixed

- **Navbar** dark color variant now uses `data-bs-theme="dark"` HTML attribute instead of pushing `[data-bs-theme=dark]` as a CSS class (which was invalid and produced broken output)

### Audit

Full audit of all 32 component files against Bootstrap 5.3 HTML docs:
- 30 of 32 components produce correct Bootstrap HTML
- 2 bugs fixed (this release)
- Crate is production-ready (proven via Project Mycelium Marketplace conversion — 65 templates, pixel-perfect)

## [0.2.6]

- Documentation update — comprehensive migration tables, showcase examples

## [0.2.5]

- Toast supports headerless mode with close button (Bootstrap 5.3 `d-flex` pattern): omit `title` and set `show_close: true`
- Toast `on_dismiss` callback fires when the toast is dismissed

## [0.2.4]

- Alert `on_dismiss` callback fires when a dismissible alert is closed

## [0.2.3]

- Button `target` prop (e.g., `"_blank"` for new tab)
- Button `download` prop (triggers file download when used with `href`)

## [0.2.2]

- Button `href` prop: renders `<a>` instead of `<button>` for link-button pattern

## [0.2.1]

- All components extend `GlobalAttributes` — Card, Table, Nav, Modal, Grid, Form, and others accept any standard HTML attribute (`id`, `title`, `aria-*`, `data-*`, etc.)

## [0.2.0]

- Button extends `GlobalAttributes`: accepts any HTML attribute directly

## [0.1.9]

- Bug fixes

## [0.1.8]

- Card styling improvements

## [0.1.7]

- Card `header_class`, `body_class`, `footer_class` props for fine-grained section styling

## [0.1.4 – 0.1.6]

- Initial release with core Bootstrap 5.3 components: Button, Card, Alert, Badge, Table, Modal, Dropdown, Tabs, Accordion, Collapse, Nav, Navbar, Form controls, Grid, Icon, Toast, Carousel, Tooltip, Popover, Offcanvas, Scrollspy, Progress, Spinner, Placeholder, ListGroup, Pagination, Breadcrumb, Figure, Ratio, ThemeProvider, BootstrapHead
