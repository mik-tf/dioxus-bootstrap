use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        button {
            class: "btn btn-primary",
            "data-bs-toggle": "tooltip",
            "data-bs-placement": "bottom",
            "data-bs-trigger": "click",
            "data-bs-custom-class": "tip-wide",
            "data-bs-html": "false",
            title: "Save changes",
            "Save"
        }
        button {
            class: "btn btn-secondary",
            "data-bs-toggle": "popover",
            "data-bs-title": "Details",
            "data-bs-content": "More information",
            "data-bs-placement": "left",
            "data-bs-trigger": "focus",
            "data-bs-html": "false",
            "More"
        }
    }
}
