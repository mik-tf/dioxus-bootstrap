use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::overlay::{
    OverlayOffset, OverlayPlacement, OverlayPosition, OverlayRect, calculate_overlay_position,
};

static NEXT_TOOLTIP_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_TOOLTIP_TRIGGER_ID: AtomicUsize = AtomicUsize::new(1);

/// Tooltip placement.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipPlacement {
    /// Choose the first fallback placement that fits the viewport.
    Auto,
    /// Place tooltip above the trigger.
    #[default]
    Top,
    /// Place tooltip below the trigger.
    Bottom,
    /// Place tooltip before the trigger in the inline axis.
    Start,
    /// Place tooltip after the trigger in the inline axis.
    End,
}

impl TooltipPlacement {
    fn class(self) -> &'static str {
        match self {
            TooltipPlacement::Auto | TooltipPlacement::Top => "bs-tooltip-top",
            TooltipPlacement::Bottom => "bs-tooltip-bottom",
            TooltipPlacement::Start => "bs-tooltip-start",
            TooltipPlacement::End => "bs-tooltip-end",
        }
    }

    fn data_value(self) -> &'static str {
        match self {
            TooltipPlacement::Auto => "auto",
            TooltipPlacement::Top => "top",
            TooltipPlacement::Bottom => "bottom",
            TooltipPlacement::Start => "start",
            TooltipPlacement::End => "end",
        }
    }
}

impl From<TooltipPlacement> for OverlayPlacement {
    fn from(value: TooltipPlacement) -> Self {
        match value {
            TooltipPlacement::Auto => OverlayPlacement::Auto,
            TooltipPlacement::Top => OverlayPlacement::Top,
            TooltipPlacement::Bottom => OverlayPlacement::Bottom,
            TooltipPlacement::Start => OverlayPlacement::Start,
            TooltipPlacement::End => OverlayPlacement::End,
        }
    }
}

impl From<OverlayPlacement> for TooltipPlacement {
    fn from(value: OverlayPlacement) -> Self {
        match value {
            OverlayPlacement::Auto => TooltipPlacement::Auto,
            OverlayPlacement::Top => TooltipPlacement::Top,
            OverlayPlacement::Bottom => TooltipPlacement::Bottom,
            OverlayPlacement::Start => TooltipPlacement::Start,
            OverlayPlacement::End => TooltipPlacement::End,
        }
    }
}

/// Tooltip trigger set.
///
/// Bootstrap allows hover, focus, click, and manual trigger styles. The default
/// matches Bootstrap's normal keyboard-friendly trigger: hover plus focus.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TooltipTriggers {
    pub hover: bool,
    pub focus: bool,
    pub click: bool,
}

impl TooltipTriggers {
    /// Hover and focus triggers.
    pub const HOVER_FOCUS: Self = Self {
        hover: true,
        focus: true,
        click: false,
    };

    /// Hover-only trigger.
    pub const HOVER: Self = Self {
        hover: true,
        focus: false,
        click: false,
    };

    /// Focus-only trigger.
    pub const FOCUS: Self = Self {
        hover: false,
        focus: true,
        click: false,
    };

    /// Click-only trigger.
    pub const CLICK: Self = Self {
        hover: false,
        focus: false,
        click: true,
    };

    /// No internal trigger; use the `open` prop.
    pub const MANUAL: Self = Self {
        hover: false,
        focus: false,
        click: false,
    };
}

impl Default for TooltipTriggers {
    fn default() -> Self {
        Self::HOVER_FOCUS
    }
}

/// Show/hide delay in milliseconds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TooltipDelay {
    pub show_ms: u32,
    pub hide_ms: u32,
}

impl TooltipDelay {
    pub const fn new(show_ms: u32, hide_ms: u32) -> Self {
        Self { show_ms, hide_ms }
    }
}

#[derive(Clone)]
struct TooltipPositioning {
    trigger_id: String,
    tooltip_id: String,
    overlay_position: Signal<Option<OverlayPosition>>,
    placement: TooltipPlacement,
    fallback_placements: Vec<TooltipPlacement>,
    offset: OverlayOffset,
    boundary_padding: f64,
}

#[derive(Clone, Copy)]
struct TooltipInteractionSignals {
    hover_active: Signal<bool>,
    focus_active: Signal<bool>,
    click_active: Signal<bool>,
}

/// Wrapper helper for disabled controls that need a tooltip trigger.
///
/// Bootstrap documents disabled elements as non-interactive. Wrap a disabled
/// button or input in this component so the wrapper can receive focus and
/// pointer events while preserving the disabled child.
#[derive(Clone, PartialEq, Props)]
pub struct TooltipDisabledTriggerProps {
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
    /// Additional inline style for the wrapper.
    #[props(default)]
    pub style: String,
    /// Disabled child element.
    pub children: Element,
}

#[component]
pub fn TooltipDisabledTrigger(props: TooltipDisabledTriggerProps) -> Element {
    let style = if props.style.is_empty() {
        "display: inline-flex;".to_string()
    } else {
        format!("display: inline-flex; {}", props.style)
    };

    rsx! {
        span {
            class: "{props.class}",
            style,
            tabindex: "0",
            {props.children}
        }
    }
}

/// Bootstrap Tooltip component.
///
/// Renders a Bootstrap tooltip with Dioxus-owned trigger state and viewport-aware
/// placement. It does not depend on Bootstrap JavaScript or Popper.js.
///
/// # Bootstrap HTML -> Dioxus
///
/// ```html
/// <button data-bs-toggle="tooltip" data-bs-placement="top" title="Tooltip text">Hover me</button>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Tooltip { text: "Save your work", placement: TooltipPlacement::Top,
///         Button { color: Color::Primary, "Save" }
///     }
///     Tooltip {
///         text: "Click for details",
///         trigger: TooltipTriggers::CLICK,
///         placement: TooltipPlacement::Auto,
///         Button { color: Color::Info, "Details" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct TooltipProps {
    /// Tooltip text content.
    pub text: String,
    /// Requested placement relative to the trigger element.
    #[props(default)]
    pub placement: TooltipPlacement,
    /// Fallback placements used when requested placement does not fit.
    #[props(default)]
    pub fallback_placements: Vec<TooltipPlacement>,
    /// Trigger behavior. Defaults to hover plus focus.
    #[props(default)]
    pub trigger: TooltipTriggers,
    /// Show/hide delay in milliseconds.
    #[props(default)]
    pub delay: TooltipDelay,
    /// Controlled open state. When set, internal triggers are ignored.
    #[props(default)]
    pub open: Option<bool>,
    /// Offset from the trigger.
    #[props(default = OverlayOffset::TOOLTIP)]
    pub offset: OverlayOffset,
    /// Padding inside the viewport boundary.
    #[props(default = 0.0)]
    pub boundary_padding: f64,
    /// Additional CSS classes for the tooltip element.
    #[props(default)]
    pub class: String,
    /// Child element or elements that act as the trigger.
    pub children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let tooltip_id = use_signal(next_tooltip_id);
    let trigger_id = use_signal(next_tooltip_trigger_id);
    let overlay_position = use_signal(|| None::<OverlayPosition>);
    let visible = use_signal(|| props.open.unwrap_or(false));
    let visibility_revision = use_signal(|| 0_u64);
    let mut hover_active = use_signal(|| false);
    let mut focus_active = use_signal(|| false);
    let mut click_active = use_signal(|| false);

    let has_text = !props.text.is_empty();
    let is_visible = has_text && props.open.unwrap_or(*visible.read());
    let effective_placement = overlay_position
        .read()
        .as_ref()
        .map(|position| TooltipPlacement::from(position.placement))
        .unwrap_or(props.placement);
    let placement_class = effective_placement.class();
    let placement_value = effective_placement.data_value();
    let tooltip_class = classes("tooltip fade show", placement_class, &props.class);
    let tooltip_style = tooltip_style(*overlay_position.read());
    let arrow_style = arrow_style(*overlay_position.read(), effective_placement);
    let describedby = if is_visible {
        tooltip_id.read().clone()
    } else {
        String::new()
    };

    let effect_has_text = has_text;
    let positioning = TooltipPositioning {
        trigger_id: trigger_id.read().clone(),
        tooltip_id: tooltip_id.read().clone(),
        overlay_position,
        placement: props.placement,
        fallback_placements: props.fallback_placements.clone(),
        offset: props.offset,
        boundary_padding: props.boundary_padding,
    };
    let effect_trigger_id = trigger_id.read().clone();
    let effect_tooltip_id = tooltip_id.read().clone();
    let mut effect_overlay_position = overlay_position;
    use_effect(use_reactive(
        (
            &props.open,
            &effect_has_text,
            &props.placement,
            &props.fallback_placements,
            &props.offset,
            &props.boundary_padding,
        ),
        move |(open, has_text, placement, fallback_placements, offset, boundary_padding)| {
            let current_visible = has_text && open.unwrap_or(*visible.read());

            if current_visible {
                measure_tooltip_position(TooltipPositioning {
                    trigger_id: effect_trigger_id.clone(),
                    tooltip_id: effect_tooltip_id.clone(),
                    overlay_position: effect_overlay_position,
                    placement,
                    fallback_placements,
                    offset,
                    boundary_padding,
                });
            } else {
                effect_overlay_position.set(None);
            }
        },
    ));

    let hover_enter_positioning = positioning.clone();
    let hover_leave_positioning = positioning.clone();
    let focus_in_positioning = positioning.clone();
    let focus_out_positioning = positioning.clone();
    let click_positioning = positioning.clone();
    let interactions = TooltipInteractionSignals {
        hover_active,
        focus_active,
        click_active,
    };

    rsx! {
        span {
            id: "{trigger_id}",
            class: "tooltip-wrapper",
            // `inline-flex` hugs the trigger; `inline-block` would add line-box
            // leading and push the measured anchor box below the element, landing
            // the tooltip low (see the same fix on `.popover-wrapper`).
            style: "display: inline-flex;",
            "aria-describedby": "{describedby}",
            onmouseenter: move |_| {
                if props.trigger.hover && props.open.is_none() {
                    hover_active.set(true);
                    schedule_tooltip_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        hover_enter_positioning.clone(),
                    );
                }
            },
            onmouseleave: move |_| {
                if props.trigger.hover && props.open.is_none() {
                    hover_active.set(false);
                    schedule_tooltip_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        hover_leave_positioning.clone(),
                    );
                }
            },
            onfocusin: move |_| {
                if props.trigger.focus && props.open.is_none() {
                    focus_active.set(true);
                    schedule_tooltip_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        focus_in_positioning.clone(),
                    );
                }
            },
            onfocusout: move |_| {
                if props.trigger.focus && props.open.is_none() {
                    focus_active.set(false);
                    schedule_tooltip_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        focus_out_positioning.clone(),
                    );
                }
            },
            onclick: move |_| {
                if props.trigger.click && props.open.is_none() {
                    let next_click_active = {
                        let active = click_active.read();
                        !*active
                    };
                    click_active.set(next_click_active);
                    schedule_tooltip_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        click_positioning.clone(),
                    );
                }
            },

            {props.children}

            if is_visible {
                div {
                    id: "{tooltip_id}",
                    class: "{tooltip_class}",
                    role: "tooltip",
                    "data-popper-placement": "{placement_value}",
                    style: "{tooltip_style}",
                    div { class: "tooltip-arrow", style: "{arrow_style}" }
                    div { class: "tooltip-inner", "{props.text}" }
                }
            }
        }
    }
}

fn next_tooltip_id() -> String {
    let id = NEXT_TOOLTIP_ID.fetch_add(1, Ordering::Relaxed);
    format!("dbcss-tooltip-{id}")
}

fn next_tooltip_trigger_id() -> String {
    let id = NEXT_TOOLTIP_TRIGGER_ID.fetch_add(1, Ordering::Relaxed);
    format!("dbcss-tooltip-trigger-{id}")
}

fn classes(base: &str, placement_class: &str, extra: &str) -> String {
    if extra.is_empty() {
        format!("{base} {placement_class}")
    } else {
        format!("{base} {placement_class} {extra}")
    }
}

fn tooltip_style(position: Option<OverlayPosition>) -> String {
    match position {
        Some(position) => format!(
            "position: fixed; left: {:.3}px; top: {:.3}px; z-index: 1080; pointer-events: none; white-space: nowrap; visibility: visible;",
            position.x, position.y
        ),
        None => {
            "position: fixed; left: 0; top: 0; z-index: 1080; pointer-events: none; white-space: nowrap; visibility: hidden;".to_string()
        }
    }
}

/// Bootstrap's tooltip arrow is `0.8rem` wide (`--bs-tooltip-arrow-width`); half of
/// that centres the arrow element on the computed cross-axis point.
const TOOLTIP_ARROW_HALF: f64 = 6.4;

/// Inline style that slides the `.tooltip-arrow` along the tooltip's cross axis so
/// it keeps pointing at the trigger after the box is clamped to the viewport — the
/// job Popper.js does for a real Bootstrap tooltip. `position: absolute` is
/// required: Bootstrap positions the arrow along the main edge only once Popper has
/// made the element absolutely positioned, so without it the offset is a no-op.
fn arrow_style(position: Option<OverlayPosition>, placement: TooltipPlacement) -> String {
    let Some(position) = position else {
        return String::new();
    };
    let edge = position.arrow - TOOLTIP_ARROW_HALF;
    match placement {
        TooltipPlacement::Start | TooltipPlacement::End => {
            format!("position: absolute; top: {edge:.3}px;")
        }
        _ => format!("position: absolute; left: {edge:.3}px;"),
    }
}

fn schedule_tooltip_visibility(
    mut visible: Signal<bool>,
    mut revision: Signal<u64>,
    open: Option<bool>,
    trigger: TooltipTriggers,
    delay: TooltipDelay,
    interactions: TooltipInteractionSignals,
    mut positioning: TooltipPositioning,
) {
    let next_revision = *revision.read() + 1;
    revision.set(next_revision);

    let should_show = tooltip_should_show(open, trigger, interactions);
    let delay_ms = if should_show {
        delay.show_ms
    } else {
        delay.hide_ms
    };

    spawn(async move {
        if delay_ms > 0 {
            TimeoutFuture::new(delay_ms).await;
        }

        if *revision.read() != next_revision {
            return;
        }

        let should_show = tooltip_should_show(open, trigger, interactions);
        visible.set(should_show);

        if should_show {
            TimeoutFuture::new(0).await;
            measure_tooltip_position(positioning);
        } else {
            positioning.overlay_position.set(None);
        }
    });
}

fn tooltip_should_show(
    open: Option<bool>,
    trigger: TooltipTriggers,
    interactions: TooltipInteractionSignals,
) -> bool {
    if let Some(open) = open {
        return open;
    }

    (trigger.hover && *interactions.hover_active.read())
        || (trigger.focus && *interactions.focus_active.read())
        || (trigger.click && *interactions.click_active.read())
}

fn measure_tooltip_position(mut positioning: TooltipPositioning) {
    spawn(async move {
        let Some(trigger_rect) = element_rect(&positioning.trigger_id).await else {
            return;
        };
        let Some(tooltip_rect) = element_rect(&positioning.tooltip_id).await else {
            return;
        };
        let Some(boundary) = viewport_boundary().await else {
            return;
        };

        let fallback_placements = positioning
            .fallback_placements
            .into_iter()
            .map(OverlayPlacement::from)
            .collect::<Vec<_>>();

        positioning
            .overlay_position
            .set(Some(calculate_overlay_position(
                trigger_rect,
                tooltip_rect,
                boundary,
                positioning.placement.into(),
                &fallback_placements,
                positioning.offset,
                positioning.boundary_padding,
            )));
    });
}

async fn element_rect(id: &str) -> Option<OverlayRect> {
    let id = format!("{id:?}");
    let value = document::eval(&format!(
        r#"
        const element = document.getElementById({id});
        if (!element) {{
            return null;
        }}
        const rect = element.getBoundingClientRect();
        return {{
            x: rect.left,
            y: rect.top,
            width: rect.width,
            height: rect.height
        }};
        "#
    ))
    .await
    .ok()?;

    if value.is_null() {
        return None;
    }

    Some(OverlayRect::new(
        value.get("x").and_then(|value| value.as_f64())?,
        value.get("y").and_then(|value| value.as_f64())?,
        value.get("width").and_then(|value| value.as_f64())?,
        value.get("height").and_then(|value| value.as_f64())?,
    ))
}

async fn viewport_boundary() -> Option<OverlayRect> {
    let value = document::eval(
        r#"
        return {
            x: 0,
            y: 0,
            width: window.innerWidth || document.documentElement.clientWidth || 0,
            height: window.innerHeight || document.documentElement.clientHeight || 0
        };
        "#,
    )
    .await
    .ok()?;

    let rect = OverlayRect::new(
        value.get("x").and_then(|value| value.as_f64())?,
        value.get("y").and_then(|value| value.as_f64())?,
        value.get("width").and_then(|value| value.as_f64())?,
        value.get("height").and_then(|value| value.as_f64())?,
    );

    if rect.width <= 0.0 || rect.height <= 0.0 {
        return None;
    }

    Some(rect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_defaults_to_hover_and_focus() {
        let triggers = TooltipTriggers::default();
        assert!(triggers.hover);
        assert!(triggers.focus);
        assert!(!triggers.click);
    }

    #[test]
    fn placement_converts_to_overlay_placement() {
        assert_eq!(
            OverlayPlacement::from(TooltipPlacement::Auto),
            OverlayPlacement::Auto
        );
        assert_eq!(
            OverlayPlacement::from(TooltipPlacement::Bottom),
            OverlayPlacement::Bottom
        );
    }

    #[test]
    fn placement_classes_match_bootstrap() {
        assert_eq!(TooltipPlacement::Top.class(), "bs-tooltip-top");
        assert_eq!(TooltipPlacement::Bottom.class(), "bs-tooltip-bottom");
        assert_eq!(TooltipPlacement::Start.class(), "bs-tooltip-start");
        assert_eq!(TooltipPlacement::End.class(), "bs-tooltip-end");
    }

    #[test]
    fn arrow_style_offsets_on_cross_axis() {
        let pos = OverlayPosition {
            x: 100.0,
            y: 50.0,
            placement: OverlayPlacement::Bottom,
            fits: false,
            arrow: 30.0,
        };
        // Top/Bottom placements slide the arrow in x; the arrow element is centred
        // on `position.arrow`, so its leading edge is that minus the arrow half-width.
        let bottom = arrow_style(Some(pos), TooltipPlacement::Bottom);
        assert!(bottom.contains("position: absolute"));
        assert!(bottom.contains("left: 23.600px"));
        // Start/End placements slide it in y instead.
        let end = arrow_style(Some(pos), TooltipPlacement::End);
        assert!(end.contains("top: 23.600px"));
        // No measured position yet -> no offset emitted.
        assert_eq!(arrow_style(None, TooltipPlacement::Bottom), "");
    }
}
