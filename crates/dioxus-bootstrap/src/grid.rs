use dioxus::prelude::*;

use crate::types::ColumnSize;

/// Bootstrap Container component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="container">` | `Container { }` |
/// | `<div class="container-fluid">` | `Container { fluid: true }` |
/// | `<div class="container py-4">` | `Container { class: "py-4" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Container { "Fixed width content" }
///     Container { fluid: true, "Full width content" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ContainerProps {
    /// Use container-fluid for full width.
    #[props(default)]
    pub fluid: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    let base = if props.fluid {
        "container-fluid"
    } else {
        "container"
    };
    let full_class = if props.class.is_empty() {
        base.to_string()
    } else {
        format!("{base} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap Row component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="row">` | `Row { }` |
/// | `<div class="row g-3">` | `Row { class: "g-3" }` |
/// | `<div class="row align-items-center">` | `Row { class: "align-items-center" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Row { class: "g-3",
///         Col { lg: ColumnSize::Span(6), "Left" }
///         Col { lg: ColumnSize::Span(6), "Right" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RowProps {
    /// Additional CSS classes (e.g., "g-3", "align-items-center").
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let full_class = if props.class.is_empty() {
        "row".to_string()
    } else {
        format!("row {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap Col (column) component with responsive breakpoint props.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="col">` | `Col { }` |
/// | `<div class="col-lg-3">` | `Col { lg: ColumnSize::Span(3) }` |
/// | `<div class="col-md-6 col-lg-4">` | `Col { md: ColumnSize::Span(6), lg: ColumnSize::Span(4) }` |
/// | `<div class="col-auto">` | `Col { xs: ColumnSize::Auto }` |
/// | `<div class="col-md-6 offset-md-3">` | `Col { md: ColumnSize::Span(6), offset_md: Some(3) }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Col { xs: ColumnSize::Span(12), md: ColumnSize::Span(6), lg: ColumnSize::Span(4),
///         "Responsive column"
///     }
///     Col { lg: ColumnSize::Auto, "Auto-width column" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ColProps {
    /// Column size at xs breakpoint (default, no breakpoint prefix).
    #[props(default)]
    pub xs: Option<ColumnSize>,
    /// Column size at sm breakpoint.
    #[props(default)]
    pub sm: Option<ColumnSize>,
    /// Column size at md breakpoint.
    #[props(default)]
    pub md: Option<ColumnSize>,
    /// Column size at lg breakpoint.
    #[props(default)]
    pub lg: Option<ColumnSize>,
    /// Column size at xl breakpoint.
    #[props(default)]
    pub xl: Option<ColumnSize>,
    /// Column size at xxl breakpoint.
    #[props(default)]
    pub xxl: Option<ColumnSize>,
    /// Offset at xs breakpoint.
    #[props(default)]
    pub offset: Option<u8>,
    /// Offset at sm breakpoint.
    #[props(default)]
    pub offset_sm: Option<u8>,
    /// Offset at md breakpoint.
    #[props(default)]
    pub offset_md: Option<u8>,
    /// Offset at lg breakpoint.
    #[props(default)]
    pub offset_lg: Option<u8>,
    /// Offset at xl breakpoint.
    #[props(default)]
    pub offset_xl: Option<u8>,
    /// Offset at xxl breakpoint.
    #[props(default)]
    pub offset_xxl: Option<u8>,
    /// Column order (0-5 or "first"/"last" via class prop).
    #[props(default)]
    pub order: Option<u8>,
    /// Column order at sm breakpoint.
    #[props(default)]
    pub order_sm: Option<u8>,
    /// Column order at md breakpoint.
    #[props(default)]
    pub order_md: Option<u8>,
    /// Column order at lg breakpoint.
    #[props(default)]
    pub order_lg: Option<u8>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    let mut classes = Vec::new();

    if let Some(size) = &props.xs {
        classes.push(format!("col-{size}"));
    }
    if let Some(size) = &props.sm {
        classes.push(format!("col-sm-{size}"));
    }
    if let Some(size) = &props.md {
        classes.push(format!("col-md-{size}"));
    }
    if let Some(size) = &props.lg {
        classes.push(format!("col-lg-{size}"));
    }
    if let Some(size) = &props.xl {
        classes.push(format!("col-xl-{size}"));
    }
    if let Some(size) = &props.xxl {
        classes.push(format!("col-xxl-{size}"));
    }

    // Default to "col" if no breakpoints specified
    if classes.is_empty() {
        classes.push("col".to_string());
    }

    // Offsets
    if let Some(n) = props.offset {
        classes.push(format!("offset-{n}"));
    }
    if let Some(n) = props.offset_sm {
        classes.push(format!("offset-sm-{n}"));
    }
    if let Some(n) = props.offset_md {
        classes.push(format!("offset-md-{n}"));
    }
    if let Some(n) = props.offset_lg {
        classes.push(format!("offset-lg-{n}"));
    }
    if let Some(n) = props.offset_xl {
        classes.push(format!("offset-xl-{n}"));
    }
    if let Some(n) = props.offset_xxl {
        classes.push(format!("offset-xxl-{n}"));
    }

    // Order
    if let Some(n) = props.order {
        classes.push(format!("order-{n}"));
    }
    if let Some(n) = props.order_sm {
        classes.push(format!("order-sm-{n}"));
    }
    if let Some(n) = props.order_md {
        classes.push(format!("order-md-{n}"));
    }
    if let Some(n) = props.order_lg {
        classes.push(format!("order-lg-{n}"));
    }

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let full_class = classes.join(" ");

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}
