//! Edge-align parity harness for the account-style dropdown.
//!
//! Unlike the popover/tooltip, the dbcss `Dropdown` does NOT use the Popper-style
//! placement engine: the menu is CSS-anchored (`.dropdown-menu` absolutely
//! positioned under the toggle, right-aligned via Bootstrap's own
//! `.dropdown-menu-end { right:0; left:auto }`). So there is no arrow to track and
//! no run-time measurement that could mis-clamp. This harness pins the toggle at
//! the same top-right point as the golden and forces the menu open, so the gate
//! can confirm the menu lands where classic Bootstrap+Popper puts it.

use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let open = use_signal(|| true);
    rsx! {
        BootstrapHead {}
        div { style: "position: fixed; top: 12px; right: 24px; z-index: 10; display: flex;",
            Dropdown {
                open,
                align_end: true,
                toggle_class: "btn-sm btn-light",
                toggle: rsx! { "mik-tf" },
                menu: rsx! {
                    DropdownItem { href: "#", "Settings" }
                    DropdownItem { href: "#", "Account" }
                    DropdownItem { href: "#", class: "text-danger", "Log out" }
                },
            }
        }
    }
}
