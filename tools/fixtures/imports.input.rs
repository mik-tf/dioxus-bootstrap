use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::{Badge, Color};

fn app() -> Element {
    rsx! {
        button {
            class: "btn btn-sm btn-outline-secondary",
            "Open"
        }
        span { class: "badge text-bg-light", "1" }
    }
}
