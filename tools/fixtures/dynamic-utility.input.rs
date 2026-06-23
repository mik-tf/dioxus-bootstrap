use dioxus::prelude::*;

fn app(stream: &str) -> Element {
    rsx! {
        div {
            class: if stream == "stderr" { "text-danger" } else { "" },
            "line"
        }
        div {
            class: if true { "flex-grow-1 d-flex flex-column" } else { "flex-shrink-0 d-flex flex-column" },
            "layout"
        }
    }
}
