use dioxus::prelude::*;

/// Bootstrap Breadcrumb component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<nav><ol class="breadcrumb"><li class="breadcrumb-item"><a href="/">Home</a></li>...` | `Breadcrumb { BreadcrumbItem { href: "/", "Home" } ... }` |
/// | `<li class="breadcrumb-item active" aria-current="page">Current</li>` | `BreadcrumbItem { active: true, "Current" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Breadcrumb {
///         BreadcrumbItem { href: "/", "Home" }
///         BreadcrumbItem { href: "/products", "Products" }
///         BreadcrumbItem { active: true, "Current Page" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (BreadcrumbItem components).
    pub children: Element,
}

#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let full_class = if props.class.is_empty() {
        "breadcrumb".to_string()
    } else {
        format!("breadcrumb {}", props.class)
    };

    rsx! {
        nav { "aria-label": "breadcrumb",
            ol { class: "{full_class}", {props.children} }
        }
    }
}

/// A single item in a Breadcrumb.
#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbItemProps {
    /// Link href. Not rendered if active.
    #[props(default)]
    pub href: String,
    /// Active (current page) state — renders as plain text instead of link.
    #[props(default)]
    pub active: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let mut classes = vec!["breadcrumb-item".to_string()];
    if props.active {
        classes.push("active".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    if props.active {
        rsx! {
            li {
                class: "{full_class}",
                "aria-current": "page",
                {props.children}
            }
        }
    } else {
        rsx! {
            li { class: "{full_class}",
                a { href: "{props.href}", {props.children} }
            }
        }
    }
}
