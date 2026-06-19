use dioxus::prelude::*;

use crate::types::ModalSize;

/// Modal fullscreen variants.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ModalFullscreen {
    /// Not fullscreen.
    #[default]
    Off,
    /// Always fullscreen.
    Always,
    /// Fullscreen below sm breakpoint.
    SmDown,
    /// Fullscreen below md breakpoint.
    MdDown,
    /// Fullscreen below lg breakpoint.
    LgDown,
    /// Fullscreen below xl breakpoint.
    XlDown,
    /// Fullscreen below xxl breakpoint.
    XxlDown,
}

/// Bootstrap Modal component — signal-driven, no JavaScript.
///
/// Replaces Bootstrap's `<div class="modal">` + JavaScript with a signal-controlled component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (requires JavaScript) -->
/// <div class="modal fade" tabindex="-1">
///   <div class="modal-dialog modal-lg modal-dialog-centered">
///     <div class="modal-content">
///       <div class="modal-header"><h5 class="modal-title">Title</h5></div>
///       <div class="modal-body"><p>Body</p></div>
///       <div class="modal-footer"><button class="btn btn-primary">OK</button></div>
///     </div>
///   </div>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent — no JavaScript needed
/// let mut show = use_signal(|| false);
/// rsx! {
///     Button { onclick: move |_| show.set(true), "Open Modal" }
///     Modal {
///         show: show,
///         title: "Confirm Action",
///         size: ModalSize::Lg,
///         centered: true,
///         body: rsx! { p { "Are you sure?" } },
///         footer: rsx! {
///             Button { color: Color::Secondary, onclick: move |_| show.set(false), "Cancel" }
///             Button { color: Color::Primary, "Confirm" }
///         },
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `show` — `Signal<bool>` controlling visibility
/// - `title` — modal title text
/// - `body` — modal body content (Element)
/// - `footer` — modal footer content (Element)
/// - `size` — `ModalSize::Sm`, `Default`, `Lg`, `Xl`
/// - `fullscreen` — `ModalFullscreen::Off`, `Always`, `SmDown`..`XxlDown`
/// - `centered` — vertically center the modal
/// - `scrollable` — scrollable modal body
/// - `backdrop_close` — close when clicking backdrop (default: true)
#[derive(Clone, PartialEq, Props)]
pub struct ModalProps {
    /// Signal controlling modal visibility.
    pub show: Signal<bool>,
    /// Modal title.
    #[props(default)]
    pub title: String,
    /// Modal body content.
    #[props(default)]
    pub body: Option<Element>,
    /// Modal footer content.
    #[props(default)]
    pub footer: Option<Element>,
    /// Modal size.
    #[props(default)]
    pub size: ModalSize,
    /// Close when clicking the backdrop.
    #[props(default = true)]
    pub backdrop_close: bool,
    /// Show the close button in the header.
    #[props(default = true)]
    pub show_close: bool,
    /// Center the modal vertically.
    #[props(default)]
    pub centered: bool,
    /// Allow the modal body to scroll.
    #[props(default)]
    pub scrollable: bool,
    /// Fullscreen mode.
    #[props(default)]
    pub fullscreen: ModalFullscreen,
    /// Additional CSS classes for the modal-dialog.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (alternative to body prop for custom layout).
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    let is_shown = *props.show.read();
    let mut show_signal = props.show;

    if !is_shown {
        return rsx! {};
    }

    let size_class = match props.size {
        ModalSize::Sm => " modal-sm",
        ModalSize::Default => "",
        ModalSize::Lg => " modal-lg",
        ModalSize::Xl => " modal-xl",
    };

    let centered = if props.centered {
        " modal-dialog-centered"
    } else {
        ""
    };

    let scrollable = if props.scrollable {
        " modal-dialog-scrollable"
    } else {
        ""
    };

    let fullscreen = match props.fullscreen {
        ModalFullscreen::Off => "",
        ModalFullscreen::Always => " modal-fullscreen",
        ModalFullscreen::SmDown => " modal-fullscreen-sm-down",
        ModalFullscreen::MdDown => " modal-fullscreen-md-down",
        ModalFullscreen::LgDown => " modal-fullscreen-lg-down",
        ModalFullscreen::XlDown => " modal-fullscreen-xl-down",
        ModalFullscreen::XxlDown => " modal-fullscreen-xxl-down",
    };

    let dialog_class = if props.class.is_empty() {
        format!("modal-dialog{size_class}{centered}{scrollable}{fullscreen}")
    } else {
        format!(
            "modal-dialog{size_class}{centered}{scrollable}{fullscreen} {}",
            props.class
        )
    };

    let backdrop_close = props.backdrop_close;

    rsx! {
        // Backdrop
        div {
            class: "modal-backdrop fade show",
            onclick: move |_| {
                if backdrop_close {
                    show_signal.set(false);
                }
            },
        }
        // Modal
        div {
            class: "modal fade show",
            style: "display: block;",
            tabindex: "-1",
            role: "dialog",
            "aria-modal": "true",
            onclick: move |_| {
                if backdrop_close {
                    show_signal.set(false);
                }
            },
            ..props.attributes,
            div {
                class: "{dialog_class}",
                // Stop click propagation so clicking inside the modal doesn't close it
                onclick: move |evt| evt.stop_propagation(),
                div { class: "modal-content",
                    // Header
                    if !props.title.is_empty() || props.show_close {
                        div { class: "modal-header",
                            if !props.title.is_empty() {
                                h5 { class: "modal-title", "{props.title}" }
                            }
                            if props.show_close {
                                button {
                                    class: "btn-close",
                                    r#type: "button",
                                    "aria-label": "Close",
                                    onclick: move |_| show_signal.set(false),
                                }
                            }
                        }
                    }
                    // Body
                    if let Some(body) = props.body {
                        div { class: "modal-body", {body} }
                    }
                    {props.children}
                    // Footer
                    if let Some(footer) = props.footer {
                        div { class: "modal-footer", {footer} }
                    }
                }
            }
        }
    }
}
