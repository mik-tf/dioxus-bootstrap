use dioxus::prelude::*;

/// A single slide in the Carousel.
#[derive(Clone, PartialEq)]
pub struct CarouselSlide {
    /// Image source URL.
    pub src: String,
    /// Alt text for the image.
    pub alt: String,
    /// Optional caption title.
    pub caption_title: Option<String>,
    /// Optional caption text.
    pub caption_text: Option<String>,
}

/// Bootstrap Carousel component — signal-driven, no JavaScript.
///
/// Supports slide/fade transitions, auto-play with configurable interval,
/// pause on hover, keyboard navigation (arrow keys), and touch swipe.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="carousel slide" data-bs-ride="carousel">` | `Carousel { active: signal, ride: true, slides: vec![...] }` |
/// | `<div class="carousel-item"><img src="..." class="d-block w-100">` | `CarouselSlide { src: "...".into(), alt: "...".into(), ... }` |
/// | `<div class="carousel slide carousel-fade">` | `Carousel { fade: true, ... }` |
/// | `data-bs-interval="3000"` | `interval: 3000` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// let active = use_signal(|| 0usize);
/// rsx! {
///     Carousel {
///         active: active,
///         slides: vec![
///             CarouselSlide { src: "/img/1.jpg".into(), alt: "First".into(),
///                 caption_title: Some("First slide".into()), caption_text: None },
///             CarouselSlide { src: "/img/2.jpg".into(), alt: "Second".into(),
///                 caption_title: None, caption_text: None },
///         ],
///         ride: true,
///         interval: 5000,
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CarouselProps {
    /// Signal controlling the active slide index.
    pub active: Signal<usize>,
    /// Slide definitions.
    pub slides: Vec<CarouselSlide>,
    /// Show indicator dots.
    #[props(default = true)]
    pub indicators: bool,
    /// Show prev/next controls.
    #[props(default = true)]
    pub controls: bool,
    /// Crossfade transition instead of slide.
    #[props(default)]
    pub fade: bool,
    /// Dark variant for lighter background images.
    #[props(default)]
    pub dark: bool,
    /// Enable auto-play cycling.
    #[props(default)]
    pub ride: bool,
    /// Auto-play interval in milliseconds (default 5000).
    #[props(default = 5000)]
    pub interval: u64,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

/// Direction of slide transition.
#[derive(Clone, Copy, PartialEq)]
enum SlideDirection {
    Next,
    Prev,
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let current = *props.active.read();
    let mut active_signal = props.active;
    let total = props.slides.len();

    if total == 0 {
        return rsx! {};
    }

    // Track which slide is transitioning and direction
    let mut transitioning = use_signal(|| Option::<(usize, usize, SlideDirection)>::None);
    let trans = *transitioning.read();

    // Pause state for hover
    let mut paused = use_signal(|| false);

    // Touch tracking for swipe
    let mut touch_start_x = use_signal(|| 0.0f64);

    // Navigate to next/prev — reads signals fresh each call
    let mut go_direction = move |direction: SlideDirection| {
        // Read current state from signals (not stale captures)
        if transitioning.read().is_some() {
            return; // already transitioning
        }
        let cur = *active_signal.read();
        let to = match direction {
            SlideDirection::Next => {
                if cur + 1 >= total {
                    0
                } else {
                    cur + 1
                }
            }
            SlideDirection::Prev => {
                if cur == 0 {
                    total - 1
                } else {
                    cur - 1
                }
            }
        };
        transitioning.set(Some((cur, to, direction)));
        // After transition duration, finalize
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(600).await;
            active_signal.set(to);
            transitioning.set(None);
        });
    };

    // Auto-play timer
    let ride = props.ride;
    let interval = props.interval;
    use_future(move || async move {
        if !ride || total <= 1 {
            return;
        }
        loop {
            gloo_timers::future::TimeoutFuture::new(interval as u32).await;
            if !*paused.read() && transitioning.read().is_none() {
                go_direction(SlideDirection::Next);
            }
        }
    });

    let mut classes = vec!["carousel".to_string(), "slide".to_string()];
    if props.fade {
        classes.push("carousel-fade".to_string());
    }
    if props.dark {
        classes.push("carousel-dark".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    rsx! {
        div {
            class: "{full_class}",
            tabindex: "0",
            // Pause on hover
            onmouseenter: move |_| paused.set(true),
            onmouseleave: move |_| paused.set(false),
            // Keyboard navigation
            onkeydown: move |evt: KeyboardEvent| {
                match evt.key() {
                    Key::ArrowLeft => go_direction(SlideDirection::Prev),
                    Key::ArrowRight => go_direction(SlideDirection::Next),
                    _ => {}
                }
            },
            // Touch start
            ontouchstart: move |evt: TouchEvent| {
                if let Some(touch) = evt.touches().first() {
                    let coords: dioxus_elements::geometry::PagePoint = touch.page_coordinates();
                    touch_start_x.set(coords.x);
                }
            },
            // Touch end — detect swipe direction
            ontouchend: move |evt: TouchEvent| {
                if let Some(touch) = evt.touches_changed().first() {
                    let start = *touch_start_x.read();
                    let coords: dioxus_elements::geometry::PagePoint = touch.page_coordinates();
                    let diff = coords.x - start;
                    // Minimum swipe threshold of 50px
                    if diff < -50.0 {
                        go_direction(SlideDirection::Next);
                    } else if diff > 50.0 {
                        go_direction(SlideDirection::Prev);
                    }
                }
            },

            // Indicators
            if props.indicators {
                div { class: "carousel-indicators",
                    for i in 0..total {
                        button {
                            class: if current == i { "active" } else { "" },
                            r#type: "button",
                            "aria-current": if current == i { "true" } else { "false" },
                            "aria-label": "Slide {i}",
                            onclick: move |_| active_signal.set(i),
                        }
                    }
                }
            }

            // Slides
            div {
                class: "carousel-inner",
                style: "overflow: hidden;",
                for (i, slide) in props.slides.iter().enumerate() {
                    {
                        let item_class = build_slide_class(i, current, trans, props.fade);
                        let item_style = build_slide_style(i, current, trans, props.fade);
                        rsx! {
                            div {
                                class: "{item_class}",
                                style: "{item_style}",
                                img {
                                    class: "d-block w-100",
                                    src: "{slide.src}",
                                    alt: "{slide.alt}",
                                }
                                if slide.caption_title.is_some() || slide.caption_text.is_some() {
                                    div { class: "carousel-caption d-none d-md-block",
                                        if let Some(ref title) = slide.caption_title {
                                            h5 { "{title}" }
                                        }
                                        if let Some(ref text) = slide.caption_text {
                                            p { "{text}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Controls
            if props.controls && total > 1 {
                button {
                    class: "carousel-control-prev",
                    r#type: "button",
                    onclick: move |_| go_direction(SlideDirection::Prev),
                    span { class: "carousel-control-prev-icon", "aria-hidden": "true" }
                    span { class: "visually-hidden", "Previous" }
                }
                button {
                    class: "carousel-control-next",
                    r#type: "button",
                    onclick: move |_| go_direction(SlideDirection::Next),
                    span { class: "carousel-control-next-icon", "aria-hidden": "true" }
                    span { class: "visually-hidden", "Next" }
                }
            }
        }
    }
}

/// Build the CSS class for a slide item during transitions.
fn build_slide_class(
    index: usize,
    current: usize,
    trans: Option<(usize, usize, SlideDirection)>,
    fade: bool,
) -> String {
    match trans {
        Some((from, to, direction)) => {
            if fade {
                if index == from {
                    "carousel-item active".to_string()
                } else if index == to {
                    "carousel-item carousel-item-next carousel-item-start active".to_string()
                } else {
                    "carousel-item".to_string()
                }
            } else if index == from {
                match direction {
                    SlideDirection::Next => "carousel-item active carousel-item-start".to_string(),
                    SlideDirection::Prev => "carousel-item active carousel-item-end".to_string(),
                }
            } else if index == to {
                match direction {
                    SlideDirection::Next => {
                        "carousel-item carousel-item-next carousel-item-start".to_string()
                    }
                    SlideDirection::Prev => {
                        "carousel-item carousel-item-prev carousel-item-end".to_string()
                    }
                }
            } else {
                "carousel-item".to_string()
            }
        }
        None => {
            if index == current {
                "carousel-item active".to_string()
            } else {
                "carousel-item".to_string()
            }
        }
    }
}

/// Build inline styles for slide positioning during transitions.
fn build_slide_style(
    index: usize,
    _current: usize,
    trans: Option<(usize, usize, SlideDirection)>,
    fade: bool,
) -> String {
    if fade {
        return String::new();
    }
    match trans {
        Some((from, to, _direction)) => {
            if index == from || index == to {
                "transition: transform 0.6s ease-in-out;".to_string()
            } else {
                String::new()
            }
        }
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slide_class_no_transition() {
        assert_eq!(build_slide_class(2, 2, None, false), "carousel-item active");
        assert_eq!(build_slide_class(1, 2, None, false), "carousel-item");
    }

    #[test]
    fn slide_class_sliding_next() {
        let trans = Some((0, 1, SlideDirection::Next));
        assert_eq!(
            build_slide_class(0, 0, trans, false),
            "carousel-item active carousel-item-start"
        );
        assert_eq!(
            build_slide_class(1, 0, trans, false),
            "carousel-item carousel-item-next carousel-item-start"
        );
        assert_eq!(build_slide_class(2, 0, trans, false), "carousel-item");
    }

    #[test]
    fn slide_class_sliding_prev() {
        let trans = Some((1, 0, SlideDirection::Prev));
        assert_eq!(
            build_slide_class(1, 1, trans, false),
            "carousel-item active carousel-item-end"
        );
        assert_eq!(
            build_slide_class(0, 1, trans, false),
            "carousel-item carousel-item-prev carousel-item-end"
        );
    }

    #[test]
    fn slide_class_fade() {
        let trans = Some((0, 1, SlideDirection::Next));
        assert_eq!(build_slide_class(0, 0, trans, true), "carousel-item active");
        assert_eq!(
            build_slide_class(1, 0, trans, true),
            "carousel-item carousel-item-next carousel-item-start active"
        );
        assert_eq!(build_slide_class(2, 0, trans, true), "carousel-item");
    }

    #[test]
    fn slide_style_only_on_active_pair() {
        let trans = Some((0, 1, SlideDirection::Next));
        let css = "transition: transform 0.6s ease-in-out;";
        assert_eq!(build_slide_style(0, 0, trans, false), css);
        assert_eq!(build_slide_style(1, 0, trans, false), css);
        assert_eq!(build_slide_style(2, 0, trans, false), "");
        assert_eq!(build_slide_style(0, 0, None, false), "");
        // Fade mode never emits an inline transition.
        assert_eq!(build_slide_style(0, 0, trans, true), "");
    }
}
