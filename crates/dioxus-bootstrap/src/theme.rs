use dioxus::prelude::*;

use crate::types::Color;

/// Bootstrap theme mode.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    /// Toggle between light and dark.
    pub fn toggle(self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    /// CSS value for `data-bs-theme` attribute.
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Applies `data-bs-theme` to the document root for Bootstrap dark/light mode.
///
/// Place this at the top of your app. It reactively sets `data-bs-theme` on `<html>`.
///
/// This controls the light/dark *mode* only. To customize the actual *colors*
/// (brand palette and surfaces), use
/// [`BootstrapThemeProvider`](crate::theme_vars::BootstrapThemeProvider); the
/// two compose, one for the mode and one for the colors.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML (manual) -->
/// <html data-bs-theme="dark">
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// // Dioxus — reactive theme switching
/// let theme = use_signal(|| Theme::Dark);
/// rsx! {
///     ThemeProvider { theme }
///     BootstrapHead {}
///     ThemeToggle { theme }  // sun/moon toggle button
///     // your app content
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ThemeProviderProps {
    /// Signal controlling the current theme.
    pub theme: Signal<Theme>,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme_signal = props.theme;

    // Reactively set data-bs-theme on <html> whenever the signal changes.
    use_effect(move || {
        let theme = *theme_signal.read();
        let theme_str = theme.as_str();
        document::eval(&format!(
            "document.documentElement.setAttribute('data-bs-theme', '{theme_str}');"
        ));
    });

    rsx! {}
}

/// A toggle button that switches between light and dark mode.
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// let theme = use_signal(|| Theme::Dark);
/// rsx! {
///     ThemeProvider { theme: theme }
///     ThemeToggle { theme: theme }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct ThemeToggleProps {
    /// Signal controlling the current theme.
    pub theme: Signal<Theme>,
    /// Button color.
    #[props(default)]
    pub color: Option<Color>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn ThemeToggle(props: ThemeToggleProps) -> Element {
    let theme = *props.theme.read();
    let mut theme_signal = props.theme;

    let icon = match theme {
        Theme::Light => "moon-stars",
        Theme::Dark => "sun",
    };

    let label = match theme {
        Theme::Light => "Switch to dark mode",
        Theme::Dark => "Switch to light mode",
    };

    let btn_class = match &props.color {
        Some(c) => format!("btn btn-outline-{c}"),
        None => "btn btn-outline-secondary".to_string(),
    };

    let full_class = if props.class.is_empty() {
        btn_class
    } else {
        format!("{btn_class} {}", props.class)
    };

    rsx! {
        button {
            class: "{full_class}",
            r#type: "button",
            title: "{label}",
            "aria-label": "{label}",
            onclick: move |_| {
                let new_theme = theme.toggle();
                theme_signal.set(new_theme);
            },
            i { class: "bi bi-{icon}" }
        }
    }
}
