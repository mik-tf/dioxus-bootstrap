use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        ul { class: "nav nav-pills", id: "adminTabs",
            li { class: "nav-item",
                a { class: "nav-link active", "data-tab": "overview", href: "#/overview", "Overview" }
            }
            li { class: "nav-item",
                a { class: "nav-link", href: "#/api", "API" }
            }
        }
    }
}
