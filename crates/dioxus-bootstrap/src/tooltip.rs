use dioxus::prelude::*;

/// Tooltip placement.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Start,
    End,
}

/// Bootstrap Tooltip component — CSS-positioned, no JavaScript.
///
/// Shows a tooltip on hover. Uses CSS-based positioning relative to the trigger element.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript + Popper.js) -->
/// <button data-bs-toggle="tooltip" data-bs-placement="top" title="Tooltip text">Hover me</button>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent — no JavaScript needed
/// rsx! {
///     Tooltip { text: "Save your work", placement: TooltipPlacement::Top,
///         Button { color: Color::Primary, "Save" }
///     }
///     Tooltip { text: "More info", placement: TooltipPlacement::End,
///         Icon { name: "info-circle" }
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `text` — tooltip text content
/// - `placement` — `TooltipPlacement::Top`, `Bottom`, `Start`, `End`
#[derive(Clone, PartialEq, Props)]
pub struct TooltipProps {
    /// Tooltip text content.
    pub text: String,
    /// Tooltip placement relative to the trigger element.
    #[props(default)]
    pub placement: TooltipPlacement,
    /// Additional CSS classes for the tooltip.
    #[props(default)]
    pub class: String,
    /// Child element (the trigger).
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let hovering = use_signal(|| false);
    let is_hovering = *hovering.read();
    let mut hover_signal = hovering;

    let placement_class = match props.placement {
        TooltipPlacement::Top => "bs-tooltip-top",
        TooltipPlacement::Bottom => "bs-tooltip-bottom",
        TooltipPlacement::Start => "bs-tooltip-start",
        TooltipPlacement::End => "bs-tooltip-end",
    };

    let tooltip_class = if props.class.is_empty() {
        format!("tooltip fade {placement_class} show")
    } else {
        format!("tooltip fade {placement_class} show {}", props.class)
    };

    // Position styles based on placement
    let position_style = match props.placement {
        TooltipPlacement::Top => {
            "position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 0.4rem;"
        }
        TooltipPlacement::Bottom => {
            "position: absolute; top: 100%; left: 50%; transform: translateX(-50%); margin-top: 0.4rem;"
        }
        TooltipPlacement::Start => {
            "position: absolute; right: 100%; top: 50%; transform: translateY(-50%); margin-right: 0.4rem;"
        }
        TooltipPlacement::End => {
            "position: absolute; left: 100%; top: 50%; transform: translateY(-50%); margin-left: 0.4rem;"
        }
    };

    let arrow_placement = match props.placement {
        TooltipPlacement::Top => "bottom: -6px; left: 50%; transform: translateX(-50%);",
        TooltipPlacement::Bottom => "top: -6px; left: 50%; transform: translateX(-50%);",
        TooltipPlacement::Start => "right: -6px; top: 50%; transform: translateY(-50%);",
        TooltipPlacement::End => "left: -6px; top: 50%; transform: translateY(-50%);",
    };

    rsx! {
        div {
            style: "position: relative; display: inline-block;",
            onmouseenter: move |_| hover_signal.set(true),
            onmouseleave: move |_| hover_signal.set(false),
            {props.children}
            if is_hovering {
                div {
                    class: "{tooltip_class}",
                    role: "tooltip",
                    style: "{position_style} z-index: 1080; pointer-events: none; white-space: nowrap;",
                    div { class: "tooltip-arrow", style: "position: absolute; {arrow_placement}" }
                    div { class: "tooltip-inner", "{props.text}" }
                }
            }
        }
    }
}
