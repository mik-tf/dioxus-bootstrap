use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        ul { class: "navbar-nav",
            li { class: "nav-item", a { class: "nav-link active", href: "#", "Home" } }
        }
        ul { class: "navbar-nav navbar-nav-scroll ms-auto",
            li { class: "nav-item", a { class: "nav-link", href: "#docs", "Docs" } }
        }
    }
}
