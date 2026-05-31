//! # dioxus-bootstrap-css
//!
//! Type-safe Bootstrap 5.3 components for [Dioxus](https://dioxuslabs.com/).
//!
//! This crate provides a 1-to-1 mapping of every Bootstrap 5.3 component as Dioxus RSX
//! components. All interactive behaviors (modals, dropdowns, accordions, carousels, etc.)
//! are driven by Dioxus signals — **zero JavaScript required**.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use dioxus_bootstrap_css::prelude::*;
//!
//! fn app() -> Element {
//!     let theme = use_signal(|| Theme::Dark);
//!     rsx! {
//!         ThemeProvider { theme }
//!         BootstrapHead {}
//!         Container {
//!             h1 { "Hello Bootstrap!" }
//!             Button { color: Color::Primary, "Click me" }
//!         }
//!     }
//! }
//! ```
//!
//! ## Bootstrap HTML → Dioxus RSX
//!
//! | Bootstrap HTML | Dioxus RSX |
//! |---|---|
//! | `<div class="container">` | `Container { }` |
//! | `<div class="row"><div class="col-md-6">` | `Row { Col { md: ColumnSize::Span(6) } }` |
//! | `<button class="btn btn-primary">` | `Button { color: Color::Primary }` |
//! | `<a class="btn btn-primary" href="/page">` | `Button { color: Color::Primary, href: "/page" }` |
//! | `<div class="card"><div class="card-body">` | `Card { body: rsx! { } }` |
//! | `<div class="alert alert-danger">` | `Alert { color: Color::Danger }` |
//! | `<span class="badge text-bg-success">` | `Badge { color: Color::Success }` |
//! | `<i class="bi bi-search">` | `Icon { name: "search" }` |
//! | `<div class="modal">` + JS | `Modal { show: signal, title: "..." }` |
//! | `<div class="dropdown">` + JS | `Dropdown { open: signal, toggle: rsx! { }, menu: rsx! { } }` |
//! | `<div class="carousel">` + JS | `Carousel { active: signal, slides: vec![...] }` |
//! | `<div class="accordion">` + JS | `Accordion { open: signal }` |
//! | `<div class="toast">` + JS | `Toast { show: signal, title: "..." }` |
//! | `<div class="offcanvas">` + JS | `Offcanvas { show: signal, title: "..." }` |
//! | `<div class="tooltip">` + JS | `Tooltip { text: "...", children }` |
//! | `<div class="popover">` + JS | `Popover { title: "...", body: rsx! { }, children }` |
//! | `<html data-bs-theme="dark">` | `ThemeProvider { theme: signal }` |
//! | `<nav><ol class="breadcrumb">` | `Breadcrumb { BreadcrumbItem { } }` |
//! | `<ul class="list-group">` | `ListGroup { ListGroupItem { } }` |
//! | `<div class="input-group">` | `InputGroup { InputGroupText { }, Input { } }` |
//! | `<div class="progress">` | `Progress { ProgressBar { value: 50.0 } }` |
//! | `<div class="spinner-border">` | `Spinner { }` |
//! | `data-bs-spy="scroll"` | `Scrollspy { target: "body", active: signal }` |
//!
//! ## Modules
//!
//! Every Bootstrap component has its own module. Import everything via the [`prelude`].

pub mod accordion;
pub mod alert;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod carousel;
pub mod collapse;
pub mod dropdown;
pub mod figure;
pub mod form;
pub mod grid;
pub mod head;
pub mod icon;
pub mod input_group;
pub mod list_group;
pub mod modal;
pub mod nav;
pub mod offcanvas;
pub mod pagination;
pub mod placeholder;
pub mod popover;
pub mod progress;
pub mod scrollspy;
pub mod spinner;
pub mod table;
pub mod tabs;
pub mod theme;
pub mod theme_vars;
pub mod toast;
pub mod tooltip;
pub mod types;

/// Prelude — import everything with `use dioxus_bootstrap_css::prelude::*`.
pub mod prelude {
    pub use crate::accordion::*;
    pub use crate::alert::*;
    pub use crate::badge::*;
    pub use crate::breadcrumb::*;
    pub use crate::button::*;
    pub use crate::card::*;
    pub use crate::carousel::*;
    pub use crate::collapse::*;
    pub use crate::dropdown::*;
    pub use crate::figure::*;
    pub use crate::form::*;
    pub use crate::grid::*;
    pub use crate::head::*;
    pub use crate::icon::*;
    pub use crate::input_group::*;
    pub use crate::list_group::*;
    pub use crate::modal::*;
    pub use crate::nav::*;
    pub use crate::offcanvas::*;
    pub use crate::pagination::*;
    pub use crate::placeholder::*;
    pub use crate::popover::*;
    pub use crate::progress::*;
    pub use crate::scrollspy::*;
    pub use crate::spinner::*;
    pub use crate::table::*;
    pub use crate::tabs::*;
    pub use crate::theme::*;
    pub use crate::theme_vars::*;
    pub use crate::toast::*;
    pub use crate::tooltip::*;
    pub use crate::types::*;
}
