use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap Progress container.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="progress">
///   <div class="progress-bar bg-success" style="width: 75%">75%</div>
/// </div>
/// <!-- Stacked bars -->
/// <div class="progress">
///   <div class="progress-bar" style="width: 30%"></div>
///   <div class="progress-bar bg-warning" style="width: 20%"></div>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Progress {
///         ProgressBar { value: 75.0, color: Color::Success, show_label: true }
///     }
///     // Stacked bars
///     Progress {
///         ProgressBar { value: 30.0, color: Color::Primary }
///         ProgressBar { value: 20.0, color: Color::Warning }
///     }
///     // Striped animated
///     Progress {
///         ProgressBar { value: 50.0, striped: true, animated: true }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ProgressProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (ProgressBar components).
    pub children: Element,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let full_class = if props.class.is_empty() {
        "progress".to_string()
    } else {
        format!("progress {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap ProgressBar component (goes inside Progress).
#[derive(Clone, PartialEq, Props)]
pub struct ProgressBarProps {
    /// Progress value (0.0 to 100.0).
    #[props(default)]
    pub value: f64,
    /// Bar color.
    #[props(default)]
    pub color: Option<Color>,
    /// Show striped pattern.
    #[props(default)]
    pub striped: bool,
    /// Animate the stripes.
    #[props(default)]
    pub animated: bool,
    /// Show the value as text inside the bar.
    #[props(default)]
    pub show_label: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let color_class = match &props.color {
        Some(c) => format!(" bg-{c}"),
        None => String::new(),
    };
    let striped = if props.striped || props.animated {
        " progress-bar-striped"
    } else {
        ""
    };
    let animated = if props.animated {
        " progress-bar-animated"
    } else {
        ""
    };

    let full_class = if props.class.is_empty() {
        format!("progress-bar{color_class}{striped}{animated}")
    } else {
        format!(
            "progress-bar{color_class}{striped}{animated} {}",
            props.class
        )
    };

    let width = format!("width: {}%", props.value);
    let label = if props.show_label {
        format!("{}%", props.value as u32)
    } else {
        String::new()
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "progressbar",
            style: "{width}",
            "aria-valuenow": "{props.value}",
            "aria-valuemin": "0",
            "aria-valuemax": "100",
            ..props.attributes,
            "{label}"
        }
    }
}
