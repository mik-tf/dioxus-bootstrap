use dioxus::prelude::*;

/// Popover placement.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PopoverPlacement {
    #[default]
    Top,
    Bottom,
    Start,
    End,
}

/// Bootstrap Popover component — CSS-positioned, no JavaScript.
///
/// Shows a popover with title and body on click. Click outside to close.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript + Popper.js) -->
/// <button data-bs-toggle="popover" title="Title" data-bs-content="Content">Click</button>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent — no JavaScript needed
/// rsx! {
///     Popover {
///         title: "Popover Title",
///         body: rsx! { p { "Rich content here." } },
///         placement: PopoverPlacement::Top,
///         Button { color: Color::Info, "Click for details" }
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `title` — popover header text
/// - `body` — popover body content (Element)
/// - `placement` — `PopoverPlacement::Top`, `Bottom`, `Start`, `End`
#[derive(Clone, PartialEq, Props)]
pub struct PopoverProps {
    /// Popover title (header).
    #[props(default)]
    pub title: String,
    /// Popover body content.
    pub body: Element,
    /// Popover placement relative to the trigger element.
    #[props(default)]
    pub placement: PopoverPlacement,
    /// Additional CSS classes for the popover.
    #[props(default)]
    pub class: String,
    /// Child element (the trigger).
    pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let open = use_signal(|| false);
    let is_open = *open.read();
    let mut open_signal = open;

    let placement_class = match props.placement {
        PopoverPlacement::Top => "bs-popover-top",
        PopoverPlacement::Bottom => "bs-popover-bottom",
        PopoverPlacement::Start => "bs-popover-start",
        PopoverPlacement::End => "bs-popover-end",
    };

    let popover_class = if props.class.is_empty() {
        format!("popover fade {placement_class} show")
    } else {
        format!("popover fade {placement_class} show {}", props.class)
    };

    let position_style = match props.placement {
        PopoverPlacement::Top => {
            "position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 0.5rem;"
        }
        PopoverPlacement::Bottom => {
            "position: absolute; top: 100%; left: 50%; transform: translateX(-50%); margin-top: 0.5rem;"
        }
        PopoverPlacement::Start => {
            "position: absolute; right: 100%; top: 50%; transform: translateY(-50%); margin-right: 0.5rem;"
        }
        PopoverPlacement::End => {
            "position: absolute; left: 100%; top: 50%; transform: translateY(-50%); margin-left: 0.5rem;"
        }
    };

    let arrow_placement = match props.placement {
        PopoverPlacement::Top => {
            "bottom: calc(-0.5rem - 1px); left: 50%; transform: translateX(-50%);"
        }
        PopoverPlacement::Bottom => {
            "top: calc(-0.5rem - 1px); left: 50%; transform: translateX(-50%);"
        }
        PopoverPlacement::Start => {
            "right: calc(-0.5rem - 1px); top: 50%; transform: translateY(-50%);"
        }
        PopoverPlacement::End => {
            "left: calc(-0.5rem - 1px); top: 50%; transform: translateY(-50%);"
        }
    };

    rsx! {
        // Overlay to close on outside click
        if is_open {
            div {
                style: "position: fixed; inset: 0; z-index: 1069;",
                onclick: move |_| open_signal.set(false),
            }
        }
        div {
            style: if is_open { "position: relative; display: inline-block; z-index: 1070;" } else { "position: relative; display: inline-block;" },
            onclick: move |evt| {
                evt.stop_propagation();
                open_signal.set(!is_open);
            },
            {props.children}
            if is_open {
                div {
                    class: "{popover_class}",
                    role: "tooltip",
                    style: "{position_style} z-index: 1070; min-width: 200px;",
                    onclick: move |evt| evt.stop_propagation(),
                    div { class: "popover-arrow", style: "position: absolute; {arrow_placement}" }
                    if !props.title.is_empty() {
                        h3 { class: "popover-header", "{props.title}" }
                    }
                    div { class: "popover-body", {props.body} }
                }
            }
        }
    }
}
