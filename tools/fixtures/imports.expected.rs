use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::{Badge, Button, Color, Size};

fn app() -> Element {
    rsx! {
        Button {
            size: Size::Sm,
            color: Color::Secondary,
            outline: true,
            "Open"
        }
        Badge {
            color: Color::Light,
            "1"
        }
    }
}
