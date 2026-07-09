rsx! {
    form {
        action: "/save",
        method: "post",
        div {
            class: "mb-2",
            label {
                class: "form-label",
                r#for: "name",
                "Name"
            }
            input {
                class: "form-control",
                r#type: "text",
                id: "name",
                name: "name",
                required: true,
            }
        }
        div {
            class: "mb-2",
            label {
                class: "form-label",
                r#for: "role",
                "Role"
            }
            select {
                class: "form-select",
                id: "role",
                name: "role",
                option {
                    value: "admin",
                    "Admin"
                }
                option {
                    value: "user",
                    selected: true,
                    "User"
                }
            }
        }
        button {
            class: "btn btn-primary",
            r#type: "submit",
            "Save"
        }
    }
}
