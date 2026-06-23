use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        div {
            id: "docs-scroll",
            "data-bs-spy": "scroll",
            "data-bs-target": "#docs-nav",
            tabindex: "0",
            section { id: "intro", "Intro" }
        }
    }
}
