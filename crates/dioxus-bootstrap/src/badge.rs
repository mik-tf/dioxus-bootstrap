use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap Badge component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<span class="badge text-bg-primary">New</span>` | `Badge { color: Color::Primary, "New" }` |
/// | `<span class="badge rounded-pill text-bg-danger">99+</span>` | `Badge { color: Color::Danger, pill: true, "99+" }` |
/// | `<span class="badge text-bg-secondary" role="button">Open</span>` | `Badge { color: Color::Secondary, onclick: move |_| {}, "Open" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Badge { color: Color::Primary, "New" }
///     Badge { color: Color::Danger, pill: true, "99+" }
///     // Inside a heading
///     h1 { "Messages " Badge { color: Color::Info, "4" } }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// Badge color variant.
    #[props(default = Color::Primary)]
    pub color: Color,
    /// Use pill (rounded) style.
    #[props(default)]
    pub pill: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let pill = if props.pill { " rounded-pill" } else { "" };
    let full_class = if props.class.is_empty() {
        format!("badge text-bg-{}{pill}", props.color)
    } else {
        format!("badge text-bg-{}{pill} {}", props.color, props.class)
    };

    rsx! {
        span {
            class: "{full_class}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
