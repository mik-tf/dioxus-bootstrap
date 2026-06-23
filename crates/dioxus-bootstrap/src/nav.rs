use dioxus::prelude::*;

use crate::types::{Color, NavbarExpand};

/// Bootstrap Nav component — standalone navigation (not inside a Navbar).
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <ul class="nav nav-pills nav-fill">
///   <li class="nav-item"><a class="nav-link active" href="#">Home</a></li>
///   <li class="nav-item"><a class="nav-link" href="#">Profile</a></li>
///   <li class="nav-item"><a class="nav-link disabled">Disabled</a></li>
/// </ul>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Nav { pills: true, fill: true,
///         NavItem { NavLink { active: true, "Home" } }
///         NavItem { NavLink { "Profile" } }
///         NavItem { NavLink { disabled: true, "Disabled" } }
///     }
///     // Tabs style
///     Nav { tabs: true, /* ... */ }
///     // Underline style
///     Nav { underline: true, /* ... */ }
///     // Vertical with pills
///     Nav { pills: true, vertical: true, /* ... */ }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `pills` — pill style
/// - `tabs` — tab style
/// - `underline` — underline style
/// - `fill` — fill available width
/// - `justified` — equal-width items
/// - `vertical` — vertical layout
#[derive(Clone, PartialEq, Props)]
pub struct NavProps {
    /// Use pill style.
    #[props(default)]
    pub pills: bool,
    /// Use tab style.
    #[props(default)]
    pub tabs: bool,
    /// Use underline style.
    #[props(default)]
    pub underline: bool,
    /// Fill available width equally.
    #[props(default)]
    pub fill: bool,
    /// Justify items to fill width (equal-width items).
    #[props(default)]
    pub justified: bool,
    /// Vertical layout.
    #[props(default)]
    pub vertical: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (NavItems).
    pub children: Element,
}

#[component]
pub fn Nav(props: NavProps) -> Element {
    let mut classes = vec!["nav".to_string()];
    if props.pills {
        classes.push("nav-pills".to_string());
    }
    if props.tabs {
        classes.push("nav-tabs".to_string());
    }
    if props.underline {
        classes.push("nav-underline".to_string());
    }
    if props.fill {
        classes.push("nav-fill".to_string());
    }
    if props.justified {
        classes.push("nav-justified".to_string());
    }
    if props.vertical {
        classes.push("flex-column".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    rsx! {
        ul { class: "{full_class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Bootstrap Navbar component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <nav class="navbar navbar-expand-lg bg-dark" data-bs-theme="dark">
///   <div class="container-fluid">
///     <a class="navbar-brand" href="#">MyApp</a>
///     <button class="navbar-toggler" data-bs-toggle="collapse" data-bs-target="#nav">
///       <span class="navbar-toggler-icon"></span>
///     </button>
///     <div class="collapse navbar-collapse" id="nav">
///       <ul class="navbar-nav"><li class="nav-item"><a class="nav-link" href="/">Home</a></li></ul>
///     </div>
///   </div>
/// </nav>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent
/// let collapsed = use_signal(|| true);
/// rsx! {
///     Navbar { expand: NavbarExpand::Lg, class: "bg-body sticky-top",
///         brand: rsx! { a { class: "navbar-brand", href: "#", "MyApp" } },
///         NavbarToggler { collapsed: collapsed }
///         NavbarCollapse { collapsed: collapsed,
///             NavbarNav {
///                 NavItem { NavLink { href: "/", active: true, "Home" } }
///                 NavItem { NavLink { href: "/about", "About" } }
///             }
///         }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarProps {
    /// Navbar color scheme.
    #[props(default)]
    pub color: Option<Color>,
    /// Responsive expand breakpoint.
    #[props(default)]
    pub expand: NavbarExpand,
    /// Brand element (logo, app name).
    #[props(default)]
    pub brand: Option<Element>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (nav items, collapse, etc.).
    pub children: Element,
}

#[component]
pub fn Navbar(props: NavbarProps) -> Element {
    let mut classes = vec!["navbar".to_string(), props.expand.to_string()];

    let is_dark = matches!(props.color.as_ref(), Some(Color::Dark));

    if let Some(ref color) = props.color {
        match color {
            Color::Dark => {
                classes.push("bg-dark".to_string());
            }
            Color::Light => {
                classes.push("bg-light".to_string());
            }
            c => {
                classes.push(format!("bg-{c}"));
            }
        }
    }

    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let full_class = classes.join(" ");

    rsx! {
        nav {
            class: "{full_class}",
            // Bootstrap 5.3: dark theme via HTML attribute, not CSS class
            "data-bs-theme": if is_dark { "dark" } else { "" },
            ..props.attributes,
            div { class: "container-fluid",
                if let Some(brand) = props.brand {
                    {brand}
                }
                {props.children}
            }
        }
    }
}

/// Navbar toggler button (hamburger menu) for responsive collapse.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<button class="navbar-toggler" data-bs-toggle="collapse" data-bs-target="#nav">` | `NavbarToggler { collapsed: signal }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// # let collapsed_signal = use_signal(|| false);
/// rsx! {
///     NavbarToggler { collapsed: collapsed_signal }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarTogglerProps {
    /// Signal to toggle — will invert the value on click.
    pub collapsed: Signal<bool>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn NavbarToggler(props: NavbarTogglerProps) -> Element {
    let is_collapsed = *props.collapsed.read();
    let mut signal = props.collapsed;

    let full_class = if props.class.is_empty() {
        "navbar-toggler".to_string()
    } else {
        format!("navbar-toggler {}", props.class)
    };

    rsx! {
        button {
            class: "{full_class}",
            r#type: "button",
            "aria-expanded": if !is_collapsed { "true" } else { "false" },
            "aria-label": "Toggle navigation",
            onclick: move |_| signal.set(!is_collapsed),
            ..props.attributes,
            span { class: "navbar-toggler-icon" }
        }
    }
}

/// Navbar collapsible content area.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="collapse navbar-collapse" id="nav">` | `NavbarCollapse { collapsed: signal, ... }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// let collapsed = use_signal(|| true);
/// rsx! {
///     NavbarToggler { collapsed: collapsed }
///     NavbarCollapse { collapsed: collapsed,
///         NavbarNav {
///             NavItem { NavLink { href: "/", "Home" } }
///         }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarCollapseProps {
    /// Signal controlling collapsed state.
    pub collapsed: Signal<bool>,
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
pub fn NavbarCollapse(props: NavbarCollapseProps) -> Element {
    let is_collapsed = *props.collapsed.read();
    let show = if !is_collapsed { " show" } else { "" };

    let full_class = if props.class.is_empty() {
        format!("collapse navbar-collapse{show}")
    } else {
        format!("collapse navbar-collapse{show} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            {props.children}
        }
    }
}

/// Bootstrap navbar navigation list.
///
/// Use this inside [`NavbarCollapse`] to render Bootstrap's required
/// `<ul class="navbar-nav">` wrapper around navbar [`NavItem`] children.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<ul class="navbar-nav">...</ul>` | `NavbarNav { ... }` |
/// | `<ul class="navbar-nav navbar-nav-scroll">...</ul>` | `NavbarNav { scroll: true, ... }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     NavbarNav {
///         NavItem { NavLink { active: true, href: "#", "Home" } }
///         NavItem { NavLink { href: "#docs", "Docs" } }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavbarNavProps {
    /// Enable Bootstrap navbar scroll behavior.
    #[props(default)]
    pub scroll: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (NavItem).
    pub children: Element,
}

#[component]
pub fn NavbarNav(props: NavbarNavProps) -> Element {
    let full_class = navbar_nav_class(props.scroll, &props.class);

    rsx! {
        ul {
            class: "{full_class}",
            ..props.attributes,
            {props.children}
        }
    }
}

fn navbar_nav_class(scroll: bool, class: &str) -> String {
    let mut classes = vec!["navbar-nav".to_string()];
    if scroll {
        classes.push("navbar-nav-scroll".to_string());
    }
    if !class.is_empty() {
        classes.push(class.to_string());
    }
    classes.join(" ")
}

/// Bootstrap NavItem component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<li class="nav-item">` | `NavItem { ... }` |
#[derive(Clone, PartialEq, Props)]
pub struct NavItemProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (NavLink).
    pub children: Element,
}

#[component]
pub fn NavItem(props: NavItemProps) -> Element {
    let full_class = if props.class.is_empty() {
        "nav-item".to_string()
    } else {
        format!("nav-item {}", props.class)
    };

    rsx! {
        li { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap NavLink component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<a class="nav-link active" href="/">Home</a>` | `NavLink { href: "/", active: true, "Home" }` |
/// | `<a class="nav-link disabled">Disabled</a>` | `NavLink { disabled: true, "Disabled" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     NavLink { href: "/dashboard", active: true, "Dashboard" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct NavLinkProps {
    /// Link href.
    #[props(default = "#".to_string())]
    pub href: String,
    /// Active state.
    #[props(default)]
    pub active: bool,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
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
pub fn NavLink(props: NavLinkProps) -> Element {
    let mut classes = vec!["nav-link".to_string()];
    if props.active {
        classes.push("active".to_string());
    }
    if props.disabled {
        classes.push("disabled".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    rsx! {
        a {
            class: "{full_class}",
            href: "{props.href}",
            "aria-current": if props.active { "page" } else { "" },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navbar_nav_class_base() {
        assert_eq!(navbar_nav_class(false, ""), "navbar-nav");
    }

    #[test]
    fn navbar_nav_class_scroll_and_extra_classes() {
        assert_eq!(
            navbar_nav_class(true, "ms-auto"),
            "navbar-nav navbar-nav-scroll ms-auto"
        );
    }
}
