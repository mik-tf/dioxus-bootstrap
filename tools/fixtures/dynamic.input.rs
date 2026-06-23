use dioxus::prelude::*;

fn app() -> Element {
    let active = true;
    rsx! {
        button {
            class: if active { "btn btn-primary" } else { "btn btn-secondary" },
            "Save"
        }
    }
}
