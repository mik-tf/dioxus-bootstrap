rsx! {
    div {
        class: "summary",
        p {
            class: "mb-0",
            "Version"
            code {
                {version}
            }
            "on host {hostname}."
        }
        a {
            class: "link",
            href: "{base_path}/services",
            "All services"
        }
    }
}
