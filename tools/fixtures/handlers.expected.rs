rsx! {
    div {
        class: "toolbar",
        button {
            class: "btn btn-sm btn-primary",
            // TODO(convert): onclick="refreshAll()" -> Dioxus onclick handler + signal
            "Refresh"
        }
        script {
            // TODO(convert): <script> block dropped — port it by hand
        }
    }
}
