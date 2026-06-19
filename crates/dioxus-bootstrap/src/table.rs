use dioxus::prelude::*;

use crate::types::{Color, Size};

/// Bootstrap Table component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="table-responsive">
///   <table class="table table-striped table-hover">
///     <caption>List of users</caption>
///     <thead><tr><th>Name</th><th>Role</th></tr></thead>
///     <tbody><tr><td>Alice</td><td>Admin</td></tr></tbody>
///   </table>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent
/// rsx! {
///     Table { striped: true, hover: true, responsive: true,
///         caption: "List of users", caption_top: true,
///         thead {
///             tr { th { "Name" } th { "Role" } }
///         }
///         tbody {
///             tr { td { "Alice" } td { "Admin" } }
///         }
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `striped` — striped rows
/// - `striped_columns` — striped columns instead of rows
/// - `hover` — highlight rows on hover
/// - `bordered` — borders on all cells
/// - `responsive` — horizontal scroll wrapper
/// - `caption` / `caption_top` — table caption text and position
/// - `size` — `Size::Sm` for compact table
/// - `color` — table color variant
#[derive(Clone, PartialEq, Props)]
pub struct TableProps {
    /// Striped rows.
    #[props(default)]
    pub striped: bool,
    /// Striped columns instead of rows.
    #[props(default)]
    pub striped_columns: bool,
    /// Highlight rows on hover.
    #[props(default)]
    pub hover: bool,
    /// Add borders to all cells.
    #[props(default)]
    pub bordered: bool,
    /// Remove all borders.
    #[props(default)]
    pub borderless: bool,
    /// Compact table with smaller padding.
    #[props(default)]
    pub size: Size,
    /// Table color variant.
    #[props(default)]
    pub color: Option<Color>,
    /// Wrap in a responsive container for horizontal scrolling.
    #[props(default)]
    pub responsive: bool,
    /// Caption text (displayed above or below the table).
    #[props(default)]
    pub caption: String,
    /// Place caption at the top (default is bottom per Bootstrap).
    #[props(default)]
    pub caption_top: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (thead, tbody, etc.).
    pub children: Element,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let mut classes = vec!["table".to_string()];

    if props.striped {
        classes.push("table-striped".to_string());
    }
    if props.striped_columns {
        classes.push("table-striped-columns".to_string());
    }
    if props.caption_top {
        classes.push("caption-top".to_string());
    }
    if props.hover {
        classes.push("table-hover".to_string());
    }
    if props.bordered {
        classes.push("table-bordered".to_string());
    }
    if props.borderless {
        classes.push("table-borderless".to_string());
    }
    if let Size::Sm = props.size {
        classes.push("table-sm".to_string());
    }
    if let Some(ref c) = props.color {
        classes.push(format!("table-{c}"));
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }

    let full_class = classes.join(" ");

    let caption = props.caption.clone();

    if props.responsive {
        rsx! {
            div { class: "table-responsive",
                table { class: "{full_class}",
                    ..props.attributes,
                    if !caption.is_empty() {
                        caption { "{caption}" }
                    }
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            table { class: "{full_class}",
                ..props.attributes,
                if !caption.is_empty() {
                    caption { "{caption}" }
                }
                {props.children}
            }
        }
    }
}
