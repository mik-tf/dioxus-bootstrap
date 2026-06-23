use dioxus::prelude::*;

use crate::types::{Color, Size};

/// Bootstrap Button component.
///
/// Renders a `<button>` by default. When `href` is set, renders an `<a>` element
/// instead (Bootstrap link-button pattern).
///
/// Accepts all standard HTML attributes via `extends = GlobalAttributes`.
/// This means `title`, `data-bs-toggle`, `aria-label`, `id`, etc. all work.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<button class="btn btn-primary">` | `Button { color: Color::Primary, "Text" }` |
/// | `<button class="btn btn-outline-danger btn-sm">` | `Button { color: Color::Danger, outline: true, size: Size::Sm, "Text" }` |
/// | `<button class="btn btn-link btn-sm">` | `Button { link: true, size: Size::Sm, "Text" }` |
/// | `<button class="btn btn-sm">` (neutral, no color variant) | `Button { plain: true, size: Size::Sm, "Text" }` |
/// | `<button class="btn btn-success btn-lg" disabled>` | `Button { color: Color::Success, size: Size::Lg, disabled: true, "Text" }` |
/// | `<a class="btn btn-primary" href="/page">` | `Button { color: Color::Primary, href: "/page", "Link" }` |
/// | `<a class="btn btn-sm" href="f.json" target="_blank" download="f.json">` | `Button { size: Size::Sm, href: "f.json", target: "_blank", download: "f.json", "DL" }` |
/// | `<button class="btn btn-primary" title="Tip">` | `Button { color: Color::Primary, title: "Tip", "Text" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Button { color: Color::Primary, "Click me" }
///     Button { color: Color::Danger, outline: true, size: Size::Sm, "Delete" }
///     Button { color: Color::Success, disabled: true, "Saved" }
///     Button { color: Color::Warning, onclick: move |_| { /* handler */ }, "Action" }
///     // Link button — renders <a> instead of <button>:
///     Button { color: Color::Primary, href: "/page", "Go to Page" }
///     // HTML attributes work directly:
///     Button { color: Color::Secondary, title: "Tooltip text", "Hover me" }
///     Button { color: Color::Primary, "data-bs-toggle": "modal", "Open Modal" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// Button color variant.
    #[props(default)]
    pub color: Color,
    /// Use outline style instead of filled.
    #[props(default)]
    pub outline: bool,
    /// Use Bootstrap link-button style (`btn-link`).
    #[props(default)]
    pub link: bool,
    /// Render a neutral button with no color variant: bare `.btn` (Bootstrap's
    /// base button already renders neutral — body-colored text, transparent
    /// background and border, with the standard focus ring and pointer cursor).
    /// Pair with utility classes (`border-0`, `p-0`, …) for ghost / borderless /
    /// text-button styles. Takes precedence over `outline`; ignored when `link`
    /// is set.
    #[props(default)]
    pub plain: bool,
    /// Button size.
    #[props(default)]
    pub size: Size,
    /// Whether the button is disabled.
    #[props(default)]
    pub disabled: bool,
    /// When set, renders an `<a>` element instead of `<button>` (link-button pattern).
    #[props(default)]
    pub href: Option<String>,
    /// Link target (e.g., `"_blank"`). Only used when `href` is set.
    #[props(default)]
    pub target: Option<String>,
    /// Download filename. Only used when `href` is set.
    #[props(default)]
    pub download: Option<String>,
    /// HTML button type attribute (ignored when `href` is set).
    #[props(default = "button".to_string())]
    pub r#type: String,
    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Active (pressed) state.
    #[props(default)]
    pub active: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes (title, data-bs-toggle, aria-*, id, etc.)
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    // The color/variant segment. A `plain` button is bare `.btn` with no variant
    // (Bootstrap's base button renders neutral). Each non-empty variant carries
    // its own leading space, so an empty (plain) segment leaves no double space.
    let variant_class = if props.plain {
        String::new()
    } else if props.link {
        " btn-link".to_string()
    } else {
        let style = if props.outline { "btn-outline" } else { "btn" };
        let color = props.color;
        format!(" {style}-{color}")
    };

    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" btn-{s}"),
    };

    let active_class = if props.active { " active" } else { "" };

    let full_class = if props.class.is_empty() {
        format!("btn{variant_class}{size_class}{active_class}")
    } else {
        format!(
            "btn{variant_class}{size_class}{active_class} {}",
            props.class
        )
    };

    if let Some(href) = &props.href {
        // Link-button: render <a> with role="button"
        let disabled_class = if props.disabled { " disabled" } else { "" };
        let link_class = format!("{full_class}{disabled_class}");
        let target = props.target.clone();
        let download = props.download.clone();
        rsx! {
            a {
                class: "{link_class}",
                href: "{href}",
                role: "button",
                target: target,
                download: download,
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                ..props.attributes,
                {props.children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{full_class}",
                r#type: "{props.r#type}",
                disabled: props.disabled,
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
}

/// Bootstrap ButtonGroup component.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     ButtonGroup {
///         Button { color: Color::Primary, "Left" }
///         Button { color: Color::Primary, "Middle" }
///         Button { color: Color::Primary, "Right" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ButtonGroupProps {
    /// Button group size.
    #[props(default)]
    pub size: Size,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (buttons).
    pub children: Element,
}

#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" btn-group-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("btn-group{size_class}")
    } else {
        format!("btn-group{size_class} {}", props.class)
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "group",
            {props.children}
        }
    }
}

/// Bootstrap ButtonToolbar — groups multiple ButtonGroups.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     ButtonToolbar {
///         ButtonGroup {
///             Button { color: Color::Primary, "1" }
///             Button { color: Color::Primary, "2" }
///         }
///         ButtonGroup {
///             Button { color: Color::Secondary, "A" }
///         }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ButtonToolbarProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (ButtonGroups).
    pub children: Element,
}

#[component]
pub fn ButtonToolbar(props: ButtonToolbarProps) -> Element {
    let full_class = if props.class.is_empty() {
        "btn-toolbar".to_string()
    } else {
        format!("btn-toolbar {}", props.class)
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "toolbar",
            {props.children}
        }
    }
}
