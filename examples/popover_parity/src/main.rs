//! Edge-clamp parity harness for the popover arrow.
//!
//! The trigger dot is pinned at a fixed top-right viewport point — the same
//! point the console top-bar's connection-status dot occupies. At 1440x900 the
//! bottom-placed popover overflows the right edge and is clamped left, so the
//! arrow must offset to keep pointing at the trigger. That is the exact case
//! Popper.js handles for a real Bootstrap popover and the case this crate has to
//! reproduce without Popper.
//!
//! The body mirrors the classic `<hero-connection-status>` popover's connected
//! state byte-for-byte (see hero_admin_lib's `connection-status.js` `_popoverHTML`)
//! so a golden-vs-candidate pixel diff isolates the arrow/placement, not content.
//! The "Last checked" time is a fixed string here (the golden's is live) — crop
//! the diff to the arrow + header strip, which excludes it.

use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        BootstrapHead {}
        // `display: flex` on the pin, mirroring the console top-bar (a flex row)
        // and the golden's inline-flex host: a plain block here would add its own
        // line box and drop the dot ~8px, which is a harness artefact, not the
        // control's placement. Keep the harness faithful so the gate measures the
        // control, not the page around it.
        div { style: "position: fixed; top: 16px; right: 24px; z-index: 10; display: flex;",
            Popover {
                title: "Connection Status",
                placement: PopoverPlacement::Bottom,
                body: rsx! {
                    div { style: "min-width:220px",
                        div { style: "font-weight:600;margin-bottom:8px;color:#3fb950", "Connected" }
                        table { style: "width:100%;border-collapse:collapse;font-size:0.82rem",
                            tr {
                                td { "\u{2705} UI Server" }
                                td { style: "font-size:0.75rem;color:var(--bs-secondary-color)", "Hero Cockpit v0.1.0" }
                            }
                        }
                        div { style: "margin-top:8px;font-size:0.7rem;color:var(--bs-secondary-color)", "Last checked: 9:00:00 AM" }
                        div { style: "margin-top:6px",
                            button {
                                style: "font-size:0.72rem;padding:2px 8px;background:transparent;border:1px solid var(--border-color,#444);border-radius:4px;color:inherit;cursor:pointer",
                                "\u{21ba} Recheck now"
                            }
                        }
                    }
                },
                span {
                    style: "display:inline-block;width:10px;height:10px;border-radius:50%;background:#4ade80;cursor:pointer;",
                }
            }
        }
    }
}
