use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static NEXT_SCROLLSPY_ID: AtomicUsize = AtomicUsize::new(1);

const DEFAULT_ROOT_MARGIN: &str = "0px 0px -25%";
const DEFAULT_THRESHOLD: [f64; 3] = [0.1, 0.5, 1.0];

/// Bootstrap Scrollspy tracks scroll position and updates active section state.
///
/// The `target` prop follows Bootstrap semantics: it points to the nav, list
/// group, or simple links container whose links reference section ids. Use
/// `root` for the body or custom scroll container being observed.
///
/// # Bootstrap HTML -> Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<body data-bs-spy="scroll" data-bs-target="#nav">` | `Scrollspy { target: "#nav", active: signal }` |
/// | `<div data-bs-spy="scroll" data-bs-target="#nav" tabindex="0">` | `Scrollspy { target: "#nav", root: "#scroll-area", active: signal }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// let active_section = use_signal(|| String::new());
/// rsx! {
///     Scrollspy {
///         target: "#docs-nav",
///         root: "#docs-scroll",
///         active: active_section,
///         offset: 80,
///     }
///     nav { id: "docs-nav",
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
    /// CSS selector for nav/list/simple links container.
    #[props(default = "body".to_string())]
    pub target: String,
    /// CSS selector for scroll container. Use `"body"` for viewport scroll.
    #[props(default = "body".to_string())]
    pub root: String,
    /// Signal receives id currently active section.
    pub active: Signal<String>,
    /// Compatibility offset in pixels for fixed or sticky headers.
    #[props(default = 0)]
    pub offset: i32,
    /// IntersectionObserver root margin.
    #[props(default = DEFAULT_ROOT_MARGIN.to_string())]
    pub root_margin: String,
    /// IntersectionObserver thresholds.
    #[props(default = DEFAULT_THRESHOLD.to_vec())]
    pub threshold: Vec<f64>,
    /// Change this value to force section/link discovery refresh.
    #[props(default = 0)]
    pub refresh_key: u64,
    /// Smooth-scroll target links owned by `target`.
    #[props(default = false)]
    pub smooth_scroll: bool,
}

#[component]
pub fn Scrollspy(props: ScrollspyProps) -> Element {
    let instance_id = use_signal(next_scrollspy_id);
    let mut listener_started = use_signal(|| false);
    let cleanup_id = instance_id.read().clone();

    let setup = ScrollspySetup {
        instance_id: instance_id.read().clone(),
        target: props.target.clone(),
        root: props.root.clone(),
        offset: props.offset,
        root_margin: props.root_margin.clone(),
        threshold: props.threshold.clone(),
        refresh_key: props.refresh_key,
        smooth_scroll: props.smooth_scroll,
    };

    use_drop(move || cleanup_scrollspy(cleanup_id));

    use_effect(use_reactive(
        (
            &props.target,
            &props.root,
            &props.offset,
            &props.root_margin,
            &props.threshold,
            &props.refresh_key,
            &props.smooth_scroll,
        ),
        move |_| setup_scrollspy(setup.clone()),
    ));

    use_effect(move || {
        if !*listener_started.read() {
            listener_started.set(true);
            listen_scrollspy_events(instance_id.read().clone(), props.active);
        }
    });

    rsx! {}
}

#[derive(Clone)]
struct ScrollspySetup {
    instance_id: String,
    target: String,
    root: String,
    offset: i32,
    root_margin: String,
    threshold: Vec<f64>,
    refresh_key: u64,
    smooth_scroll: bool,
}

fn next_scrollspy_id() -> String {
    let id = NEXT_SCROLLSPY_ID.fetch_add(1, Ordering::Relaxed);
    format!("dbcss-scrollspy-{id}")
}

fn setup_scrollspy(setup: ScrollspySetup) {
    spawn(async move {
        let script = SCROLLSPY_SETUP_SCRIPT
            .replace("__ID__", &js_string(&setup.instance_id))
            .replace("__TARGET__", &js_string(&setup.target))
            .replace("__ROOT__", &js_string(&setup.root))
            .replace("__ROOT_MARGIN__", &js_string(&setup.root_margin))
            .replace("__THRESHOLD__", &threshold_js(&setup.threshold))
            .replace("__OFFSET__", &setup.offset.to_string())
            .replace(
                "__SMOOTH_SCROLL__",
                if setup.smooth_scroll { "true" } else { "false" },
            )
            .replace("__REFRESH_KEY__", &setup.refresh_key.to_string());

        let _ = document::eval(&script).await;
    });
}

fn listen_scrollspy_events(instance_id: String, mut active: Signal<String>) {
    spawn(async move {
        let mut last = String::new();

        loop {
            let script = SCROLLSPY_EVENT_SCRIPT
                .replace("__ID__", &js_string(&instance_id))
                .replace("__LAST__", &js_string(&last));

            let Ok(value) = document::eval(&script).await else {
                break;
            };

            let Some(next) = value.as_str() else {
                continue;
            };

            if next != last {
                last = next.to_string();
                active.set(last.clone());
            }
        }
    });
}

fn cleanup_scrollspy(instance_id: String) {
    let script = SCROLLSPY_CLEANUP_SCRIPT.replace("__ID__", &js_string(&instance_id));
    let _ = document::eval(&script);
}

fn js_string(value: &str) -> String {
    format!("{value:?}")
}

fn threshold_js(threshold: &[f64]) -> String {
    let mut values = threshold
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .map(|value| value.clamp(0.0, 1.0))
        .collect::<Vec<_>>();

    if values.is_empty() {
        values = DEFAULT_THRESHOLD.to_vec();
    }

    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("{value:.3}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}

const SCROLLSPY_SETUP_SCRIPT: &str = r##"
(function() {
    const id = __ID__;
    const targetSelector = __TARGET__;
    const rootSelector = __ROOT__;
    const rootMargin = __ROOT_MARGIN__;
    const threshold = __THRESHOLD__;
    const offset = __OFFSET__;
    const smoothScroll = __SMOOTH_SCROLL__;
    const refreshKey = __REFRESH_KEY__;
    const eventName = "dbcss:scrollspy:" + id;

    window.__dbcssScrollspy = window.__dbcssScrollspy || {};
    const previous = window.__dbcssScrollspy[id];
    if (previous && typeof previous.cleanup === "function") {
        previous.cleanup();
    }

    const state = {
        active: "",
        refreshKey,
        cleanup: function() {}
    };
    window.__dbcssScrollspy[id] = state;

    function resolveRoot(selector) {
        const normalized = String(selector || "").trim();
        if (!normalized || normalized === "body" || normalized === "html" || normalized === "window" || normalized === "document") {
            return null;
        }
        return document.querySelector(normalized);
    }

    const rootElement = resolveRoot(rootSelector);
    const scrollTarget = rootElement || window;
    const targetElement = targetSelector ? document.querySelector(targetSelector) : null;
    const scope = rootElement || document;
    let sections = [];
    let links = [];
    let observer = null;
    let mutationObserver = null;
    let animationFrame = 0;
    const linkCleanups = [];

    function rootBounds() {
        if (rootElement) {
            const rect = rootElement.getBoundingClientRect();
            return { top: rect.top, bottom: rect.bottom, height: rect.height };
        }

        const height = window.innerHeight || document.documentElement.clientHeight || 0;
        return { top: 0, bottom: height, height };
    }

    function idFromSelector(selector) {
        if (!selector || selector[0] !== "#") {
            return "";
        }

        try {
            return decodeURIComponent(selector.slice(1));
        } catch (_) {
            return selector.slice(1);
        }
    }

    function idFromLink(link) {
        return idFromSelector(link.getAttribute("href") || link.getAttribute("data-bs-target") || "");
    }

    function sectionFromId(sectionId) {
        if (!sectionId) {
            return null;
        }

        return document.getElementById(sectionId);
    }

    function elementVisible(element) {
        if (!element || !element.isConnected) {
            return false;
        }

        const style = window.getComputedStyle(element);
        if (style.display === "none" || style.visibility === "hidden") {
            return false;
        }

        const rect = element.getBoundingClientRect();
        return rect.width > 0 && rect.height > 0;
    }

    function elementInScope(element) {
        return !rootElement || rootElement.contains(element);
    }

    function discoverLinks() {
        if (!targetElement) {
            return [];
        }

        return Array.from(targetElement.querySelectorAll('a[href^="#"], [data-bs-target^="#"]'));
    }

    function discoverSections(nextLinks) {
        const ids = [];
        nextLinks.forEach(function(link) {
            const sectionId = idFromLink(link);
            if (sectionId && ids.indexOf(sectionId) === -1) {
                ids.push(sectionId);
            }
        });

        if (ids.length > 0) {
            return ids
                .map(sectionFromId)
                .filter(function(section) {
                    return section && elementInScope(section);
                });
        }

        return Array.from(scope.querySelectorAll("[id]")).filter(function(section) {
            return !targetElement || !targetElement.contains(section);
        });
    }

    function updateLinks(activeId) {
        links.forEach(function(link) {
            const isActive = idFromLink(link) === activeId;
            link.classList.toggle("active", isActive);
            if (isActive) {
                link.setAttribute("aria-current", "true");
            } else {
                link.removeAttribute("aria-current");
            }
        });
    }

    function setActive(nextActive) {
        const activeId = String(nextActive || "");
        if (state.active === activeId) {
            updateLinks(activeId);
            return;
        }

        state.active = activeId;
        updateLinks(activeId);
        window.dispatchEvent(new CustomEvent(eventName, { detail: activeId }));
    }

    function chooseActive() {
        const bounds = rootBounds();
        const visibleSections = sections
            .filter(function(section) {
                if (!elementVisible(section)) {
                    return false;
                }

                const rect = section.getBoundingClientRect();
                return rect.bottom > bounds.top && rect.top < bounds.bottom;
            })
            .sort(function(a, b) {
                return a.getBoundingClientRect().top - b.getBoundingClientRect().top;
            });

        if (visibleSections.length === 0) {
            return "";
        }

        let activeSection = visibleSections[0];
        visibleSections.forEach(function(section) {
            const rect = section.getBoundingClientRect();
            if (rect.top - bounds.top <= offset + 1) {
                activeSection = section;
            }
        });

        return activeSection.id || "";
    }

    function update() {
        animationFrame = 0;
        setActive(chooseActive());
    }

    function scheduleUpdate() {
        if (animationFrame) {
            return;
        }
        animationFrame = window.requestAnimationFrame(update);
    }

    function attachSmoothScroll() {
        if (!smoothScroll) {
            return;
        }

        links.forEach(function(link) {
            const handler = function(event) {
                const section = sectionFromId(idFromLink(link));
                if (!section || !elementInScope(section)) {
                    return;
                }

                event.preventDefault();
                const behavior = "smooth";
                if (rootElement) {
                    const rootRect = rootElement.getBoundingClientRect();
                    const sectionRect = section.getBoundingClientRect();
                    rootElement.scrollTo({
                        top: rootElement.scrollTop + sectionRect.top - rootRect.top - offset,
                        behavior
                    });
                } else {
                    const sectionRect = section.getBoundingClientRect();
                    window.scrollTo({
                        top: window.scrollY + sectionRect.top - offset,
                        behavior
                    });
                }
            };
            link.addEventListener("click", handler);
            linkCleanups.push(function() {
                link.removeEventListener("click", handler);
            });
        });
    }

    function refresh() {
        links = discoverLinks();
        sections = discoverSections(links);

        if (observer) {
            observer.disconnect();
        }

        observer = new IntersectionObserver(scheduleUpdate, {
            root: rootElement,
            rootMargin,
            threshold
        });

        sections.forEach(function(section) {
            if (elementVisible(section)) {
                observer.observe(section);
            }
        });

        attachSmoothScroll();
        scheduleUpdate();
    }

    const scrollHandler = scheduleUpdate;
    scrollTarget.addEventListener("scroll", scrollHandler, { passive: true });
    window.addEventListener("resize", scrollHandler, { passive: true });

    mutationObserver = new MutationObserver(refresh);
    mutationObserver.observe(rootElement || document.body, {
        childList: true,
        subtree: true,
        attributes: true,
        attributeFilter: ["id", "href", "data-bs-target", "class", "style", "hidden"]
    });

    state.cleanup = function() {
        if (animationFrame) {
            window.cancelAnimationFrame(animationFrame);
        }
        if (observer) {
            observer.disconnect();
        }
        if (mutationObserver) {
            mutationObserver.disconnect();
        }
        scrollTarget.removeEventListener("scroll", scrollHandler);
        window.removeEventListener("resize", scrollHandler);
        linkCleanups.forEach(function(cleanup) {
            cleanup();
        });
        delete window.__dbcssScrollspy[id];
    };

    refresh();
})();
"##;

const SCROLLSPY_EVENT_SCRIPT: &str = r##"
const id = __ID__;
const last = __LAST__;
const eventName = "dbcss:scrollspy:" + id;

return new Promise(function(resolve) {
    const state = window.__dbcssScrollspy && window.__dbcssScrollspy[id];
    if (state && state.active !== last) {
        resolve(state.active || "");
        return;
    }

    const handler = function(event) {
        const next = String(event.detail || "");
        if (next !== last) {
            window.removeEventListener(eventName, handler);
            resolve(next);
        }
    };

    window.addEventListener(eventName, handler);
});
"##;

const SCROLLSPY_CLEANUP_SCRIPT: &str = r##"
(function() {
    const id = __ID__;
    const state = window.__dbcssScrollspy && window.__dbcssScrollspy[id];
    if (state && typeof state.cleanup === "function") {
        state.cleanup();
    }
})();
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_js_uses_default_for_empty_or_invalid_values() {
        assert_eq!(threshold_js(&[]), "[0.100,0.500,1.000]");
        assert_eq!(threshold_js(&[f64::NAN]), "[0.100,0.500,1.000]");
    }

    #[test]
    fn threshold_js_clamps_to_intersection_observer_range() {
        assert_eq!(threshold_js(&[-1.0, 0.25, 2.0]), "[0.000,0.250,1.000]");
    }
}
