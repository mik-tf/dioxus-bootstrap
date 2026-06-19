use dioxus::prelude::*;

/// Definition for a single tab.
///
/// Used with [`TabList`] to define tab labels, icons, and content.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> dioxus_bootstrap_css::tabs::TabDef {
/// use dioxus_bootstrap_css::tabs::TabDef;
///
/// TabDef {
///     label: "Home".into(),
///     icon: Some("house".into()),  // Bootstrap Icon name without "bi-" prefix
///     content: rsx! { p { "Home content" } },
/// }
/// # }
/// ```
#[derive(Clone, PartialEq)]
pub struct TabDef {
    /// Tab button label.
    pub label: String,
    /// Optional Bootstrap icon name (without "bi-" prefix).
    pub icon: Option<String>,
    /// Tab content.
    pub content: Element,
}

/// Bootstrap Tabs component — signal-driven, no JavaScript.
///
/// Produces pixel-perfect Bootstrap 5.3 HTML with separated `<ul class="nav nav-tabs">`
/// and `<div class="tab-content">` areas. This is the **recommended** component for tabs.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <ul class="nav nav-tabs" role="tablist">
///   <li class="nav-item"><button class="nav-link active">Home</button></li>
///   <li class="nav-item"><button class="nav-link">Profile</button></li>
/// </ul>
/// <div class="tab-content border border-top-0 rounded-bottom p-3">
///   <div class="tab-pane fade show active">Home content</div>
///   <div class="tab-pane fade">Profile content</div>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// use dioxus_bootstrap_css::tabs::TabDef;
///
/// let active = use_signal(|| 0usize);
/// rsx! {
///     TabList {
///         active: active,
///         content_class: "border border-top-0 rounded-bottom p-3",
///         tabs: vec![
///             TabDef { label: "Home".into(), icon: Some("house".into()),
///                 content: rsx! { p { "Home content" } } },
///             TabDef { label: "Profile".into(), icon: Some("person".into()),
///                 content: rsx! { p { "Profile content" } } },
///         ],
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `active` — `Signal<usize>` controlling active tab index
/// - `tabs` — `Vec<TabDef>` defining each tab's label, icon, and content
/// - `pills` — pill style instead of tabs
/// - `fill` — fill available width
/// - `justified` — equal-width items
/// - `content_class` — additional CSS classes for the tab-content div
///   (e.g., `"border border-top-0 rounded-bottom p-3"` for standard Bootstrap bordered tabs)
#[derive(Clone, PartialEq, Props)]
pub struct TabListProps {
    /// Signal controlling the active tab index.
    pub active: Signal<usize>,
    /// Tab definitions.
    pub tabs: Vec<TabDef>,
    /// Use pill style.
    #[props(default)]
    pub pills: bool,
    /// Fill available width.
    #[props(default)]
    pub fill: bool,
    /// Justify items equally.
    #[props(default)]
    pub justified: bool,
    /// Additional CSS classes for the nav.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the tab content area.
    #[props(default)]
    pub content_class: String,
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    let current = *props.active.read();
    let mut active_signal = props.active;
    let style = if props.pills { "nav-pills" } else { "nav-tabs" };

    let mut nav_classes = vec![format!("nav {style}")];
    if props.fill {
        nav_classes.push("nav-fill".to_string());
    }
    if props.justified {
        nav_classes.push("nav-justified".to_string());
    }
    if !props.class.is_empty() {
        nav_classes.push(props.class.clone());
    }
    let nav_class = nav_classes.join(" ");

    let content_class = if props.content_class.is_empty() {
        "tab-content".to_string()
    } else {
        format!("tab-content {}", props.content_class)
    };

    rsx! {
        ul { class: "{nav_class}", role: "tablist",
            for (i, tab) in props.tabs.iter().enumerate() {
                li { class: "nav-item", role: "presentation",
                    button {
                        class: if current == i { "nav-link active" } else { "nav-link" },
                        r#type: "button",
                        role: "tab",
                        "aria-selected": if current == i { "true" } else { "false" },
                        onclick: move |_| active_signal.set(i),
                        if let Some(ref icon) = tab.icon {
                            i { class: "bi bi-{icon} me-1" }
                        }
                        "{tab.label}"
                    }
                }
            }
        }
        div { class: "{content_class}",
            for (i, tab) in props.tabs.iter().enumerate() {
                div {
                    class: if current == i { "tab-pane fade show active" } else { "tab-pane fade" },
                    role: "tabpanel",
                    if current == i {
                        {tab.content.clone()}
                    }
                }
            }
        }
    }
}

/// Alias: `Tabs` works the same as `TabList`.
///
/// Both names produce identical output. `TabList` is the canonical name.
#[component]
pub fn Tabs(props: TabListProps) -> Element {
    TabList(props)
}

/// Alias for TabListProps.
pub type TabsProps = TabListProps;
