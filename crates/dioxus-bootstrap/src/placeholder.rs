use dioxus::prelude::*;

use crate::types::{Color, Size};

/// Bootstrap Placeholder component — loading skeleton.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<span class="placeholder col-6">` | `Placeholder { width: 6 }` |
/// | `<span class="placeholder col-8 bg-primary placeholder-lg">` | `Placeholder { width: 8, color: Color::Primary, size: Size::Lg }` |
/// | `<p class="placeholder-glow"><span class="placeholder col-7">` | `Placeholder { width: 7, glow: true }` |
/// | `<p class="placeholder-wave"><span class="placeholder col-5">` | `Placeholder { width: 5, wave: true }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Placeholder { width: 75 }
///     Placeholder { width: 50, color: Color::Primary, size: Size::Lg }
///     // Placeholder card
///     Card {
///         body: rsx! {
///             PlaceholderParagraph { lines: 3 }
///             Placeholder { width: 30, tag: "button" }
///         },
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct PlaceholderProps {
    /// Width percentage (1-100). Maps to Bootstrap's col-* classes.
    #[props(default = 100)]
    pub width: u8,
    /// Placeholder color.
    #[props(default)]
    pub color: Option<Color>,
    /// Placeholder size.
    #[props(default)]
    pub size: Size,
    /// Use wave animation.
    #[props(default)]
    pub wave: bool,
    /// Use glow animation.
    #[props(default)]
    pub glow: bool,
    /// HTML tag to render (default: "span").
    #[props(default = "span".to_string())]
    pub tag: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Placeholder(props: PlaceholderProps) -> Element {
    let col = format!("col-{}", props.width.min(12));
    let color_class = match &props.color {
        Some(c) => format!(" bg-{c}"),
        None => String::new(),
    };
    let size_class = match props.size {
        Size::Sm => " placeholder-sm",
        Size::Lg => " placeholder-lg",
        Size::Md => "",
    };

    let full_class = if props.class.is_empty() {
        format!("placeholder {col}{color_class}{size_class}")
    } else {
        format!("placeholder {col}{color_class}{size_class} {}", props.class)
    };

    let animation = if props.wave {
        "placeholder-wave"
    } else if props.glow {
        "placeholder-glow"
    } else {
        ""
    };

    if animation.is_empty() {
        rsx! {
            span { class: "{full_class}", "aria-hidden": "true" }
        }
    } else {
        rsx! {
            p { class: "{animation}",
                span { class: "{full_class}", "aria-hidden": "true" }
            }
        }
    }
}

/// Quick placeholder paragraph with multiple lines.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     PlaceholderParagraph { lines: 3, glow: true }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct PlaceholderParagraphProps {
    /// Number of placeholder lines.
    #[props(default = 3)]
    pub lines: usize,
    /// Use wave animation.
    #[props(default)]
    pub wave: bool,
    /// Use glow animation.
    #[props(default)]
    pub glow: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn PlaceholderParagraph(props: PlaceholderParagraphProps) -> Element {
    let animation = if props.wave {
        " placeholder-wave"
    } else if props.glow {
        " placeholder-glow"
    } else {
        ""
    };

    let full_class = if props.class.is_empty() {
        animation.trim().to_string()
    } else {
        format!("{} {}", animation.trim(), props.class)
    };

    // Vary widths for a natural look
    let widths = [7, 9, 6, 8, 5, 10, 7, 4, 11, 6];

    rsx! {
        p { class: "{full_class}",
            for i in 0..props.lines {
                {
                    let w = widths[i % widths.len()];
                    let cls = format!("placeholder col-{w}");
                    rsx! {
                        span {
                            class: "{cls}",
                            "aria-hidden": "true",
                        }
                        " "
                    }
                }
            }
        }
    }
}
