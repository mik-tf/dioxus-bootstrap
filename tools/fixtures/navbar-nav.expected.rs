use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::{NavbarNav};

fn app() -> Element {
    rsx! {
        NavbarNav {
            li { class: "nav-item", a { class: "nav-link active", href: "#", "Home" } }
        }
        NavbarNav {
            scroll: true,
            class: "ms-auto",
            li { class: "nav-item", a { class: "nav-link", href: "#docs", "Docs" } }
        }
    }
}
