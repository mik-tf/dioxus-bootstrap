use dioxus::prelude::*;

/// Bootstrap Card component with optional header, body, and footer slots.
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

#[component]
pub fn Card(props: CardProps) -> Element {
    let full_class = if props.class.is_empty() {
        "card".to_string()
    } else {
        format!("card {}", props.class)
    };

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

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
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
        }
    }
}
