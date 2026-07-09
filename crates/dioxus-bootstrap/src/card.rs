use dioxus::prelude::*;

/// Bootstrap Card component with optional header, body, and footer slots.
///
/// Renders a `<div class="card">` by default. When `href` is set, renders an
/// `<a class="card" href=...>` instead, so the whole card is a single link — the
/// standard Bootstrap clickable-card pattern. `target` applies only in that mode.
/// This mirrors `Button` (`button.btn` / `a.btn`) and `DropdownItem`, which switch
/// the same way; both render paths carry identical classes via `card_class`.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="card">
///   <div class="card-header">Title</div>
///   <div class="card-body"><p>Content</p></div>
///   <div class="card-footer">Footer</div>
/// </div>
/// <!-- Clickable card (the whole card is a link) -->
/// <a class="card text-decoration-none text-reset" href="/page">
///   <div class="card-body"><h5>Title</h5><p>Text</p></div>
/// </a>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent
/// rsx! {
///     Card {
///         header: rsx! { "Card Title" },
///         body: rsx! { p { "Card content goes here." } },
///         footer: rsx! { "Last updated 3 mins ago" },
///     }
///     // Body-only card
///     Card { body: rsx! { "Simple card" } }
///     // Clickable card — the whole card is a link
///     Card {
///         href: "/page",
///         class: "text-decoration-none text-reset",
///         body: rsx! { h5 { "Title" } p { "Text" } },
///     }
///     // Card with custom header styling (e.g., flex layout with action buttons)
///     Card {
///         class: "mb-3",
///         header_class: "d-flex justify-content-between align-items-center py-2",
///         body_class: "py-2",
///         header: rsx! {
///             span { class: "small", "Server" }
///             button { class: "btn btn-sm btn-outline-secondary py-0 px-1",
///                 i { class: "bi bi-arrow-clockwise small" }
///             }
///         },
///         body: rsx! { p { "Stats here" } },
///     }
///     // Custom layout (children go inside card, outside body)
///     Card { class: "text-center",
///         img { class: "card-img-top", src: "/photo.jpg" }
///         div { class: "card-body", h5 { "Title" } p { "Text" } }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CardProps {
    /// Card header content.
    #[props(default)]
    pub header: Option<Element>,
    /// Card body content.
    #[props(default)]
    pub body: Option<Element>,
    /// Card footer content.
    #[props(default)]
    pub footer: Option<Element>,
    /// When set, render an `<a class="card" href=...>` anchor instead of a
    /// `<div class="card">`, so the whole card is a link.
    #[props(default)]
    pub href: Option<String>,
    /// Anchor `target` (e.g. `"_blank"` to open in a new tab). Only applies when
    /// `href` is set.
    #[props(default)]
    pub target: Option<String>,
    /// Additional CSS classes for the card container.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the card-header div.
    #[props(default)]
    pub header_class: String,
    /// Additional CSS classes for the card body.
    #[props(default)]
    pub body_class: String,
    /// Additional CSS classes for the card-footer div.
    #[props(default)]
    pub footer_class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (rendered inside card, outside body — for custom layouts).
    #[props(default)]
    pub children: Element,
}

/// Root class for the card container. Shared by the `<div>` and `<a>` render
/// paths so both carry identical classes.
fn card_class(class: &str) -> String {
    if class.is_empty() {
        "card".to_string()
    } else {
        format!("card {class}")
    }
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let full_class = card_class(&props.class);

    let header_class = if props.header_class.is_empty() {
        "card-header".to_string()
    } else {
        format!("card-header {}", props.header_class)
    };

    let body_class = if props.body_class.is_empty() {
        "card-body".to_string()
    } else {
        format!("card-body {}", props.body_class)
    };

    let footer_class = if props.footer_class.is_empty() {
        "card-footer".to_string()
    } else {
        format!("card-footer {}", props.footer_class)
    };

    // Build the slot content once so both render paths share it verbatim.
    let inner = rsx! {
        if let Some(header) = props.header {
            div { class: "{header_class}", {header} }
        }
        if let Some(body) = props.body {
            div { class: "{body_class}", {body} }
        }
        {props.children}
        if let Some(footer) = props.footer {
            div { class: "{footer_class}", {footer} }
        }
    };

    // Anchor form: the whole card is a single link. Mirrors `Button`/`DropdownItem`
    // — the early return keeps `props.attributes` unmoved on the `<div>` path.
    if let Some(href) = props.href.clone() {
        let target = props.target.clone();
        return rsx! {
            a { class: "{full_class}", href: "{href}", target: target,
                ..props.attributes,
                {inner}
            }
        };
    }

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            {inner}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_class_base() {
        assert_eq!(card_class(""), "card");
    }

    #[test]
    fn card_class_with_extra() {
        // Identical class output for the `<div>` and `<a href>` render paths.
        assert_eq!(
            card_class("h-100 text-decoration-none text-reset"),
            "card h-100 text-decoration-none text-reset"
        );
    }
}
