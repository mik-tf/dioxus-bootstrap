rsx! {
    div {
        class: "card mb-3",
        div {
            class: "card-header py-2",
            i {
                class: "bi bi-info-circle",
            }
            "Details"
        }
        div {
            class: "card-body",
            h5 {
                class: "card-title",
                "Overview"
            }
            p {
                class: "card-text",
                "A static card with an icon & some text."
            }
            a {
                class: "btn btn-primary",
                href: "/docs",
                target: "_blank",
                "Open docs"
            }
        }
    }
}
