use dioxus::prelude::*;

/// Bootstrap Scrollspy — tracks scroll position and updates active signal.
///
/// Place this component in your layout. It watches the given target element
/// (by CSS selector) and updates the `active` signal with the `id` of the
/// currently visible section.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<body data-bs-spy="scroll" data-bs-target="#nav" data-bs-offset="80">` | `Scrollspy { target: "body", active: signal, offset: 80 }` |
/// | Check active section via JS | `if *active.read() == "intro" { "active" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// let active_section = use_signal(|| String::new());
/// rsx! {
///     Scrollspy { target: "main", active: active_section, offset: 80 }
///     nav {
///         a { class: if *active_section.read() == "intro" { "nav-link active" } else { "nav-link" },
///             href: "#intro", "Intro" }
///         a { class: if *active_section.read() == "features" { "nav-link active" } else { "nav-link" },
///             href: "#features", "Features" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ScrollspyProps {
    /// CSS selector for the scrollable container (e.g., "main", "#content", "body").
    #[props(default = "body".to_string())]
    pub target: String,
    /// Signal that receives the `id` of the currently active section.
    pub active: Signal<String>,
    /// Offset in pixels from the top (useful for fixed/sticky navbars).
    #[props(default = 0)]
    pub offset: i32,
}

#[component]
pub fn Scrollspy(props: ScrollspyProps) -> Element {
    let mut active_signal = props.active;
    let target = props.target.clone();
    let offset = props.offset;

    // Set up IntersectionObserver via eval to track which [id] section is visible
    use_effect(move || {
        let target = target.clone();
        document::eval(&format!(
            r#"
            (function() {{
                var container = document.querySelector('{target}');
                if (!container || container === document.body) container = document;
                var sections = document.querySelectorAll('[id]');
                if (sections.length === 0) return;

                function update() {{
                    var scrollTop = (container === document)
                        ? window.scrollY || document.documentElement.scrollTop
                        : container.scrollTop;
                    var offset = {offset};
                    var active = '';
                    sections.forEach(function(section) {{
                        var rect = section.getBoundingClientRect();
                        if (rect.top <= offset + 10) {{
                            active = section.id;
                        }}
                    }});
                    if (active && window.__dioxus_scrollspy_active !== active) {{
                        window.__dioxus_scrollspy_active = active;
                        // Dispatch a custom event that Dioxus can listen to
                        window.dispatchEvent(new CustomEvent('scrollspy', {{ detail: active }}));
                    }}
                }}

                var scrollTarget = (container === document) ? window : container;
                scrollTarget.addEventListener('scroll', update, {{ passive: true }});
                update();
            }})();
            "#
        ));
    });

    // Listen for scrollspy events via polling with eval
    // Note: This uses a simple approach — the JS side updates a global,
    // and we read it periodically via use_effect
    use_effect(move || {
        let eval_handle = document::eval(
            r#"
            (function() {
                return new Promise(function(resolve) {
                    var current = window.__dioxus_scrollspy_active || '';
                    // Set up listener for changes
                    window.addEventListener('scrollspy', function handler(e) {
                        resolve(e.detail);
                        window.removeEventListener('scrollspy', handler);
                    });
                    // If already set, resolve immediately
                    if (current) resolve(current);
                });
            })()
            "#,
        );

        spawn(async move {
            if let Ok(value) = eval_handle.await {
                if let Some(id) = value.as_str() {
                    active_signal.set(id.to_string());
                }
            }
        });
    });

    rsx! {}
}
