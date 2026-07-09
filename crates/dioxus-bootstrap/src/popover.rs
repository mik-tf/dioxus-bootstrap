use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::core::{DynamicNode, TemplateNode, VNode};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::overlay::{
    OverlayOffset, OverlayPlacement, OverlayPosition, OverlayRect, calculate_overlay_position,
};

static NEXT_POPOVER_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_POPOVER_TRIGGER_ID: AtomicUsize = AtomicUsize::new(1);

/// Popover placement.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverPlacement {
    /// Choose the first fallback placement that fits the viewport.
    Auto,
    /// Place popover above trigger.
    Top,
    /// Place popover below trigger.
    Bottom,
    /// Place popover before trigger in the inline axis.
    Start,
    /// Place popover after trigger in the inline axis.
    #[default]
    End,
}

impl PopoverPlacement {
    fn class(self) -> &'static str {
        match self {
            PopoverPlacement::Auto | PopoverPlacement::End => "bs-popover-end",
            PopoverPlacement::Top => "bs-popover-top",
            PopoverPlacement::Bottom => "bs-popover-bottom",
            PopoverPlacement::Start => "bs-popover-start",
        }
    }

    fn data_value(self) -> &'static str {
        match self {
            PopoverPlacement::Auto => "auto",
            PopoverPlacement::Top => "top",
            PopoverPlacement::Bottom => "bottom",
            PopoverPlacement::Start => "start",
            PopoverPlacement::End => "end",
        }
    }
}

impl From<PopoverPlacement> for OverlayPlacement {
    fn from(value: PopoverPlacement) -> Self {
        match value {
            PopoverPlacement::Auto => OverlayPlacement::Auto,
            PopoverPlacement::Top => OverlayPlacement::Top,
            PopoverPlacement::Bottom => OverlayPlacement::Bottom,
            PopoverPlacement::Start => OverlayPlacement::Start,
            PopoverPlacement::End => OverlayPlacement::End,
        }
    }
}

impl From<OverlayPlacement> for PopoverPlacement {
    fn from(value: OverlayPlacement) -> Self {
        match value {
            OverlayPlacement::Auto => PopoverPlacement::Auto,
            OverlayPlacement::Top => PopoverPlacement::Top,
            OverlayPlacement::Bottom => PopoverPlacement::Bottom,
            OverlayPlacement::Start => PopoverPlacement::Start,
            OverlayPlacement::End => PopoverPlacement::End,
        }
    }
}

/// Popover trigger behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverTriggers {
    pub hover: bool,
    pub focus: bool,
    pub click: bool,
}

impl PopoverTriggers {
    /// Click-only trigger.
    pub const CLICK: Self = Self {
        hover: false,
        focus: false,
        click: true,
    };

    /// Hover plus focus triggers.
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

    /// No internal trigger; use the `open` prop.
    pub const MANUAL: Self = Self {
        hover: false,
        focus: false,
        click: false,
    };
}

impl Default for PopoverTriggers {
    fn default() -> Self {
        Self::CLICK
    }
}

/// Show/hide delay in milliseconds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PopoverDelay {
    pub show_ms: u32,
    pub hide_ms: u32,
}

impl PopoverDelay {
    pub const fn new(show_ms: u32, hide_ms: u32) -> Self {
        Self { show_ms, hide_ms }
    }
}

#[derive(Clone)]
struct PopoverPositioning {
    trigger_id: String,
    popover_id: String,
    overlay_position: Signal<Option<OverlayPosition>>,
    placement: PopoverPlacement,
    fallback_placements: Vec<PopoverPlacement>,
    offset: OverlayOffset,
    boundary_padding: f64,
}

#[derive(Clone, Copy)]
struct PopoverInteractionSignals {
    hover_active: Signal<bool>,
    focus_active: Signal<bool>,
    click_active: Signal<bool>,
}

/// Wrapper helper for disabled controls that need a popover trigger.
///
/// Bootstrap documents disabled elements as non-interactive. Wrap a disabled
/// button or input in this component so the wrapper can receive focus and
/// pointer events while preserving the disabled child.
#[derive(Clone, PartialEq, Props)]
pub struct PopoverDisabledTriggerProps {
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
pub fn PopoverDisabledTrigger(props: PopoverDisabledTriggerProps) -> Element {
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

/// Bootstrap Popover component.
///
/// Renders Bootstrap popover markup with Dioxus-owned trigger state and
/// viewport-aware placement. It does not depend on Bootstrap JavaScript or
/// Popper.js.
///
/// # Bootstrap HTML -> Dioxus
///
/// ```html
/// <button data-bs-toggle="popover" data-bs-title="Title" data-bs-content="Content">Click</button>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Popover {
///         title: "Popover Title",
///         body: rsx! { "Rich content here." },
///         Button { color: Color::Info, "Click for details" }
///     }
///     Popover {
///         title: "Focus dismiss",
///         body: rsx! { "Focus another element to dismiss." },
///         trigger: PopoverTriggers::FOCUS,
///         placement: PopoverPlacement::Auto,
///         Button { color: Color::Secondary, "Focus" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct PopoverProps {
    /// Popover title text.
    #[props(default)]
    pub title: String,
    /// Popover body content.
    pub body: Element,
    /// Requested placement relative to the trigger element.
    #[props(default)]
    pub placement: PopoverPlacement,
    /// Fallback placements used when requested placement does not fit.
    #[props(default)]
    pub fallback_placements: Vec<PopoverPlacement>,
    /// Trigger behavior. Defaults to click.
    #[props(default)]
    pub trigger: PopoverTriggers,
    /// Show/hide delay in milliseconds.
    #[props(default)]
    pub delay: PopoverDelay,
    /// Controlled open state. When set, internal triggers are ignored.
    #[props(default)]
    pub open: Option<bool>,
    /// Offset from trigger.
    #[props(default = OverlayOffset::POPOVER)]
    pub offset: OverlayOffset,
    /// Padding inside viewport boundary.
    #[props(default = 0.0)]
    pub boundary_padding: f64,
    /// Close uncontrolled popovers when clicking outside the trigger/popover.
    #[props(default = true)]
    pub dismiss_on_outside_click: bool,
    /// Additional CSS classes for the popover element.
    #[props(default)]
    pub class: String,
    /// Child element used as the trigger.
    pub children: Element,
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let popover_id = use_signal(next_popover_id);
    let trigger_id = use_signal(next_popover_trigger_id);
    let overlay_position = use_signal(|| None::<OverlayPosition>);
    let visible = use_signal(|| props.open.unwrap_or(false));
    let visibility_revision = use_signal(|| 0_u64);
    let mut hover_active = use_signal(|| false);
    let mut focus_active = use_signal(|| false);
    let mut click_active = use_signal(|| false);

    let has_content = !props.title.is_empty() || element_has_content(&props.body);
    let is_visible = has_content && props.open.unwrap_or(*visible.read());
    let effective_placement = overlay_position
        .read()
        .as_ref()
        .map(|position| PopoverPlacement::from(position.placement))
        .unwrap_or(props.placement);
    let placement_class = effective_placement.class();
    let placement_value = effective_placement.data_value();
    let popover_class = classes("popover fade show", placement_class, &props.class);
    let popover_style = popover_style(*overlay_position.read());
    let arrow_style = arrow_style(*overlay_position.read(), effective_placement);
    let describedby = if is_visible {
        popover_id.read().clone()
    } else {
        String::new()
    };

    let effect_has_content = has_content;
    let positioning = PopoverPositioning {
        trigger_id: trigger_id.read().clone(),
        popover_id: popover_id.read().clone(),
        overlay_position,
        placement: props.placement,
        fallback_placements: props.fallback_placements.clone(),
        offset: props.offset,
        boundary_padding: props.boundary_padding,
    };
    let effect_trigger_id = trigger_id.read().clone();
    let effect_popover_id = popover_id.read().clone();
    let mut effect_overlay_position = overlay_position;

    let effect_positioning = positioning.clone();
    use_effect(use_reactive(
        (
            &props.open,
            &effect_has_content,
            &props.placement,
            &props.fallback_placements,
            &props.offset,
            &props.boundary_padding,
            &effect_trigger_id,
            &effect_popover_id,
        ),
        move |_| {
            if !effect_has_content || props.open == Some(false) {
                effect_overlay_position.set(None);
                return;
            }

            if props.open == Some(true) || (props.open.is_none() && *visible.read()) {
                measure_popover_position(effect_positioning.clone());
            }
        },
    ));

    let interactions = PopoverInteractionSignals {
        hover_active,
        focus_active,
        click_active,
    };

    let enter_positioning = positioning.clone();
    let leave_positioning = positioning.clone();
    let focus_in_positioning = positioning.clone();
    let focus_out_positioning = positioning.clone();
    let click_positioning = positioning.clone();
    let dismiss_positioning = positioning;
    let should_render_backdrop =
        is_visible && props.open.is_none() && props.dismiss_on_outside_click;

    rsx! {
        if should_render_backdrop {
            div {
                class: "popover-backdrop",
                style: "position: fixed; inset: 0; z-index: 1069;",
                onclick: move |_| {
                    hover_active.set(false);
                    focus_active.set(false);
                    click_active.set(false);
                    schedule_popover_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        dismiss_positioning.clone(),
                    );
                },
            }
        }

        span {
            id: "{trigger_id}",
            class: "popover-wrapper",
            // `inline-flex`, not `inline-block`: an inline-block wrapper carries
            // line-box leading (descent below the baseline), so its measured box
            // extends past the trigger and the popover anchors a dozen-odd pixels
            // low. A flex wrapper has no line box, so it hugs the trigger and the
            // popover lands where Popper would put it against the element itself.
            style: if is_visible { "position: relative; display: inline-flex; z-index: 1070;" } else { "position: relative; display: inline-flex;" },
            "aria-describedby": "{describedby}",
            onmouseenter: move |_| {
                if props.trigger.hover && props.open.is_none() {
                    hover_active.set(true);
                    schedule_popover_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        enter_positioning.clone(),
                    );
                }
            },
            onmouseleave: move |_| {
                if props.trigger.hover && props.open.is_none() {
                    hover_active.set(false);
                    schedule_popover_visibility(
                        visible,
                        visibility_revision,
                        props.open,
                        props.trigger,
                        props.delay,
                        interactions,
                        leave_positioning.clone(),
                    );
                }
            },
            onfocusin: move |_| {
                if props.trigger.focus && props.open.is_none() {
                    focus_active.set(true);
                    schedule_popover_visibility(
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
                    schedule_popover_visibility(
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
            onclick: move |evt| {
                if props.trigger.click && props.open.is_none() {
                    evt.stop_propagation();
                    let next_click_active = {
                        let active = click_active.read();
                        !*active
                    };
                    click_active.set(next_click_active);
                    schedule_popover_visibility(
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
                    id: "{popover_id}",
                    class: "{popover_class}",
                    role: "tooltip",
                    "data-popper-placement": "{placement_value}",
                    style: "{popover_style}",
                    onclick: move |evt| evt.stop_propagation(),
                    div { class: "popover-arrow", style: "{arrow_style}" }
                    if !props.title.is_empty() {
                        h3 { class: "popover-header", "{props.title}" }
                    }
                    if element_has_content(&props.body) {
                        div { class: "popover-body", {props.body} }
                    }
                }
            }
        }
    }
}

fn next_popover_id() -> String {
    let id = NEXT_POPOVER_ID.fetch_add(1, Ordering::Relaxed);
    format!("dbcss-popover-{id}")
}

fn next_popover_trigger_id() -> String {
    let id = NEXT_POPOVER_TRIGGER_ID.fetch_add(1, Ordering::Relaxed);
    format!("dbcss-popover-trigger-{id}")
}

fn classes(base: &str, placement_class: &str, extra: &str) -> String {
    if extra.is_empty() {
        format!("{base} {placement_class}")
    } else {
        format!("{base} {placement_class} {extra}")
    }
}

fn popover_style(position: Option<OverlayPosition>) -> String {
    match position {
        Some(position) => format!(
            "position: fixed; left: {:.3}px; top: {:.3}px; z-index: 1070; visibility: visible;",
            position.x, position.y
        ),
        None => "position: fixed; left: 0; top: 0; z-index: 1070; visibility: hidden;".to_string(),
    }
}

/// Inline style that slides the `.popover-arrow` along the popover's cross axis so
/// it stays pointing at the trigger even after the popover box is clamped to the
/// viewport. Without this the arrow sits at its static position and misses the
/// trigger whenever the box is shifted — the gap left by not running Popper.js.
fn arrow_style(position: Option<OverlayPosition>, placement: PopoverPlacement) -> String {
    let Some(position) = position else {
        return String::new();
    };
    // `position.arrow` is the arrow centre in popover-local coordinates; the arrow
    // box is 1rem (16px), so its leading edge is the centre minus half that.
    //
    // `position: absolute` is required: Bootstrap positions the arrow along the main
    // edge (its `.bs-popover-*>.popover-arrow{top/bottom/left/right}` rules) but only
    // Popper.js makes the element absolutely positioned. Without it those rules and
    // this cross-axis offset are both ignored, so we set it here.
    let edge = position.arrow - 8.0;
    match placement {
        PopoverPlacement::Start | PopoverPlacement::End => {
            format!("position: absolute; top: {edge:.3}px;")
        }
        _ => format!("position: absolute; left: {edge:.3}px;"),
    }
}

fn schedule_popover_visibility(
    mut visible: Signal<bool>,
    mut revision: Signal<u64>,
    open: Option<bool>,
    trigger: PopoverTriggers,
    delay: PopoverDelay,
    interactions: PopoverInteractionSignals,
    mut positioning: PopoverPositioning,
) {
    if open.is_some() {
        return;
    }

    let next_revision = *revision.read() + 1;
    revision.set(next_revision);
    let should_show = (trigger.hover && *interactions.hover_active.read())
        || (trigger.focus && *interactions.focus_active.read())
        || (trigger.click && *interactions.click_active.read());
    let delay_ms = if should_show {
        delay.show_ms
    } else {
        delay.hide_ms
    };

    spawn(async move {
        if delay_ms > 0 {
            TimeoutFuture::new(delay_ms).await;
        }

        if *revision.read() == next_revision {
            visible.set(should_show);
            if should_show {
                measure_popover_position(positioning);
            } else {
                positioning.overlay_position.set(None);
            }
        }
    });
}

fn measure_popover_position(mut positioning: PopoverPositioning) {
    spawn(async move {
        let Some(trigger_rect) = element_rect(&positioning.trigger_id).await else {
            return;
        };
        let Some(popover_rect) = element_rect(&positioning.popover_id).await else {
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
                popover_rect,
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

fn element_has_content(element: &Element) -> bool {
    let Ok(vnode) = element else {
        return false;
    };

    vnode_has_content(vnode)
}

fn vnode_has_content(vnode: &VNode) -> bool {
    vnode
        .template
        .roots
        .iter()
        .any(template_node_has_static_content)
        || vnode.dynamic_nodes.iter().any(dynamic_node_has_content)
}

fn template_node_has_static_content(node: &TemplateNode) -> bool {
    match node {
        TemplateNode::Element { .. } => true,
        TemplateNode::Text { text } => !text.is_empty(),
        TemplateNode::Dynamic { .. } => false,
    }
}

fn dynamic_node_has_content(node: &DynamicNode) -> bool {
    match node {
        DynamicNode::Component(_) => true,
        DynamicNode::Text(text) => !text.value.is_empty(),
        DynamicNode::Placeholder(_) => false,
        DynamicNode::Fragment(nodes) => nodes.iter().any(vnode_has_content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_defaults_to_click() {
        let triggers = PopoverTriggers::default();
        assert!(!triggers.hover);
        assert!(!triggers.focus);
        assert!(triggers.click);
    }

    #[test]
    fn placement_defaults_to_bootstrap_end() {
        assert_eq!(PopoverPlacement::default(), PopoverPlacement::End);
    }

    #[test]
    fn placement_converts_to_overlay_placement() {
        assert_eq!(
            OverlayPlacement::from(PopoverPlacement::Auto),
            OverlayPlacement::Auto
        );
        assert_eq!(
            OverlayPlacement::from(PopoverPlacement::Bottom),
            OverlayPlacement::Bottom
        );
    }

    #[test]
    fn placement_classes_match_bootstrap() {
        assert_eq!(PopoverPlacement::Top.class(), "bs-popover-top");
        assert_eq!(PopoverPlacement::Bottom.class(), "bs-popover-bottom");
        assert_eq!(PopoverPlacement::Start.class(), "bs-popover-start");
        assert_eq!(PopoverPlacement::End.class(), "bs-popover-end");
    }
}
