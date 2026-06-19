use dioxus::prelude::*;

/// Runtime Bootstrap CSS variable theme overrides.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct BootstrapTheme {
    pub colors: ThemeColors,
    pub surfaces: SurfaceColors,
    pub dark: Option<ThemeModeTokens>,
}

/// Token overrides for a specific theme mode.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeModeTokens {
    pub colors: ThemeColors,
    pub surfaces: SurfaceColors,
}

/// Semantic Bootstrap color slots.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThemeColors {
    pub primary: Option<SemanticColorScale>,
    pub secondary: Option<SemanticColorScale>,
    pub success: Option<SemanticColorScale>,
    pub info: Option<SemanticColorScale>,
    pub warning: Option<SemanticColorScale>,
    pub danger: Option<SemanticColorScale>,
    pub light: Option<SemanticColorScale>,
    pub dark: Option<SemanticColorScale>,
}

/// Bootstrap surface and utility color variables.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SurfaceColors {
    pub body_bg: Option<String>,
    pub body_color: Option<String>,
    pub secondary_bg: Option<String>,
    pub secondary_color: Option<String>,
    pub tertiary_bg: Option<String>,
    pub tertiary_color: Option<String>,
    pub border_color: Option<String>,
    pub link_color: Option<String>,
    pub link_hover_color: Option<String>,
}

/// Semantic Bootstrap color variables for one slot.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticColorScale {
    pub base: String,
    pub rgb: Option<(u8, u8, u8)>,
    pub text_emphasis: Option<String>,
    pub bg_subtle: Option<String>,
    pub border_subtle: Option<String>,
}

impl SemanticColorScale {
    pub fn new(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            rgb: None,
            text_emphasis: None,
            bg_subtle: None,
            border_subtle: None,
        }
    }
}

impl From<&str> for SemanticColorScale {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Copy)]
enum ThemeVariant {
    Light,
    Dark,
}

/// Injects Bootstrap 5.3 CSS variables as a runtime `<style>` block.
#[derive(Clone, PartialEq, Props)]
pub struct BootstrapThemeProviderProps {
    pub theme: BootstrapTheme,
}

#[component]
pub fn BootstrapThemeProvider(props: BootstrapThemeProviderProps) -> Element {
    let css = build_theme_css(&props.theme);

    if css.is_empty() {
        return rsx! {};
    }

    rsx! {
        style { "{css}" }
    }
}

fn build_theme_css(theme: &BootstrapTheme) -> String {
    let mut out = String::new();

    push_mode_css(
        &mut out,
        ":root",
        &theme.colors,
        &theme.surfaces,
        ThemeVariant::Light,
    );

    if let Some(dark) = &theme.dark {
        push_mode_css(
            &mut out,
            r#"[data-bs-theme="dark"]"#,
            &dark.colors,
            &dark.surfaces,
            ThemeVariant::Dark,
        );
    }

    out
}

fn push_mode_css(
    out: &mut String,
    selector: &str,
    colors: &ThemeColors,
    surfaces: &SurfaceColors,
    variant: ThemeVariant,
) {
    let mut block = String::new();

    for (name, scale) in [
        ("primary", colors.primary.as_ref()),
        ("secondary", colors.secondary.as_ref()),
        ("success", colors.success.as_ref()),
        ("info", colors.info.as_ref()),
        ("warning", colors.warning.as_ref()),
        ("danger", colors.danger.as_ref()),
        ("light", colors.light.as_ref()),
        ("dark", colors.dark.as_ref()),
    ] {
        push_color_scale_css(&mut block, name, scale, variant);
    }

    for (name, value) in [
        ("--bs-body-bg", surfaces.body_bg.as_deref()),
        ("--bs-body-color", surfaces.body_color.as_deref()),
        ("--bs-secondary-bg", surfaces.secondary_bg.as_deref()),
        ("--bs-secondary-color", surfaces.secondary_color.as_deref()),
        ("--bs-tertiary-bg", surfaces.tertiary_bg.as_deref()),
        ("--bs-tertiary-color", surfaces.tertiary_color.as_deref()),
        ("--bs-border-color", surfaces.border_color.as_deref()),
        ("--bs-link-color", surfaces.link_color.as_deref()),
        (
            "--bs-link-hover-color",
            surfaces.link_hover_color.as_deref(),
        ),
    ] {
        push_var(&mut block, name, value);
    }

    if block.is_empty() {
        return;
    }

    out.push_str(selector);
    out.push_str(" {\n");
    out.push_str(&block);
    out.push_str("}\n");
}

fn push_color_scale_css(
    out: &mut String,
    name: &str,
    scale: Option<&SemanticColorScale>,
    variant: ThemeVariant,
) {
    let Some(scale) = scale else {
        return;
    };

    push_var(out, &format!("--bs-{name}"), Some(scale.base.as_str()));

    let parsed_rgb = parse_hex_color(&scale.base);
    let rgb = scale.rgb.or(parsed_rgb);
    let derived = parsed_rgb.map(|rgb| derive_scale(rgb, variant));

    if let Some((red, green, blue)) = rgb {
        let rgb_value = format!("{red}, {green}, {blue}");
        push_var(out, &format!("--bs-{name}-rgb"), Some(&rgb_value));
    }

    let text_emphasis = scale.text_emphasis.as_deref().or_else(|| {
        derived
            .as_ref()
            .map(|derived| derived.text_emphasis.as_str())
    });
    let bg_subtle = scale
        .bg_subtle
        .as_deref()
        .or_else(|| derived.as_ref().map(|derived| derived.bg_subtle.as_str()));
    let border_subtle = scale.border_subtle.as_deref().or_else(|| {
        derived
            .as_ref()
            .map(|derived| derived.border_subtle.as_str())
    });

    push_var(out, &format!("--bs-{name}-text-emphasis"), text_emphasis);
    push_var(out, &format!("--bs-{name}-bg-subtle"), bg_subtle);
    push_var(out, &format!("--bs-{name}-border-subtle"), border_subtle);
}

fn push_var(out: &mut String, name: &str, value: Option<&str>) {
    let Some(value) = value else {
        return;
    };

    out.push_str("  ");
    out.push_str(name);
    out.push_str(": ");
    out.push_str(value);
    out.push_str(";\n");
}

#[derive(Clone)]
struct DerivedScale {
    text_emphasis: String,
    bg_subtle: String,
    border_subtle: String,
}

fn derive_scale((red, green, blue): (u8, u8, u8), variant: ThemeVariant) -> DerivedScale {
    let base = (red, green, blue);

    let (text_emphasis, bg_subtle, border_subtle) = match variant {
        ThemeVariant::Light => (
            mix_rgb(base, (0, 0, 0), 0.35),
            mix_rgb(base, (255, 255, 255), 0.85),
            mix_rgb(base, (255, 255, 255), 0.65),
        ),
        ThemeVariant::Dark => {
            let subtle = mix_rgb(base, (0, 0, 0), 0.75);
            (
                mix_rgb(base, (255, 255, 255), 0.55),
                subtle,
                mix_rgb(subtle, (255, 255, 255), 0.08),
            )
        }
    };

    DerivedScale {
        text_emphasis: rgb_to_hex(text_emphasis),
        bg_subtle: rgb_to_hex(bg_subtle),
        border_subtle: rgb_to_hex(border_subtle),
    }
}

fn parse_hex_color(value: &str) -> Option<(u8, u8, u8)> {
    let hex = value.strip_prefix('#')?;

    match hex.len() {
        3 => {
            let red = parse_hex_byte(&hex[0..1].repeat(2))?;
            let green = parse_hex_byte(&hex[1..2].repeat(2))?;
            let blue = parse_hex_byte(&hex[2..3].repeat(2))?;
            Some((red, green, blue))
        }
        6 => Some((
            parse_hex_byte(&hex[0..2])?,
            parse_hex_byte(&hex[2..4])?,
            parse_hex_byte(&hex[4..6])?,
        )),
        _ => None,
    }
}

fn parse_hex_byte(value: &str) -> Option<u8> {
    u8::from_str_radix(value, 16).ok()
}

fn mix_rgb(from: (u8, u8, u8), to: (u8, u8, u8), amount: f32) -> (u8, u8, u8) {
    (
        mix_channel(from.0, to.0, amount),
        mix_channel(from.1, to.1, amount),
        mix_channel(from.2, to.2, amount),
    )
}

fn mix_channel(from: u8, to: u8, amount: f32) -> u8 {
    let blended = (from as f32 * (1.0 - amount)) + (to as f32 * amount);
    blended.round().clamp(0.0, 255.0) as u8
}

fn rgb_to_hex((red, green, blue): (u8, u8, u8)) -> String {
    format!("#{red:02x}{green:02x}{blue:02x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_short_and_long_hex_colors() {
        assert_eq!(parse_hex_color("#165317"), Some((22, 83, 23)));
        assert_eq!(parse_hex_color("#abc"), Some((170, 187, 204)));
        assert_eq!(parse_hex_color("165317"), None);
        assert_eq!(parse_hex_color("#abcd"), None);
    }

    #[test]
    fn generates_light_and_dark_theme_css() {
        let theme = BootstrapTheme {
            colors: ThemeColors {
                primary: Some(SemanticColorScale::new("#165317")),
                dark: Some(SemanticColorScale::new("#092817")),
                ..ThemeColors::default()
            },
            surfaces: SurfaceColors {
                body_bg: Some("#f7fbf7".into()),
                body_color: Some("#1d2b1f".into()),
                ..SurfaceColors::default()
            },
            dark: Some(ThemeModeTokens {
                colors: ThemeColors {
                    primary: Some(SemanticColorScale::new("#4e9f53")),
                    ..ThemeColors::default()
                },
                surfaces: SurfaceColors {
                    body_bg: Some("#0b120c".into()),
                    body_color: Some("#e6efe7".into()),
                    ..SurfaceColors::default()
                },
            }),
        };

        let css = build_theme_css(&theme);

        assert!(css.contains(":root {\n"));
        assert!(css.contains(r#"[data-bs-theme="dark"] {"#));
        assert!(css.contains("  --bs-primary: #165317;"));
        assert!(css.contains("  --bs-primary-rgb: 22, 83, 23;"));
        assert!(css.contains("  --bs-dark: #092817;"));
        assert!(css.contains("  --bs-body-bg: #f7fbf7;"));
        assert!(css.contains("  --bs-body-color: #1d2b1f;"));
        assert!(css.contains("  --bs-primary: #4e9f53;"));
        assert!(css.contains("  --bs-primary-rgb: 78, 159, 83;"));
        assert!(css.contains("  --bs-body-bg: #0b120c;"));
        assert!(css.contains("  --bs-body-color: #e6efe7;"));
    }

    #[test]
    fn preserves_raw_values_and_explicit_overrides() {
        let theme = BootstrapTheme {
            colors: ThemeColors {
                primary: Some(SemanticColorScale {
                    base: "var(--brand-primary)".into(),
                    rgb: None,
                    text_emphasis: Some("#112233".into()),
                    bg_subtle: None,
                    border_subtle: Some("#ddeeff".into()),
                }),
                ..ThemeColors::default()
            },
            ..BootstrapTheme::default()
        };

        let css = build_theme_css(&theme);

        assert!(css.contains("  --bs-primary: var(--brand-primary);"));
        assert!(css.contains("  --bs-primary-text-emphasis: #112233;"));
        assert!(css.contains("  --bs-primary-border-subtle: #ddeeff;"));
        assert!(!css.contains("--bs-primary-rgb"));
        assert!(!css.contains("--bs-primary-bg-subtle"));
    }
}
