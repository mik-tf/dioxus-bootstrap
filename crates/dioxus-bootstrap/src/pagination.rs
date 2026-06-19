use dioxus::prelude::*;

use crate::types::Size;

/// Bootstrap Pagination component — signal-driven, no JavaScript.
///
/// Automatically generates page numbers with ellipsis, prev/next buttons,
/// and highlights the active page.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (manual) -->
/// <nav><ul class="pagination pagination-sm">
///   <li class="page-item"><button class="page-link">‹</button></li>
///   <li class="page-item active"><button class="page-link">1</button></li>
///   <li class="page-item"><button class="page-link">2</button></li>
///   <li class="page-item"><button class="page-link">›</button></li>
/// </ul></nav>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus equivalent — fully automatic
/// let page = use_signal(|| 1usize);
/// rsx! {
///     Pagination { current: page, total: 20, window: 2, size: Size::Sm }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `current` — `Signal<usize>` for current page (1-based)
/// - `total` — total number of pages
/// - `window` — number of page links around current (default: 2)
/// - `size` — `Size::Sm`, `Md`, `Lg`
/// - `show_prev_next` — show prev/next buttons (default: true)
#[derive(Clone, PartialEq, Props)]
pub struct PaginationProps {
    /// Signal controlling the current page (1-based).
    pub current: Signal<usize>,
    /// Total number of pages.
    pub total: usize,
    /// Number of page links to show around the current page.
    #[props(default = 2)]
    pub window: usize,
    /// Pagination size.
    #[props(default)]
    pub size: Size,
    /// Show previous/next buttons.
    #[props(default = true)]
    pub show_prev_next: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let current = *props.current.read();
    let mut page_signal = props.current;
    let total = props.total;

    if total == 0 {
        return rsx! {};
    }

    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" pagination-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("pagination{size_class}")
    } else {
        format!("pagination{size_class} {}", props.class)
    };

    // Calculate visible page range
    let start = if current > props.window {
        current - props.window
    } else {
        1
    };
    let end = if current + props.window <= total {
        current + props.window
    } else {
        total
    };

    rsx! {
        nav { "aria-label": "Page navigation",
            ul { class: "{full_class}",
                // Previous
                if props.show_prev_next {
                    li { class: if current <= 1 { "page-item disabled" } else { "page-item" },
                        button {
                            class: "page-link",
                            disabled: current <= 1,
                            onclick: move |_| {
                                if current > 1 {
                                    page_signal.set(current - 1);
                                }
                            },
                            "aria-label": "Previous",
                            span { "aria-hidden": "true", "\u{2039}" }
                        }
                    }
                }

                // First page + ellipsis
                if start > 1 {
                    li { class: "page-item",
                        button {
                            class: "page-link",
                            onclick: move |_| page_signal.set(1),
                            "1"
                        }
                    }
                    if start > 2 {
                        li { class: "page-item disabled",
                            span { class: "page-link", "\u{2026}" }
                        }
                    }
                }

                // Page numbers
                for p in start..=end {
                    li {
                        class: if p == current { "page-item active" } else { "page-item" },
                        button {
                            class: "page-link",
                            "aria-current": if p == current { "page" } else { "" },
                            onclick: move |_| page_signal.set(p),
                            "{p}"
                        }
                    }
                }

                // Last page + ellipsis
                if end < total {
                    if end < total - 1 {
                        li { class: "page-item disabled",
                            span { class: "page-link", "\u{2026}" }
                        }
                    }
                    li { class: "page-item",
                        button {
                            class: "page-link",
                            onclick: move |_| page_signal.set(total),
                            "{total}"
                        }
                    }
                }

                // Next
                if props.show_prev_next {
                    li { class: if current >= total { "page-item disabled" } else { "page-item" },
                        button {
                            class: "page-link",
                            disabled: current >= total,
                            onclick: move |_| {
                                if current < total {
                                    page_signal.set(current + 1);
                                }
                            },
                            "aria-label": "Next",
                            span { "aria-hidden": "true", "\u{203A}" }
                        }
                    }
                }
            }
        }
    }
}
