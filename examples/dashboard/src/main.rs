use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let active_tab = use_signal(|| 0usize);
    let nav_collapsed = use_signal(|| true);
    let theme = use_signal(|| Theme::Dark);
    let custom_theme = BootstrapTheme {
        colors: ThemeColors {
            primary: Some(SemanticColorScale::new("#165317")),
            secondary: Some(SemanticColorScale::new("#5c6b5d")),
            success: Some(SemanticColorScale::new("#2f9e44")),
            info: Some(SemanticColorScale::new("#0b7285")),
            warning: Some(SemanticColorScale::new("#e67700")),
            danger: Some(SemanticColorScale::new("#c92a2a")),
            light: Some(SemanticColorScale::new("#f5f8f5")),
            dark: Some(SemanticColorScale::new("#092817")),
        },
        surfaces: SurfaceColors {
            body_bg: Some("#f7fbf7".into()),
            body_color: Some("#1d2b1f".into()),
            secondary_bg: Some("#edf3ed".into()),
            secondary_color: Some("#314033".into()),
            tertiary_bg: None,
            tertiary_color: None,
            border_color: Some("#d4dfd4".into()),
            link_color: None,
            link_hover_color: None,
        },
        dark: Some(ThemeModeTokens {
            colors: ThemeColors {
                primary: Some(SemanticColorScale::new("#4e9f53")),
                secondary: Some(SemanticColorScale::new("#7b8b7d")),
                success: Some(SemanticColorScale::new("#51cf66")),
                info: Some(SemanticColorScale::new("#3bc9db")),
                warning: Some(SemanticColorScale::new("#f08c00")),
                danger: Some(SemanticColorScale::new("#ff6b6b")),
                light: Some(SemanticColorScale::new("#e9f2ea")),
                dark: Some(SemanticColorScale::new("#081e12")),
            },
            surfaces: SurfaceColors {
                body_bg: Some("#0b120c".into()),
                body_color: Some("#e6efe7".into()),
                secondary_bg: Some("#162019".into()),
                secondary_color: Some("#c7d3c8".into()),
                tertiary_bg: None,
                tertiary_color: None,
                border_color: Some("#2a382d".into()),
                link_color: Some("#74c97d".into()),
                link_hover_color: Some("#95d89c".into()),
            },
        }),
    };

    rsx! {
        ThemeProvider { theme: theme }
        BootstrapHead {}
        BootstrapThemeProvider { theme: custom_theme }

        // Navbar
        Navbar { color: Color::Dark, expand: NavbarExpand::Lg,
            brand: rsx! {
                a { class: "navbar-brand", href: "#",
                    Icon { name: "speedometer2", class: "me-2" }
                    "Dashboard"
                }
            },
            NavbarToggler { collapsed: nav_collapsed }
            NavbarCollapse { collapsed: nav_collapsed,
                NavItem { NavLink { href: "#", active: true, "Overview" } }
                NavItem { NavLink { href: "#", "Services" } }
                NavItem { NavLink { href: "#", "Settings" } }
            }
            ThemeToggle { theme: theme }
        }

        // Main content
        Container { fluid: true, class: "py-4",
            // Breadcrumb
            Breadcrumb {
                BreadcrumbItem { href: "#", "Home" }
                BreadcrumbItem { active: true, "Dashboard" }
            }

            Row { class: "g-3",
                // Sidebar — stats
                Col { lg: ColumnSize::Span(3),
                    StatsCard {}
                }

                // Main — tabbed content
                Col { lg: ColumnSize::Span(9),
                    TabList {
                        active: active_tab,
                        tabs: vec![
                            TabDef {
                                label: "Overview".into(),
                                icon: Some("speedometer2".into()),
                                content: rsx! { OverviewTab {} },
                            },
                            TabDef {
                                label: "Services".into(),
                                icon: Some("diagram-3".into()),
                                content: rsx! { ServicesTab {} },
                            },
                            TabDef {
                                label: "Logs".into(),
                                icon: Some("terminal".into()),
                                content: rsx! { LogsTab {} },
                            },
                        ],
                    }
                }
            }
        }
    }
}

#[component]
fn StatsCard() -> Element {
    rsx! {
        Card {
            header: rsx! {
                div { class: "d-flex justify-content-between align-items-center",
                    span { Icon { name: "bar-chart", class: "me-1" } "Stats" }
                    Button { color: Color::Primary, outline: true, size: Size::Sm,
                        Icon { name: "arrow-clockwise" }
                    }
                }
            },
            body: rsx! {
                div { class: "mb-3",
                    small { class: "text-muted", "Total Services" }
                    h4 { "12" }
                }
                div { class: "mb-3",
                    small { class: "text-muted", "Running" }
                    h4 { class: "text-success", "10" }
                }
                div { class: "mb-3",
                    small { class: "text-muted", "Stopped" }
                    h4 { class: "text-danger", "2" }
                }
                div { class: "mb-3",
                    small { class: "text-muted", "Uptime" }
                    Progress {
                        ProgressBar { value: 99.7, color: Color::Success, show_label: true }
                    }
                }
                hr {}
                div {
                    small { class: "text-muted", "CPU Usage" }
                    Progress { class: "mt-1 mb-2",
                        ProgressBar { value: 42.0, color: Color::Info }
                    }
                    small { class: "text-muted", "Memory" }
                    Progress { class: "mt-1",
                        ProgressBar { value: 68.0, color: Color::Warning }
                    }
                }
            },
        }
    }
}

#[component]
fn OverviewTab() -> Element {
    rsx! {
        div { class: "mt-3",
            // Status cards row
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(3),
                    Card { class: "text-bg-primary",
                        body: rsx! {
                            div { class: "d-flex justify-content-between",
                                div {
                                    h6 { class: "card-title mb-0", "Requests" }
                                    h3 { class: "mb-0", "1.2M" }
                                }
                                Icon { name: "arrow-up-right", class: "fs-1 opacity-50" }
                            }
                        },
                    }
                }
                Col { md: ColumnSize::Span(3),
                    Card { class: "text-bg-success",
                        body: rsx! {
                            div { class: "d-flex justify-content-between",
                                div {
                                    h6 { class: "card-title mb-0", "Success Rate" }
                                    h3 { class: "mb-0", "99.8%" }
                                }
                                Icon { name: "check-circle", class: "fs-1 opacity-50" }
                            }
                        },
                    }
                }
                Col { md: ColumnSize::Span(3),
                    Card { class: "text-bg-warning",
                        body: rsx! {
                            div { class: "d-flex justify-content-between",
                                div {
                                    h6 { class: "card-title mb-0", "Avg Latency" }
                                    h3 { class: "mb-0", "45ms" }
                                }
                                Icon { name: "clock", class: "fs-1 opacity-50" }
                            }
                        },
                    }
                }
                Col { md: ColumnSize::Span(3),
                    Card { class: "text-bg-danger",
                        body: rsx! {
                            div { class: "d-flex justify-content-between",
                                div {
                                    h6 { class: "card-title mb-0", "Errors" }
                                    h3 { class: "mb-0", "23" }
                                }
                                Icon { name: "exclamation-triangle", class: "fs-1 opacity-50" }
                            }
                        },
                    }
                }
            }

            // Recent activity
            Card {
                header: rsx! { "Recent Activity" },
                body: rsx! {
                    ListGroup { flush: true,
                        ListGroupItem {
                            div { class: "d-flex justify-content-between align-items-center",
                                div {
                                    Icon { name: "arrow-up-circle", class: "text-success me-2" }
                                    "API Gateway scaled to 3 replicas"
                                }
                                Badge { color: Color::Success, pill: true, "Scale Up" }
                            }
                        }
                        ListGroupItem {
                            div { class: "d-flex justify-content-between align-items-center",
                                div {
                                    Icon { name: "exclamation-circle", class: "text-warning me-2" }
                                    "Cache hit rate dropped below 90%"
                                }
                                Badge { color: Color::Warning, pill: true, "Warning" }
                            }
                        }
                        ListGroupItem {
                            div { class: "d-flex justify-content-between align-items-center",
                                div {
                                    Icon { name: "x-circle", class: "text-danger me-2" }
                                    "Worker process crashed — auto-restart triggered"
                                }
                                Badge { color: Color::Danger, pill: true, "Error" }
                            }
                        }
                        ListGroupItem {
                            div { class: "d-flex justify-content-between align-items-center",
                                div {
                                    Icon { name: "gear", class: "text-info me-2" }
                                    "Configuration updated for auth service"
                                }
                                Badge { color: Color::Info, pill: true, "Config" }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn ServicesTab() -> Element {
    let mut delete_modal = use_signal(|| false);

    rsx! {
        div { class: "mt-3",
            div { class: "d-flex justify-content-between align-items-center mb-3",
                h4 { class: "mb-0", "Services" }
                Button { color: Color::Primary, size: Size::Sm,
                    Icon { name: "plus-lg", class: "me-1" }
                    "Add Service"
                }
            }

            Table { striped: true, hover: true, responsive: true,
                thead {
                    tr {
                        th { "#" }
                        th { "Name" }
                        th { "Status" }
                        th { "Port" }
                        th { "Uptime" }
                        th { "Actions" }
                    }
                }
                tbody {
                    tr {
                        td { "1" }
                        td {
                            Icon { name: "globe", class: "me-1" }
                            "API Gateway"
                        }
                        td { Badge { color: Color::Success, "Running" } }
                        td { code { "8080" } }
                        td { "3d 14h" }
                        td { ServiceActions { delete_modal: delete_modal } }
                    }
                    tr {
                        td { "2" }
                        td {
                            Icon { name: "database", class: "me-1" }
                            "Database"
                        }
                        td { Badge { color: Color::Success, "Running" } }
                        td { code { "5432" } }
                        td { "7d 2h" }
                        td { ServiceActions { delete_modal: delete_modal } }
                    }
                    tr {
                        td { "3" }
                        td {
                            Icon { name: "lightning", class: "me-1" }
                            "Cache"
                        }
                        td { Badge { color: Color::Warning, "Degraded" } }
                        td { code { "6379" } }
                        td { "1d 8h" }
                        td { ServiceActions { delete_modal: delete_modal } }
                    }
                    tr {
                        td { "4" }
                        td {
                            Icon { name: "cpu", class: "me-1" }
                            "Worker"
                        }
                        td { Badge { color: Color::Danger, "Down" } }
                        td { code { "—" } }
                        td { "—" }
                        td { ServiceActions { delete_modal: delete_modal } }
                    }
                }
            }

            // Delete confirmation modal
            Modal {
                show: delete_modal,
                title: "Confirm Delete".to_string(),
                centered: true,
                body: rsx! {
                    Alert { color: Color::Danger,
                        Icon { name: "exclamation-triangle", class: "me-2" }
                        "This action cannot be undone. The service and all its data will be permanently removed."
                    }
                },
                footer: rsx! {
                    Button { color: Color::Secondary, onclick: move |_| delete_modal.set(false), "Cancel" }
                    Button { color: Color::Danger,
                        Icon { name: "trash", class: "me-1" }
                        "Delete Service"
                    }
                },
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct ServiceActionsProps {
    delete_modal: Signal<bool>,
}

#[component]
fn ServiceActions(props: ServiceActionsProps) -> Element {
    let dropdown = use_signal(|| false);
    let mut delete_modal = props.delete_modal;

    rsx! {
        Dropdown {
            open: dropdown,
            toggle_class: "btn-outline-secondary btn-sm".to_string(),
            toggle: rsx! { Icon { name: "three-dots-vertical" } },
            menu: rsx! {
                DropdownItem { Icon { name: "play", class: "me-2" } "Start" }
                DropdownItem { Icon { name: "stop", class: "me-2" } "Stop" }
                DropdownItem { Icon { name: "arrow-clockwise", class: "me-2" } "Restart" }
                DropdownDivider {}
                DropdownItem {
                    onclick: move |_| delete_modal.set(true),
                    span { class: "text-danger",
                        Icon { name: "trash", class: "me-2" }
                        "Delete"
                    }
                }
            },
        }
    }
}

#[component]
fn LogsTab() -> Element {
    rsx! {
        div { class: "mt-3",
            div { class: "d-flex justify-content-between align-items-center mb-3",
                h4 { class: "mb-0", "Logs" }
                div { class: "d-flex gap-2",
                    Select { size: Size::Sm,
                        option { value: "all", "All Services" }
                        option { value: "api", "API Gateway" }
                        option { value: "db", "Database" }
                        option { value: "cache", "Cache" }
                        option { value: "worker", "Worker" }
                    }
                    Button { color: Color::Primary, outline: true, size: Size::Sm,
                        Icon { name: "arrow-clockwise", class: "me-1" }
                        "Refresh"
                    }
                }
            }

            Card {
                body_class: "p-0".to_string(),
                body: rsx! {
                    div { class: "font-monospace small p-3 bg-dark text-light",
                        style: "max-height: 400px; overflow-y: auto;",
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:01] " }
                            Badge { color: Color::Info, class: "me-1", "API" }
                            "GET /api/v1/services → 200 (12ms)"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:02] " }
                            Badge { color: Color::Info, class: "me-1", "API" }
                            "POST /api/v1/deploy → 201 (145ms)"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:03] " }
                            Badge { color: Color::Warning, class: "me-1", "Cache" }
                            "WARN: Cache miss rate 12% — consider increasing capacity"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:04] " }
                            Badge { color: Color::Danger, class: "me-1", "Worker" }
                            "ERROR: Process exited with code 137 (OOMKilled)"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:05] " }
                            Badge { color: Color::Success, class: "me-1", "Worker" }
                            "INFO: Auto-restart triggered — attempt 1/3"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:08] " }
                            Badge { color: Color::Success, class: "me-1", "Worker" }
                            "INFO: Process started successfully (PID 4521)"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:10] " }
                            Badge { color: Color::Info, class: "me-1", "DB" }
                            "INFO: Connection pool stats — active: 8, idle: 12, max: 20"
                        }
                        div { class: "mb-1",
                            span { class: "text-muted", "[12:34:12] " }
                            Badge { color: Color::Info, class: "me-1", "API" }
                            "GET /api/v1/health → 200 (2ms)"
                        }
                    }
                },
            }
        }
    }
}
