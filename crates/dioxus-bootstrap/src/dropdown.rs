use dioxus::prelude::*;

/// Bootstrap Dropdown component — signal-driven, no JavaScript.
///
/// Replaces Bootstrap's dropdown JavaScript plugin with signal-controlled open/close.
/// Supports split buttons, drop directions, and auto-closes on outside click.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript) -->
/// <div class="dropdown">
///   <button class="btn btn-secondary dropdown-toggle" data-bs-toggle="dropdown">Menu</button>
///   <ul class="dropdown-menu">
///     <li><button class="dropdown-item">Action</button></li>
///     <li><hr class="dropdown-divider"></li>
///     <li><button class="dropdown-item">Other</button></li>
///   </ul>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent
/// let open = use_signal(|| false);
/// rsx! {
///     Dropdown { open: open,
///         toggle: rsx! { "Menu" },
///         menu: rsx! {
///             DropdownItem { "Action" }
///             DropdownDivider {}
///             DropdownItem { "Other" }
///         },
///     }
///     // Split button variant
///     Dropdown { open: open, split: true, color: Color::Danger,
///         toggle: rsx! { "Delete" },
///         menu: rsx! { DropdownItem { "Confirm Delete" } },
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `open` — `Signal<bool>` controlling open state
/// - `toggle` — toggle button content (Element)
/// - `menu` — dropdown menu content (Element)
/// - `split` — split button mode (separate action button + caret toggle)
/// - `color` — button color in split mode
/// - `direction` — `DropDirection::Down`, `Up`, `Start`, `End`
/// - `align_end` — align menu's right edge to the toggle (works JS-free)
#[derive(Clone, PartialEq, Props)]
pub struct DropdownProps {
    /// Signal controlling dropdown open state.
    pub open: Signal<bool>,
    /// Toggle button content.
    pub toggle: Element,
    /// Dropdown menu content (DropdownItem components).
    pub menu: Element,
    /// Additional CSS classes for the dropdown container.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the toggle button.
    #[props(default)]
    pub toggle_class: String,
    /// Drop direction.
    #[props(default)]
    pub direction: DropDirection,
    /// Align menu to the end (right).
    #[props(default)]
    pub align_end: bool,
    /// Split button mode — toggle is a separate caret-only button.
    #[props(default)]
    pub split: bool,
    /// Color for split button mode (used for the main button).
    #[props(default)]
    pub color: Option<crate::types::Color>,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

/// Dropdown direction.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DropDirection {
    #[default]
    Down,
    Up,
    Start,
    End,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let is_open = *props.open.read();
    let mut open_signal = props.open;

    let dir_class = match props.direction {
        DropDirection::Down => "dropdown",
        DropDirection::Up => "dropup",
        DropDirection::Start => "dropstart",
        DropDirection::End => "dropend",
    };

    let container_class = if props.class.is_empty() {
        dir_class.to_string()
    } else {
        format!("{dir_class} {}", props.class)
    };

    let color_name = match &props.color {
        Some(c) => format!("{c}"),
        None => "secondary".to_string(),
    };

    let toggle_class = if props.split {
        format!("btn btn-{color_name} dropdown-toggle dropdown-toggle-split")
    } else if props.toggle_class.is_empty() {
        format!("btn btn-{color_name} dropdown-toggle")
    } else {
        format!("btn dropdown-toggle {}", props.toggle_class)
    };

    let menu_class = if is_open {
        if props.align_end {
            "dropdown-menu dropdown-menu-end show"
        } else {
            "dropdown-menu show"
        }
    } else if props.align_end {
        "dropdown-menu dropdown-menu-end"
    } else {
        "dropdown-menu"
    };

    rsx! {
        // Invisible overlay to close on outside click (only when open)
        if is_open {
            div {
                style: "position: fixed; inset: 0; z-index: 990;",
                onclick: move |_| open_signal.set(false),
            }
        }
        div { class: "{container_class}",
            style: if is_open { "position: relative; z-index: 991;" } else { "" },
            ..props.attributes,
            // Split mode: main button + separate toggle caret
            if props.split {
                button {
                    class: "btn btn-{color_name}",
                    r#type: "button",
                    {props.toggle.clone()}
                }
            }
            button {
                class: "{toggle_class}",
                r#type: "button",
                "aria-expanded": if is_open { "true" } else { "false" },
                onclick: move |evt| {
                    evt.stop_propagation();
                    open_signal.set(!is_open);
                },
                if !props.split {
                    {props.toggle}
                }
                if props.split {
                    span { class: "visually-hidden", "Toggle Dropdown" }
                }
            }
            ul { class: "{menu_class}",
                // Bootstrap 5.3 gates .dropdown-menu-end right-alignment on
                // [data-bs-popper], set only by Bootstrap's JS. This crate is
                // JS-free, so apply Bootstrap's own end values directly.
                style: if props.align_end { "right: 0; left: auto;" } else { "" },
                // Close dropdown when clicking an item
                onclick: move |_| open_signal.set(false),
                {props.menu}
            }
        }
    }
}

/// Standalone Bootstrap dropdown menu.
///
/// Use this when open/position behavior is owned by a surrounding component
/// but the menu and items should still use Bootstrap dropdown structure.
#[derive(Clone, PartialEq, Props)]
pub struct DropdownMenuProps {
    /// Whether to show the menu.
    #[props(default)]
    pub show: bool,
    /// Align menu to the end side.
    #[props(default)]
    pub align_end: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child menu items.
    pub children: Element,
}

#[component]
pub fn DropdownMenu(props: DropdownMenuProps) -> Element {
    let mut classes = vec!["dropdown-menu".to_string()];
    if props.align_end {
        classes.push("dropdown-menu-end".to_string());
    }
    if props.show {
        classes.push("show".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");
    // JS-free end-alignment: Bootstrap gates .dropdown-menu-end{right:0;left:auto}
    // on [data-bs-popper] (set only by its JS), so apply the values directly.
    let align_style = if props.align_end {
        "right: 0; left: auto;"
    } else {
        ""
    };
    rsx! {
    ul { class: "{full_class}", style: "{align_style}", ..props.attributes, {props.children} }
    }
}

/// A single item in a Dropdown menu.
///
/// Renders a `<button class="dropdown-item">` by default. Set `href` to render a
/// real `<a class="dropdown-item" href=...>` instead — use this for menu entries
/// that navigate to a URL so the browser's link behaviours work (middle-click /
/// ctrl-click to open in a background tab, copy-link and open-in-new-window
/// context actions, and a visible target URL on hover). Add `target` (e.g.
/// `"_blank"`) to open in a new tab. The same `active`, `disabled`, `class`, and
/// `onclick` props apply to both forms.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     DropdownItem { "Action" }                                  // <button>
///     DropdownItem { href: "/settings", "Settings" }             // <a href="/settings">
///     DropdownItem { href: "https://example.com", target: "_blank", "Docs" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct DropdownItemProps {
    /// Active state.
    #[props(default)]
    pub active: bool,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// When set, render an `<a class="dropdown-item" href=...>` anchor instead of
    /// a `<button>` so the item behaves as a real hyperlink. Anchors cannot be
    /// HTML-`disabled`, so a disabled anchor item carries the `.disabled` class,
    /// `aria-disabled="true"`, and `tabindex="-1"` (matching `NavLink`).
    #[props(default)]
    pub href: Option<String>,
    /// Anchor `target` (e.g. `"_blank"` to open in a new tab). Only applies when
    /// `href` is set.
    #[props(default)]
    pub target: Option<String>,
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

/// Build the class string for a dropdown item (`dropdown-item` + optional
/// `active`/`disabled` + caller classes). Shared by the `<button>` and `<a>`
/// render paths so both carry identical classes.
fn dropdown_item_class(active: bool, disabled: bool, class: &str) -> String {
    let mut classes = vec!["dropdown-item".to_string()];
    if active {
        classes.push("active".to_string());
    }
    if disabled {
        classes.push("disabled".to_string());
    }
    if !class.is_empty() {
        classes.push(class.to_string());
    }
    classes.join(" ")
}

#[component]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let full_class = dropdown_item_class(props.active, props.disabled, &props.class);

    // Anchor form: a real hyperlink so browser link behaviours work. Anchors
    // can't be HTML-`disabled`, so disabled state is conveyed by the `.disabled`
    // class plus `aria-disabled`/`tabindex="-1"` (matching NavLink).
    if let Some(href) = props.href.clone() {
        let target = props.target.clone();
        return rsx! {
            li {
                a {
                    class: "{full_class}",
                    href: "{href}",
                    target: target,
                    "aria-disabled": if props.disabled { "true" } else { "" },
                    tabindex: if props.disabled { "-1" } else { "" },
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    ..props.attributes,
                    {props.children}
                }
            }
        };
    }

    rsx! {
        li {
            button {
                class: "{full_class}",
                r#type: "button",
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

/// Dropdown menu divider.
#[component]
pub fn DropdownDivider() -> Element {
    rsx! {
        li { hr { class: "dropdown-divider" } }
    }
}

/// Dropdown menu header text.
#[derive(Clone, PartialEq, Props)]
pub struct DropdownHeaderProps {
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn DropdownHeader(props: DropdownHeaderProps) -> Element {
    rsx! {
        li { h6 { class: "dropdown-header", ..props.attributes, {props.children} } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_item_class_base() {
        assert_eq!(dropdown_item_class(false, false, ""), "dropdown-item");
    }

    #[test]
    fn dropdown_item_class_active_disabled_and_extra() {
        // Identical class output for the `<button>` and `<a>` render paths.
        assert_eq!(
            dropdown_item_class(true, true, "text-danger"),
            "dropdown-item active disabled text-danger"
        );
    }

    #[test]
    fn dropdown_item_class_active_only() {
        assert_eq!(dropdown_item_class(true, false, ""), "dropdown-item active");
    }
}
