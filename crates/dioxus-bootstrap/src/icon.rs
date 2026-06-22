use dioxus::prelude::*;

/// Bootstrap Icon component.
///
/// Renders a Bootstrap Icon by name. See https://icons.getbootstrap.com/ for available icons.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<i class="bi bi-search">` | `Icon { name: "search" }` |
/// | `<i class="bi bi-shield-lock me-2 fs-4">` | `Icon { name: "shield-lock", class: "me-2 fs-4" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Icon { name: "search" }
///     Icon { name: "shield-lock", class: "me-2 fs-4" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct IconProps {
    /// Icon name without the `bi-` prefix (e.g., "search", "shield-lock").
    pub name: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let icon_class = if props.class.is_empty() {
        format!("bi bi-{}", props.name)
    } else {
        format!("bi bi-{} {}", props.name, props.class)
    };
    rsx! {
    i { class: "{icon_class}", ..props.attributes }
    }
}
