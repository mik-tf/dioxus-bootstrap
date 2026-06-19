use dioxus::prelude::*;

use crate::types::{Color, Size, SpinnerStyle};

/// Bootstrap Spinner (loading indicator) component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="spinner-border">` | `Spinner {}` |
/// | `<div class="spinner-grow text-success">` | `Spinner { style: SpinnerStyle::Grow, color: Color::Success }` |
/// | `<div class="spinner-border spinner-border-sm">` | `Spinner { size: Size::Sm }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Spinner {}
///     Spinner { color: Color::Success, style: SpinnerStyle::Grow }
///     Spinner { size: Size::Sm, "Loading..." }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct SpinnerProps {
    /// Spinner color.
    #[props(default)]
    pub color: Option<Color>,
    /// Spinner size.
    #[props(default)]
    pub size: Size,
    /// Spinner style (border or grow).
    #[props(default)]
    pub style: SpinnerStyle,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Screen reader text (child elements).
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let base = match props.style {
        SpinnerStyle::Border => "spinner-border",
        SpinnerStyle::Grow => "spinner-grow",
    };

    let size_class = match props.size {
        Size::Sm => format!(" {base}-sm"),
        _ => String::new(),
    };

    let color_class = match &props.color {
        Some(c) => format!(" text-{c}"),
        None => String::new(),
    };

    let full_class = if props.class.is_empty() {
        format!("{base}{size_class}{color_class}")
    } else {
        format!("{base}{size_class}{color_class} {}", props.class)
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "status",
            ..props.attributes,
            span { class: "visually-hidden", {props.children} }
        }
    }
}
