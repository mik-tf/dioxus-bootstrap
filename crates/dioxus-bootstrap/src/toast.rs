use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap Toast notification — signal-driven, no JavaScript.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript) -->
/// <div class="toast show">
///   <div class="toast-header">
///     <strong class="me-auto">Notification</strong>
///     <small>just now</small>
///     <button class="btn-close" data-bs-dismiss="toast"></button>
///   </div>
///   <div class="toast-body">You have a new message.</div>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent
/// let show = use_signal(|| true);
/// rsx! {
///     ToastContainer { position: ToastPosition::TopEnd,
///         Toast { show: show, title: "Notification", subtitle: "just now",
///             "You have a new message."
///         }
///     }
/// }
/// # }
/// ```
///
/// # Headerless Mode
///
/// Omit `title` and set `show_close: true` to render a headerless toast with
/// a side-aligned close button (Bootstrap 5.3 `d-flex` pattern):
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// # let signal = use_signal(|| true);
/// rsx! {
///     Toast { show: signal, show_close: true, color: Color::Primary,
///         "Body-only toast with close button."
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `show` — `Signal<bool>` controlling visibility
/// - `title` — toast header title (omit for headerless mode)
/// - `subtitle` — small text in header (e.g., "just now")
/// - `color` — background color variant
/// - `show_close` — show close button (default: true)
/// - `autohide` — auto-dismiss after `delay_ms` (default: false)
/// - `delay_ms` — auto-dismiss delay in milliseconds (default: 5000)
/// - `on_dismiss` — callback when the toast is dismissed
#[derive(Clone, PartialEq, Props)]
pub struct ToastProps {
    /// Signal controlling visibility.
    pub show: Signal<bool>,
    /// Toast title (shown in header).
    #[props(default)]
    pub title: String,
    /// Small text in header (e.g., "just now", "2 mins ago").
    #[props(default)]
    pub subtitle: String,
    /// Show close button.
    #[props(default = true)]
    pub show_close: bool,
    /// Auto-dismiss the toast after `delay_ms` (Bootstrap's `autohide` option).
    #[props(default)]
    pub autohide: bool,
    /// Auto-dismiss delay in milliseconds (Bootstrap's `delay` option).
    #[props(default = 5000)]
    pub delay_ms: u32,
    /// Toast color variant (applied as bg class).
    #[props(default)]
    pub color: Option<Color>,
    /// Callback when the toast is dismissed.
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Toast body content.
    pub children: Element,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let mut show_signal = props.show;
    let on_dismiss = props.on_dismiss;
    let autohide = props.autohide;
    let delay_ms = props.delay_ms;

    // Autohide — when the toast is shown and autohide is on, close it after
    // `delay_ms`, matching Bootstrap's `autohide` + `delay`. Runs before the
    // early return so the hook count is stable across shown/hidden renders.
    use_effect(move || {
        let shown = *show_signal.read();
        if autohide && shown {
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(delay_ms).await;
                // The toast may have been dismissed already while we waited.
                if *show_signal.peek() {
                    show_signal.set(false);
                    if let Some(handler) = &on_dismiss {
                        handler.call(());
                    }
                }
            });
        }
    });

    let is_shown = *show_signal.read();
    if !is_shown {
        return rsx! {};
    }

    let dismiss = move |_| {
        show_signal.set(false);
        if let Some(handler) = &on_dismiss {
            handler.call(());
        }
    };

    let color_class = match &props.color {
        Some(c) => format!(" text-bg-{c}"),
        None => String::new(),
    };

    let full_class = if props.class.is_empty() {
        format!("toast show{color_class}")
    } else {
        format!("toast show{color_class} {}", props.class)
    };

    // Determine close button class — use white variant for colored toasts
    let close_class = if props.color.is_some() {
        "btn-close btn-close-white me-2 m-auto"
    } else {
        "btn-close"
    };

    rsx! {
        div {
            class: "{full_class}",
            role: "alert",
            "aria-live": "assertive",
            "aria-atomic": "true",
            if !props.title.is_empty() {
                // Header mode: title + subtitle + close button
                div { class: "toast-header",
                    strong { class: "me-auto", "{props.title}" }
                    if !props.subtitle.is_empty() {
                        small { "{props.subtitle}" }
                    }
                    if props.show_close {
                        button {
                            class: "btn-close",
                            r#type: "button",
                            "aria-label": "Close",
                            onclick: dismiss,
                        }
                    }
                }
                div { class: "toast-body", {props.children} }
            } else if props.show_close {
                // Headerless mode with close button: d-flex layout (Bootstrap 5.3 pattern)
                div { class: "d-flex",
                    div { class: "toast-body", {props.children} }
                    button {
                        class: "{close_class}",
                        r#type: "button",
                        "aria-label": "Close",
                        onclick: move |_| show_signal.set(false),
                    }
                }
            } else {
                // Simple body-only toast
                div { class: "toast-body", {props.children} }
            }
        }
    }
}

/// Container for positioning toasts on screen.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// # let signal1 = use_signal(|| true);
/// # let signal2 = use_signal(|| true);
/// rsx! {
///     ToastContainer { position: ToastPosition::TopEnd,
///         Toast { show: signal1, title: "Success", "Saved!" }
///         Toast { show: signal2, title: "Error", color: Color::Danger, "Failed." }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ToastContainerProps {
    /// Position on screen.
    #[props(default)]
    pub position: ToastPosition,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child elements (Toast components).
    pub children: Element,
}

/// Toast position on screen.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ToastPosition {
    TopStart,
    TopCenter,
    #[default]
    TopEnd,
    MiddleCenter,
    BottomStart,
    BottomCenter,
    BottomEnd,
}

impl std::fmt::Display for ToastPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToastPosition::TopStart => write!(f, "top-0 start-0"),
            ToastPosition::TopCenter => write!(f, "top-0 start-50 translate-middle-x"),
            ToastPosition::TopEnd => write!(f, "top-0 end-0"),
            ToastPosition::MiddleCenter => {
                write!(f, "top-50 start-50 translate-middle")
            }
            ToastPosition::BottomStart => write!(f, "bottom-0 start-0"),
            ToastPosition::BottomCenter => {
                write!(f, "bottom-0 start-50 translate-middle-x")
            }
            ToastPosition::BottomEnd => write!(f, "bottom-0 end-0"),
        }
    }
}

#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    let pos = props.position;
    let full_class = if props.class.is_empty() {
        format!("toast-container position-fixed p-3 {pos}")
    } else {
        format!("toast-container position-fixed p-3 {pos} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", {props.children} }
    }
}
