use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::{
    Button,
    Color,
    Popover,
    PopoverPlacement,
    PopoverTriggers,
    Tooltip,
    TooltipPlacement,
    TooltipTriggers,
};

fn app() -> Element {
    rsx! {
        Tooltip {
            text: "Save changes",
            placement: TooltipPlacement::Bottom,
            trigger: TooltipTriggers::CLICK,
            class: "tip-wide",
            Button {
                color: Color::Primary,
                "Save"
            }
        }
        Popover {
            title: "Details",
            body: rsx! { "More information" },
            placement: PopoverPlacement::Start,
            trigger: PopoverTriggers::FOCUS,
            Button {
                color: Color::Secondary,
                "More"
            }
        }
    }
}
