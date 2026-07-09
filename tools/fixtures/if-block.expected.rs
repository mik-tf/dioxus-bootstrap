rsx! {
    div {
        class: "status",
        if running {
            span {
                class: "badge text-bg-success",
                "running"
            }
        } else {
            span {
                class: "badge text-bg-secondary",
                "stopped"
            }
        }
    }
}
