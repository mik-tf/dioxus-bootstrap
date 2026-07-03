use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::{Nav, NavItem, NavLink};

fn app() -> Element {
    rsx! {
        Nav {
            pills: true,
            id: "adminTabs",
            NavItem {
                NavLink {
                    active: true,
                    "data-tab": "overview", href: "#/overview", "Overview"
                }
            }
            NavItem {
                NavLink {
                    href: "#/api", "API"
                }
            }
        }
    }
}
