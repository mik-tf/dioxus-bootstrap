use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::{
    Alert,
    Badge,
    Button,
    Card,
    Color,
    Input,
    Select,
    Size,
    Spinner,
    SpinnerStyle,
    Table,
    Textarea,
};

fn refresh() {}

fn app() -> Element {
    let email = "a@example.com";
    rsx! {
        Button {
            size: Size::Sm,
            color: Color::Secondary,
            outline: true,
            class: "border-0",
            onclick: move |_| refresh(),
            "Refresh"
        }
        Button {
            plain: true,
            size: Size::Sm,
            "Cancel"
        }
        Button {
            color: Color::Primary,
            href: "/docs",
            target: "_blank",
            "Docs"
        }
        Badge {
            color: Color::Danger,
            pill: true,
            class: "ms-1",
            "9"
        }
        Alert {
            color: Color::Warning,
            dismissible: true,
            class: "mb-2",
            "Careful"
        }
        Spinner {
            style: SpinnerStyle::Grow,
            size: Size::Sm,
            color: Some(Color::Success),
            "style": "width:16px;height:16px;",
            "Loading"
        }
        Input {
            size: Size::Sm,
            class: "mb-2",
            r#type: "email",
            value: "{email}",
        }
        Select {
            size: Size::Lg,
            option { "One" }
        }
        Textarea {
            size: Size::Lg,
            "notes"
        }
        Table {
            striped: true,
            hover: true,
            size: Size::Sm,
            class: "mb-0",
            tbody {
                tr { td { "A" } }
            }
        }
        Card {
            class: "mb-3",
            header_class: "py-2",
            header: rsx! {
                "Title"
            },
            body: rsx! {
                div {
                    Button {
                        size: Size::Sm,
                        color: Color::Secondary,
                        outline: true,
                        "Nested"
                    }
                }
            },
            footer_class: "text-muted",
            footer: rsx! {
                "Footer"
            },
        }
    }
}
