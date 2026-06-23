use dioxus::prelude::*;

fn refresh() {}

fn app() -> Element {
    let email = "a@example.com";
    rsx! {
        button {
            class: "btn btn-sm btn-outline-secondary border-0",
            onclick: move |_| refresh(),
            "Refresh"
        }
        button {
            class: "btn btn-sm",
            "Cancel"
        }
        a {
            class: "btn btn-primary",
            href: "/docs",
            target: "_blank",
            "Docs"
        }
        span {
            class: "badge text-bg-danger rounded-pill ms-1",
            "9"
        }
        div {
            class: "alert alert-warning alert-dismissible fade show mb-2",
            "Careful"
        }
        div {
            class: "spinner-grow spinner-grow-sm text-success",
            "Loading"
        }
        input {
            class: "form-control form-control-sm mb-2",
            r#type: "email",
            value: "{email}",
        }
        select {
            class: "form-select form-select-lg",
            option { "One" }
        }
        textarea {
            class: "form-control form-control-lg",
            "notes"
        }
        table {
            class: "table table-striped table-hover table-sm mb-0",
            tbody {
                tr { td { "A" } }
            }
        }
        div {
            class: "card mb-3",
            div { class: "card-header py-2", "Title" }
            div {
                class: "card-body",
                div {
                    button {
                        class: "btn btn-sm btn-outline-secondary",
                        "Nested"
                    }
                }
            }
            div { class: "card-footer text-muted", "Footer" }
        }
    }
}
