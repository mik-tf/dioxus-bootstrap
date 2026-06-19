use dioxus::prelude::*;

/// Bootstrap Figure component — image with optional caption.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <figure class="figure">
///   <img src="/photo.jpg" class="figure-img img-fluid rounded" alt="Photo">
///   <figcaption class="figure-caption text-end">A caption.</figcaption>
/// </figure>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Figure { src: "/photo.jpg", alt: "A photo",
///         caption: "A caption for the image.",
///         caption_align: "end",
///         rounded: true,
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `src` — image URL
/// - `alt` — alt text
/// - `caption` / `caption_align` — caption text and alignment
/// - `fluid` — responsive image (default: true)
/// - `rounded` — rounded corners
/// - `thumbnail` — thumbnail border
#[derive(Clone, PartialEq, Props)]
pub struct FigureProps {
    /// Image source URL.
    pub src: String,
    /// Alt text for the image.
    #[props(default)]
    pub alt: String,
    /// Caption text.
    #[props(default)]
    pub caption: String,
    /// Align caption (start, center, end).
    #[props(default)]
    pub caption_align: String,
    /// Make the image fluid (responsive).
    #[props(default = true)]
    pub fluid: bool,
    /// Add rounded corners to the image.
    #[props(default)]
    pub rounded: bool,
    /// Add thumbnail border to the image.
    #[props(default)]
    pub thumbnail: bool,
    /// Additional CSS classes for the figure.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the image.
    #[props(default)]
    pub img_class: String,
}

#[component]
pub fn Figure(props: FigureProps) -> Element {
    let fig_class = if props.class.is_empty() {
        "figure".to_string()
    } else {
        format!("figure {}", props.class)
    };

    let mut img_classes = vec!["figure-img".to_string()];
    if props.fluid {
        img_classes.push("img-fluid".to_string());
    }
    if props.rounded {
        img_classes.push("rounded".to_string());
    }
    if props.thumbnail {
        img_classes.push("img-thumbnail".to_string());
    }
    if !props.img_class.is_empty() {
        img_classes.push(props.img_class.clone());
    }
    let img_class = img_classes.join(" ");

    let caption_class = if props.caption_align.is_empty() {
        "figure-caption".to_string()
    } else {
        format!("figure-caption text-{}", props.caption_align)
    };

    rsx! {
        figure { class: "{fig_class}",
            img {
                class: "{img_class}",
                src: "{props.src}",
                alt: "{props.alt}",
            }
            if !props.caption.is_empty() {
                figcaption { class: "{caption_class}", "{props.caption}" }
            }
        }
    }
}

/// Bootstrap responsive embed / aspect ratio wrapper.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="ratio ratio-16x9">
///   <iframe src="https://www.youtube.com/embed/..."></iframe>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Ratio { aspect: "16x9",
///         iframe { src: "https://www.youtube.com/embed/..." }
///     }
///     Ratio { aspect: "1x1",
///         div { class: "bg-primary text-white d-flex align-items-center justify-content-center", "1:1" }
///     }
/// }
/// # }
/// ```
///
/// # Props
///
/// - `aspect` — "1x1", "4x3", "16x9", "21x9"
#[derive(Clone, PartialEq, Props)]
pub struct RatioProps {
    /// Aspect ratio: "1x1", "4x3", "16x9", "21x9".
    #[props(default = "16x9".to_string())]
    pub aspect: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child element (iframe, video, embed, etc.).
    pub children: Element,
}

#[component]
pub fn Ratio(props: RatioProps) -> Element {
    let full_class = if props.class.is_empty() {
        format!("ratio ratio-{}", props.aspect)
    } else {
        format!("ratio ratio-{} {}", props.aspect, props.class)
    };

    rsx! {
        div { class: "{full_class}",
            {props.children}
        }
    }
}
