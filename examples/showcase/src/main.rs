use dioxus::prelude::*;
use dioxus_bootstrap_css::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let active_tab = use_signal(|| 0usize);
    let theme = use_signal(|| Theme::Dark);

    rsx! {
        ThemeProvider { theme: theme }
        BootstrapHead {}
        NavbarDemo { theme: theme }
        Container { class: "py-4",
            h1 { class: "mb-4", "dioxus-bootstrap Showcase" }
            p { class: "lead mb-4",
                "All Bootstrap 5.3 components rendered by Dioxus — zero JavaScript."
            }

            TabList {
                active: active_tab,
                tabs: vec![
                    TabDef {
                        label: "Basics".into(),
                        icon: Some("grid".into()),
                        content: rsx! { BasicsSection {} },
                    },
                    TabDef {
                        label: "Forms".into(),
                        icon: Some("input-cursor-text".into()),
                        content: rsx! { FormsSection {} },
                    },
                    TabDef {
                        label: "Data".into(),
                        icon: Some("table".into()),
                        content: rsx! { DataSection {} },
                    },
                    TabDef {
                        label: "Interactive".into(),
                        icon: Some("lightning".into()),
                        content: rsx! { InteractiveSection {} },
                    },
                    TabDef {
                        label: "Overlays".into(),
                        icon: Some("chat-square".into()),
                        content: rsx! { OverlaysSection {} },
                    },
                    TabDef {
                        label: "Media".into(),
                        icon: Some("image".into()),
                        content: rsx! { MediaSection {} },
                    },
                    TabDef {
                        label: "Navigation".into(),
                        icon: Some("signpost".into()),
                        content: rsx! { NavigationSection {} },
                    },
                    TabDef {
                        label: "More".into(),
                        icon: Some("plus-circle".into()),
                        content: rsx! { MoreSection {} },
                    },
                ],
            }

            // Docs section — scroll target for navbar link
            div { id: "docs-section", class: "mt-5 pt-4 border-top",
                h2 { class: "mb-3",
                    Icon { name: "book", class: "me-2" }
                    "Component Reference"
                }
                p { class: "lead", "All Bootstrap 5.3 components available in dioxus-bootstrap-css." }

                Row { class: "g-4",
                    Col { md: ColumnSize::Span(4),
                        h5 { "Layout" }
                        ul {
                            li { "Container / Container-fluid" }
                            li { "Row / Col (offset, order)" }
                            li { "BootstrapHead" }
                            li { "ThemeProvider / ThemeToggle" }
                        }
                        h5 { class: "mt-3", "Content" }
                        ul {
                            li { "Button / ButtonGroup / ButtonToolbar" }
                            li { "Card (header, body, footer)" }
                            li { "Alert (dismissible)" }
                            li { "Badge (pill)" }
                            li { "Icon (Bootstrap Icons)" }
                            li { "Spinner (border, grow)" }
                            li { "Progress / ProgressBar" }
                            li { "Placeholder / PlaceholderParagraph" }
                            li { "Figure / Ratio" }
                        }
                    }
                    Col { md: ColumnSize::Span(4),
                        h5 { "Forms" }
                        ul {
                            li { "Input / Select / Textarea" }
                            li { "Checkbox / Radio / Switch" }
                            li { "Range (slider)" }
                            li { "FloatingLabel" }
                            li { "FormGroup / FormFeedback / FormText" }
                            li { "InputGroup / InputGroupText" }
                        }
                        h5 { class: "mt-3", "Data Display" }
                        ul {
                            li { "Table (striped, hover, caption)" }
                            li { "ListGroup / ListGroupItem" }
                            li { "Pagination" }
                        }
                    }
                    Col { md: ColumnSize::Span(4),
                        h5 { "Interactive (Signal-Driven)" }
                        ul {
                            li { "Modal (sizes, fullscreen)" }
                            li { "Dropdown (split, directions)" }
                            li { "Collapse" }
                            li { "Tabs / Tab / TabList" }
                            li { "Accordion / AccordionItem" }
                            li { "Offcanvas (placements)" }
                            li { "Toast / ToastContainer" }
                            li { "Carousel" }
                            li { "Tooltip" }
                            li { "Popover" }
                            li { "Scrollspy" }
                        }
                        h5 { class: "mt-3", "Navigation" }
                        ul {
                            li { "Navbar / NavbarToggler / NavbarCollapse / NavbarNav" }
                            li { "Nav (pills, tabs, underline, fill)" }
                            li { "NavItem / NavLink" }
                            li { "Breadcrumb / BreadcrumbItem" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct NavbarDemoProps {
    theme: Signal<Theme>,
}

#[component]
fn NavbarDemo(props: NavbarDemoProps) -> Element {
    let collapsed = use_signal(|| true);

    rsx! {
        Navbar {
            expand: NavbarExpand::Lg,
            class: "sticky-top border-bottom bg-body",
            brand: rsx! { a { class: "navbar-brand", href: "#", Icon { name: "bootstrap", class: "me-2" } "dioxus-bootstrap-css" } },
            NavbarToggler { collapsed: collapsed }
            NavbarCollapse { collapsed: collapsed,
                NavbarNav {
                    NavItem { NavLink { href: "#", active: true, "Showcase" } }
                    NavItem {
                        NavLink { href: "#docs-section", "Docs" }
                    }
                }
            }
            ThemeToggle { theme: props.theme }
        }
    }
}

// ── Basics ──────────────────────────────────────────────────────────────────

#[component]
fn BasicsSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Buttons
            h3 { class: "mb-3", "Buttons" }
            div { class: "d-flex flex-wrap gap-2 mb-3",
                Button { color: Color::Primary, "Primary" }
                Button { color: Color::Secondary, "Secondary" }
                Button { color: Color::Success, "Success" }
                Button { color: Color::Danger, "Danger" }
                Button { color: Color::Warning, "Warning" }
                Button { color: Color::Info, "Info" }
                Button { color: Color::Light, "Light" }
                Button { color: Color::Dark, "Dark" }
            }
            div { class: "d-flex flex-wrap gap-2 mb-3",
                Button { color: Color::Primary, outline: true, "Outline" }
                Button { color: Color::Primary, size: Size::Sm, "Small" }
                Button { color: Color::Primary, size: Size::Lg, "Large" }
                Button { color: Color::Primary, disabled: true, "Disabled" }
                Button { color: Color::Success, active: true, "Active" }
                Button { color: Color::Primary, href: "https://getbootstrap.com/", target: "_blank",
                    Icon { name: "box-arrow-up-right", class: "me-1" }
                    "Link Button"
                }
                Button { size: Size::Sm, href: "/example.json", download: "example.json",
                    Icon { name: "download", class: "me-1" }
                    "Download"
                }
            }

            // Button Group & Toolbar
            h4 { class: "mb-2 mt-3", "Button Group & Toolbar" }
            ButtonGroup { class: "mb-3",
                Button { color: Color::Primary, "Left" }
                Button { color: Color::Primary, "Middle" }
                Button { color: Color::Primary, "Right" }
            }
            ButtonToolbar { class: "mb-4 gap-2",
                ButtonGroup {
                    Button { color: Color::Primary, "1" }
                    Button { color: Color::Primary, "2" }
                    Button { color: Color::Primary, "3" }
                }
                ButtonGroup {
                    Button { color: Color::Secondary, "A" }
                    Button { color: Color::Secondary, "B" }
                }
            }

            // Grid
            h3 { class: "mb-3", "Grid" }
            Row { class: "g-3 mb-3",
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-primary-subtle border rounded", "col-md-4" }
                }
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-primary-subtle border rounded", "col-md-4" }
                }
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-primary-subtle border rounded", "col-md-4" }
                }
            }

            // Grid with offset and order
            h4 { class: "mb-2", "Grid Offset & Order" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4), offset_md: Some(2),
                    div { class: "p-3 bg-info-subtle border rounded", "col-md-4 offset-md-2" }
                }
                Col { md: ColumnSize::Span(4),
                    div { class: "p-3 bg-info-subtle border rounded", "col-md-4" }
                }
            }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4), order: Some(3),
                    div { class: "p-3 bg-warning-subtle border rounded", "First in source, order-3" }
                }
                Col { md: ColumnSize::Span(4), order: Some(1),
                    div { class: "p-3 bg-warning-subtle border rounded", "Second in source, order-1" }
                }
                Col { md: ColumnSize::Span(4), order: Some(2),
                    div { class: "p-3 bg-warning-subtle border rounded", "Third in source, order-2" }
                }
            }

            // Cards
            h3 { class: "mb-3", "Cards" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4),
                    Card {
                        header: rsx! { "Card Header" },
                        body: rsx! {
                            h5 { class: "card-title", "Card Title" }
                            p { class: "card-text", "Some quick example text." }
                            Button { color: Color::Primary, size: Size::Sm, "Go somewhere" }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            h5 { class: "card-title", "Simple Card" }
                            p { class: "card-text", "A card with just a body." }
                        },
                        footer: rsx! { small { class: "text-muted", "2 days ago" } },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card { class: "text-bg-primary",
                        body: rsx! {
                            h5 { class: "card-title", "Colored Card" }
                            p { class: "card-text", "A primary-colored card." }
                        },
                    }
                }
            }

            // Alerts
            h3 { class: "mb-3", "Alerts" }
            Alert { color: Color::Primary, "A simple primary alert." }
            Alert { color: Color::Success, "A simple success alert." }
            Alert { color: Color::Danger, dismissible: true, "A dismissible danger alert." }
            Alert { color: Color::Warning, dismissible: true, "A dismissible warning alert." }
            Alert { color: Color::Info, dismissible: true,
                on_dismiss: move |_| { /* handle dismiss, e.g., clear state */ },
                Icon { name: "info-circle", class: "me-2" }
                "Dismissible with on_dismiss callback — check the console."
            }

            // Badges
            h3 { class: "mb-3 mt-4", "Badges" }
            div { class: "d-flex flex-wrap gap-2 mb-4",
                Badge { color: Color::Primary, "Primary" }
                Badge { color: Color::Secondary, "Secondary" }
                Badge { color: Color::Success, "Success" }
                Badge { color: Color::Danger, pill: true, "Danger Pill" }
                Badge { color: Color::Warning, pill: true, "Warning Pill" }
                Badge { color: Color::Info, "Info" }
            }

            // Icons
            h3 { class: "mb-3", "Icons" }
            div { class: "d-flex flex-wrap gap-3 fs-4 mb-4",
                Icon { name: "house" }
                Icon { name: "search" }
                Icon { name: "gear" }
                Icon { name: "person" }
                Icon { name: "shield-lock" }
                Icon { name: "lightning" }
                Icon { name: "book" }
                Icon { name: "terminal" }
                Icon { name: "speedometer2" }
                Icon { name: "diagram-3" }
            }

            // Breadcrumb
            h3 { class: "mb-3", "Breadcrumb" }
            Breadcrumb {
                BreadcrumbItem { href: "#", "Home" }
                BreadcrumbItem { href: "#", "Products" }
                BreadcrumbItem { active: true, "Current Page" }
            }
        }
    }
}

// ── Forms ───────────────────────────────────────────────────────────────────

#[component]
fn FormsSection() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut accept = use_signal(|| false);
    let mut selected = use_signal(String::new);
    let mut switch_on = use_signal(|| true);
    let mut range_val = use_signal(|| "50".to_string());

    rsx! {
        div { class: "mt-3",
            Row { class: "g-3",
                Col { lg: ColumnSize::Span(6),
                    Card {
                        header: rsx! { "Login Form" },
                        body: rsx! {
                            FormGroup { label: "Email address",
                                Input {
                                    r#type: "email",
                                    placeholder: "you@example.com",
                                    value: "{email}",
                                    oninput: move |e: FormEvent| email.set(e.value()),
                                }
                            }
                            FormGroup { label: "Password",
                                Input {
                                    r#type: "password",
                                    placeholder: "Enter password",
                                    value: "{password}",
                                    oninput: move |e: FormEvent| password.set(e.value()),
                                }
                                FormText { "Must be 8-20 characters long." }
                            }
                            Checkbox {
                                checked: *accept.read(),
                                label: "Remember me".to_string(),
                                onchange: move |_| {
                                    let current = *accept.read();
                                    accept.set(!current);
                                },
                            }
                            div { class: "mt-3",
                                Button { color: Color::Primary, r#type: "submit", "Sign In" }
                            }
                        },
                    }
                }
                Col { lg: ColumnSize::Span(6),
                    Card {
                        header: rsx! { "Other Controls" },
                        body: rsx! {
                            FormGroup { label: "Select",
                                Select {
                                    value: "{selected}",
                                    onchange: move |e: FormEvent| selected.set(e.value()),
                                    option { value: "", "Choose..." }
                                    option { value: "1", "Option 1" }
                                    option { value: "2", "Option 2" }
                                    option { value: "3", "Option 3" }
                                }
                            }
                            FormGroup { label: "Textarea",
                                Textarea {
                                    rows: 3,
                                    placeholder: "Write something...",
                                    value: "{message}",
                                    oninput: move |e: FormEvent| message.set(e.value()),
                                }
                            }
                            div { class: "mb-3",
                                label { class: "form-label d-block", "Radio Group" }
                                Radio { name: "color".to_string(), label: "Red".to_string(), checked: true }
                                Radio { name: "color".to_string(), label: "Green".to_string() }
                                Radio { name: "color".to_string(), label: "Blue".to_string() }
                            }
                        },
                    }
                }
            }

            // Switch
            h3 { class: "mb-3 mt-4", "Switch" }
            Switch {
                checked: *switch_on.read(),
                label: "Enable notifications".to_string(),
                onchange: move |_| {
                    let current = *switch_on.read();
                    switch_on.set(!current);
                },
            }
            p { class: "text-muted", "Switch is: ", if *switch_on.read() { "ON" } else { "OFF" } }

            // Range
            h3 { class: "mb-3 mt-3", "Range" }
            FormGroup { label: "Volume",
                Range {
                    value: "{range_val}",
                    min: "0".to_string(),
                    max: "100".to_string(),
                    oninput: move |e: FormEvent| range_val.set(e.value()),
                }
            }
            p { class: "text-muted", "Value: {range_val}" }

            // Floating Labels
            h3 { class: "mb-3 mt-3", "Floating Labels" }
            Row { class: "g-3 mb-3",
                Col { md: ColumnSize::Span(6),
                    FloatingLabel { label: "Email address".to_string(),
                        Input { r#type: "email", placeholder: "name@example.com" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FloatingLabel { label: "Password".to_string(),
                        Input { r#type: "password", placeholder: "Password" }
                    }
                }
            }

            // Validation
            h3 { class: "mb-3 mt-3", "Validation Feedback" }
            Row { class: "g-3 mb-3",
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Valid input",
                        Input { value: "correct value", class: "is-valid".to_string() }
                        FormFeedback { valid: true, "Looks good!" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    FormGroup { label: "Invalid input",
                        Input { value: "", class: "is-invalid".to_string(), placeholder: "Required field" }
                        FormFeedback { "Please provide a value." }
                    }
                }
            }

            // Input sizes
            h3 { class: "mb-3 mt-3", "Input Sizes" }
            FormGroup { label: "Small Input",
                Input { size: Size::Sm, placeholder: "Small" }
            }
            FormGroup { label: "Default Input",
                Input { placeholder: "Default" }
            }
            FormGroup { label: "Large Input",
                Input { size: Size::Lg, placeholder: "Large" }
            }

            // Input Group
            h3 { class: "mb-3 mt-3", "Input Group" }
            InputGroup { class: "mb-2",
                InputGroupText { "@" }
                Input { placeholder: "Username".to_string() }
            }
            InputGroup { class: "mb-2",
                Input { placeholder: "Search...".to_string() }
                InputGroupText {
                    Button { color: Color::Primary,
                        Icon { name: "search" }
                    }
                }
            }
            InputGroup { class: "mb-2", size: Size::Sm,
                InputGroupText { "$" }
                Input { r#type: "number".to_string(), placeholder: "Amount".to_string() }
                InputGroupText { ".00" }
            }
        }
    }
}

// ── Data ────────────────────────────────────────────────────────────────────

#[component]
fn DataSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Table
            h3 { class: "mb-3", "Table" }
            Card { class: "mb-4",
                body: rsx! {
                    Table { striped: true, hover: true, responsive: true,
                        thead {
                            tr {
                                th { "#" }
                                th { "Service" }
                                th { "Status" }
                                th { "Uptime" }
                            }
                        }
                        tbody {
                            tr {
                                td { "1" }
                                td { "API Gateway" }
                                td { Badge { color: Color::Success, "Running" } }
                                td { "99.9%" }
                            }
                            tr {
                                td { "2" }
                                td { "Database" }
                                td { Badge { color: Color::Success, "Running" } }
                                td { "99.7%" }
                            }
                            tr {
                                td { "3" }
                                td { "Cache" }
                                td { Badge { color: Color::Warning, "Degraded" } }
                                td { "98.2%" }
                            }
                            tr {
                                td { "4" }
                                td { "Worker" }
                                td { Badge { color: Color::Danger, "Down" } }
                                td { "—" }
                            }
                        }
                    }
                },
            }

            // Table with caption and striped columns
            h4 { class: "mb-3", "Table with Caption & Striped Columns" }
            Card { class: "mb-4",
                body: rsx! {
                    Table { striped_columns: true, bordered: true, caption: "List of users".to_string(), caption_top: true,
                        thead {
                            tr { th { "Name" } th { "Role" } th { "Status" } }
                        }
                        tbody {
                            tr { td { "Alice" } td { "Admin" } td { "Active" } }
                            tr { td { "Bob" } td { "Editor" } td { "Active" } }
                            tr { td { "Carol" } td { "Viewer" } td { "Inactive" } }
                        }
                    }
                },
            }

            // List Group
            h3 { class: "mb-3", "List Group" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    ListGroup {
                        ListGroupItem { active: true, "Active item" }
                        ListGroupItem { "Second item" }
                        ListGroupItem { "Third item" }
                        ListGroupItem { disabled: true, "Disabled item" }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    ListGroup { flush: true,
                        ListGroupItem { color: Color::Primary, "Primary" }
                        ListGroupItem { color: Color::Success, "Success" }
                        ListGroupItem { color: Color::Danger, "Danger" }
                        ListGroupItem { color: Color::Warning, "Warning" }
                    }
                }
            }

            // Progress & Spinners
            h3 { class: "mb-3", "Progress Bars" }
            div { class: "mb-4",
                Progress { class: "mb-2",
                    ProgressBar { value: 25.0, show_label: true }
                }
                Progress { class: "mb-2",
                    ProgressBar { value: 50.0, color: Color::Success, show_label: true }
                }
                Progress { class: "mb-2",
                    ProgressBar { value: 75.0, color: Color::Warning, striped: true }
                }
                Progress { class: "mb-2",
                    ProgressBar { value: 100.0, color: Color::Danger, striped: true, animated: true }
                }
                // Stacked
                Progress { class: "mb-2",
                    ProgressBar { value: 30.0, color: Color::Primary }
                    ProgressBar { value: 20.0, color: Color::Success }
                    ProgressBar { value: 15.0, color: Color::Warning }
                }
            }

            h3 { class: "mb-3", "Spinners" }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Spinner { "Loading..." }
                Spinner { color: Color::Primary, "Loading..." }
                Spinner { color: Color::Success, "Loading..." }
                Spinner { style: SpinnerStyle::Grow, color: Color::Danger, "Loading..." }
                Spinner { size: Size::Sm, color: Color::Info, "Loading..." }
            }

            // Pagination
            h3 { class: "mb-3", "Pagination" }
            div { class: "mb-4",
                Pagination { current: use_signal(|| 3usize), total: 20 }
                Pagination { current: use_signal(|| 3usize), total: 20, size: Size::Sm }
            }
        }
    }
}

// ── Interactive ─────────────────────────────────────────────────────────────

#[component]
fn InteractiveSection() -> Element {
    let mut show_modal = use_signal(|| false);
    let mut show_modal_fs = use_signal(|| false);
    let dropdown_open = use_signal(|| false);
    let split_dropdown_open = use_signal(|| false);
    let mut collapse_expanded = use_signal(|| false);
    let accordion_open = use_signal(|| Some(0usize));
    let mut toast_show = use_signal(|| false);
    let mut toast_headerless_show = use_signal(|| false);
    let mut offcanvas_show = use_signal(|| false);

    rsx! {
        div { class: "mt-3",
            // Modal
            h3 { class: "mb-3", "Modal" }
            div { class: "d-flex flex-wrap gap-2 mb-4",
                Button { color: Color::Primary, onclick: move |_| show_modal.set(true),
                    Icon { name: "box-arrow-up-right", class: "me-1" }
                    "Open Modal"
                }
                Button { color: Color::Secondary, onclick: move |_| show_modal_fs.set(true),
                    Icon { name: "arrows-fullscreen", class: "me-1" }
                    "Fullscreen Modal"
                }
                Modal {
                    show: show_modal,
                    title: "Example Modal".to_string(),
                    centered: true,
                    body: rsx! {
                        p { "This modal is controlled entirely by a Dioxus signal." }
                        p { "No JavaScript involved — click the backdrop or close button to dismiss." }
                    },
                    footer: rsx! {
                        Button { color: Color::Secondary, onclick: move |_| show_modal.set(false), "Close" }
                        Button { color: Color::Primary, "Save changes" }
                    },
                }
                Modal {
                    show: show_modal_fs,
                    title: "Fullscreen Modal".to_string(),
                    fullscreen: ModalFullscreen::Always,
                    body: rsx! {
                        p { "This modal takes up the entire screen." }
                        p { "Useful for complex forms or content that needs more space." }
                    },
                    footer: rsx! {
                        Button { color: Color::Secondary, onclick: move |_| show_modal_fs.set(false), "Close" }
                    },
                }
            }

            // Dropdown
            h3 { class: "mb-3", "Dropdown" }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Dropdown {
                    open: dropdown_open,
                    toggle: rsx! { "Dropdown Menu" },
                    menu: rsx! {
                        DropdownHeader { "Actions" }
                        DropdownItem { Icon { name: "pencil", class: "me-2" } "Edit" }
                        DropdownItem { Icon { name: "files", class: "me-2" } "Duplicate" }
                        DropdownDivider {}
                        DropdownItem { Icon { name: "trash", class: "me-2" } "Delete" }
                    },
                }
                // Split dropdown
                Dropdown {
                    open: split_dropdown_open,
                    split: true,
                    color: Some(Color::Success),
                    toggle: rsx! { "Split Action" },
                    menu: rsx! {
                        DropdownItem { "Action 1" }
                        DropdownItem { "Action 2" }
                        DropdownDivider {}
                        DropdownItem { "Separated action" }
                    },
                }
            }

            // Collapse
            h3 { class: "mb-3", "Collapse" }
            div { class: "mb-4",
                Button {
                    color: Color::Primary,
                    onclick: move |_| {
                        let current = *collapse_expanded.read();
                        collapse_expanded.set(!current);
                    },
                    if *collapse_expanded.read() { "Hide Content" } else { "Show Content" }
                }
                Collapse { expanded: collapse_expanded, class: "mt-2",
                    Card {
                        body: rsx! {
                            "This content is shown/hidden using a Dioxus signal driving the Bootstrap collapse classes. No JavaScript transitions needed."
                        },
                    }
                }
            }

            // Accordion
            h3 { class: "mb-3", "Accordion" }
            Accordion { open: accordion_open, class: "mb-4",
                AccordionItem { index: 0, title: "Accordion Item #1".to_string(), open: accordion_open,
                    "This is the first item's accordion body. It is shown by default."
                }
                AccordionItem { index: 1, title: "Accordion Item #2".to_string(), open: accordion_open,
                    "This is the second item's accordion body. Click the header to toggle."
                }
                AccordionItem { index: 2, title: "Accordion Item #3".to_string(), open: accordion_open,
                    "This is the third item's accordion body. Only one item is open at a time."
                }
            }

            // Tabs with different styles
            h3 { class: "mb-3", "Tabs" }
            h5 { "Standard Tabs" }
            TabList {
                active: use_signal(|| 0usize),
                tabs: vec![
                    TabDef { label: "Home".into(), icon: Some("house".into()), content: rsx! { p { class: "mt-2", "Home tab content." } } },
                    TabDef { label: "Profile".into(), icon: Some("person".into()), content: rsx! { p { class: "mt-2", "Profile tab content." } } },
                    TabDef { label: "Messages".into(), icon: None, content: rsx! { p { class: "mt-2", "Messages tab content." } } },
                ],
            }
            h5 { class: "mt-3", "Pills (fill)" }
            TabList {
                active: use_signal(|| 0usize),
                pills: true,
                tabs: vec![
                    TabDef { label: "Tab A".into(), icon: None, content: rsx! { p { class: "mt-2", "Tab A content." } } },
                    TabDef { label: "Tab B".into(), icon: None, content: rsx! { p { class: "mt-2", "Tab B content." } } },
                    TabDef { label: "Tab C".into(), icon: None, content: rsx! { p { class: "mt-2", "Tab C content." } } },
                ],
                class: "nav-fill".to_string(),
            }

            // Toast
            h3 { class: "mb-3 mt-4", "Toast" }
            div { class: "mb-4",
                div { class: "d-flex flex-wrap gap-2",
                    Button { color: Color::Success, onclick: move |_| toast_show.set(true),
                        Icon { name: "bell", class: "me-1" }
                        "Show Toast"
                    }
                    Button { color: Color::Primary, onclick: move |_| toast_headerless_show.set(true),
                        Icon { name: "chat-square-text", class: "me-1" }
                        "Headerless Toast"
                    }
                }
                ToastContainer { position: ToastPosition::TopEnd,
                    Toast { show: toast_show, title: "Notification".to_string(), subtitle: "just now".to_string(),
                        on_dismiss: move |_| { /* handle dismiss */ },
                        "This toast is controlled by a Dioxus signal. No JavaScript!"
                    }
                    Toast { show: toast_headerless_show, show_close: true, color: Color::Primary,
                        on_dismiss: move |_| { /* handle dismiss */ },
                        "A headerless toast with the d-flex close button pattern."
                    }
                }
            }

            // Offcanvas
            h3 { class: "mb-3", "Offcanvas" }
            div { class: "mb-4",
                Button { color: Color::Info, onclick: move |_| offcanvas_show.set(true),
                    Icon { name: "layout-sidebar", class: "me-1" }
                    "Open Offcanvas"
                }
                Offcanvas { show: offcanvas_show, title: "Sidebar Menu".to_string(),
                    p { "This sidebar slides in from the left. Powered by Dioxus signals." }
                    ListGroup { flush: true,
                        ListGroupItem { Icon { name: "house", class: "me-2" } "Home" }
                        ListGroupItem { Icon { name: "speedometer2", class: "me-2" } "Dashboard" }
                        ListGroupItem { Icon { name: "gear", class: "me-2" } "Settings" }
                    }
                }
            }
        }
    }
}

// ── Overlays (Tooltips & Popovers) ─────────────────────────────────────────

#[component]
fn OverlaysSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Tooltips
            h3 { class: "mb-3", "Tooltips" }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Tooltip { text: "Tooltip on top".to_string(), placement: TooltipPlacement::Top,
                    Button { id: "tooltip-hover-trigger", color: Color::Primary, "Hover" }
                }
                Tooltip {
                    text: "Tooltip on focus".to_string(),
                    placement: TooltipPlacement::Bottom,
                    trigger: TooltipTriggers::FOCUS,
                    Button { id: "tooltip-focus-trigger", color: Color::Secondary, "Focus" }
                }
                Tooltip {
                    text: "Tooltip on click".to_string(),
                    placement: TooltipPlacement::End,
                    trigger: TooltipTriggers::CLICK,
                    Button { id: "tooltip-click-trigger", color: Color::Info, "Click" }
                }
                Tooltip {
                    text: "Fallback below when top overflows".to_string(),
                    placement: TooltipPlacement::Top,
                    fallback_placements: vec![TooltipPlacement::Bottom],
                    Button { id: "tooltip-edge-trigger", color: Color::Warning, "Viewport fallback" }
                }
                Tooltip { text: "Disabled action unavailable".to_string(), placement: TooltipPlacement::Top,
                    TooltipDisabledTrigger {
                        Button { id: "tooltip-disabled-trigger", color: Color::Danger, disabled: true, "Disabled" }
                    }
                }
            }

            // Popovers
            h3 { class: "mb-3", "Popovers" }
            p { class: "text-muted mb-3", "Typed click, focus, viewport fallback, and disabled-trigger popovers." }
            div { class: "d-flex flex-wrap gap-3 mb-4",
                Popover {
                    title: "Click Popover".to_string(),
                    body: rsx! { "Default click trigger with Bootstrap's right/end placement." },
                    Button { id: "popover-click-trigger", color: Color::Primary, "Click" }
                }
                Popover {
                    title: "Focus Dismiss".to_string(),
                    body: rsx! { "Move focus away to dismiss this popover." },
                    placement: PopoverPlacement::Bottom,
                    trigger: PopoverTriggers::FOCUS,
                    Button { id: "popover-focus-trigger", color: Color::Secondary, "Focus" }
                }
                Popover {
                    title: "Outside Dismiss".to_string(),
                    body: rsx! { "Click outside the trigger or popover to close." },
                    placement: PopoverPlacement::Top,
                    Button { id: "popover-outside-trigger", color: Color::Success, "Outside" }
                }
                div { id: "popover-edge-container", style: "display: inline-block;",
                    Popover {
                        title: "Fallback Popover".to_string(),
                        body: rsx! { "Falls back below when top would overflow." },
                        placement: PopoverPlacement::Top,
                        fallback_placements: vec![PopoverPlacement::Bottom],
                        Button { id: "popover-edge-trigger", color: Color::Warning, "Viewport fallback" }
                    }
                }
                Popover {
                    title: "".to_string(),
                    body: rsx! {},
                    Button { id: "popover-empty-trigger", color: Color::Light, "Empty content" }
                }
                Popover {
                    title: "Disabled Popover".to_string(),
                    body: rsx! { "Disabled controls use a focusable wrapper." },
                    trigger: PopoverTriggers::HOVER_FOCUS,
                    PopoverDisabledTrigger {
                        Button { id: "popover-disabled-trigger", color: Color::Danger, disabled: true, "Disabled" }
                    }
                }
            }
        }
    }
}

fn showcase_svg(width: u32, height: u32, fill: &str, label: &str) -> String {
    let fill = fill.replace('#', "%23");
    let label = label.replace(' ', "%20");
    format!(
        "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}' viewBox='0 0 {width} {height}'%3E%3Crect width='{width}' height='{height}' fill='{fill}'/%3E%3Ctext x='50%25' y='52%25' text-anchor='middle' font-family='Arial,sans-serif' font-size='34' font-weight='700' fill='white'%3E{label}%3C/text%3E%3C/svg%3E"
    )
}

// ── Media (Carousel, Figure, Ratio) ────────────────────────────────────────

#[component]
fn MediaSection() -> Element {
    let carousel_active = use_signal(|| 0usize);
    let slide_one = showcase_svg(600, 300, "#0d6efd", "First Slide");
    let slide_two = showcase_svg(600, 300, "#198754", "Second Slide");
    let slide_three = showcase_svg(600, 300, "#6f42c1", "Third Slide");
    let figure_image = showcase_svg(400, 250, "#0dcaf0", "Figure");

    rsx! {
        div { class: "mt-3",
            // Carousel
            h3 { class: "mb-3", "Carousel" }
            div { class: "mb-4", style: "max-width: 600px;",
                Carousel {
                    active: carousel_active,
                    ride: true,
                    interval: 4000,
                slides: vec![
                    CarouselSlide {
                        src: slide_one,
                            alt: "First slide".into(),
                            caption_title: Some("First Slide".into()),
                            caption_text: Some("This is the first slide caption.".into()),
                        },
                    CarouselSlide {
                        src: slide_two,
                            alt: "Second slide".into(),
                            caption_title: Some("Second Slide".into()),
                            caption_text: Some("Another slide with a caption.".into()),
                        },
                    CarouselSlide {
                        src: slide_three,
                            alt: "Third slide".into(),
                            caption_title: None,
                            caption_text: None,
                        },
                    ],
                }
                p { class: "text-muted mt-2", "Active slide: {carousel_active}" }
            }

            // Figure
            h3 { class: "mb-3", "Figure" }
            Figure {
                src: figure_image,
                alt: "Sample image".to_string(),
                caption: "A caption for this figure image.".to_string(),
                rounded: true,
            }

            // Ratio (responsive embeds)
            h3 { class: "mb-3 mt-4", "Ratio (Responsive Embeds)" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    h5 { "16x9" }
                    Ratio { aspect: "16x9".to_string(),
                        div { class: "bg-primary-subtle d-flex align-items-center justify-content-center h-100 rounded",
                            "16x9 content"
                        }
                    }
                }
                Col { md: ColumnSize::Span(3),
                    h5 { "4x3" }
                    Ratio { aspect: "4x3".to_string(),
                        div { class: "bg-success-subtle d-flex align-items-center justify-content-center h-100 rounded",
                            "4x3"
                        }
                    }
                }
                Col { md: ColumnSize::Span(3),
                    h5 { "1x1" }
                    Ratio { aspect: "1x1".to_string(),
                        div { class: "bg-warning-subtle d-flex align-items-center justify-content-center h-100 rounded",
                            "1x1"
                        }
                    }
                }
            }
        }
    }
}

// ── Navigation ──────────────────────────────────────────────────────────────

#[component]
fn NavigationSection() -> Element {
    let body_scrollspy_active = use_signal(String::new);
    let custom_scrollspy_active = use_signal(String::new);
    let mut show_dynamic_scrollspy_section = use_signal(|| false);
    let custom_refresh_key = if *show_dynamic_scrollspy_section.read() {
        1
    } else {
        0
    };

    rsx! {
        div { class: "mt-3",
            // Nav — Pills
            h3 { class: "mb-3", "Nav — Pills" }
            Nav { pills: true, class: "mb-4",
                NavItem { NavLink { active: true, "Active" } }
                NavItem { NavLink { "Link" } }
                NavItem { NavLink { "Another Link" } }
                NavItem { NavLink { disabled: true, "Disabled" } }
            }

            // Nav — Tabs
            h3 { class: "mb-3", "Nav — Tabs" }
            Nav { tabs: true, class: "mb-4",
                NavItem { NavLink { active: true, "Home" } }
                NavItem { NavLink { "Profile" } }
                NavItem { NavLink { "Messages" } }
            }

            // Nav — Underline
            h3 { class: "mb-3", "Nav — Underline" }
            Nav { underline: true, class: "mb-4",
                NavItem { NavLink { active: true, "Active" } }
                NavItem { NavLink { "Link" } }
                NavItem { NavLink { "Another" } }
            }

            // Nav — Fill
            h3 { class: "mb-3", "Nav — Fill" }
            Nav { pills: true, fill: true, class: "mb-4",
                NavItem { NavLink { active: true, "Home" } }
                NavItem { NavLink { "Much longer nav link" } }
                NavItem { NavLink { "Short" } }
            }

            // Nav — Justified
            h3 { class: "mb-3", "Nav — Justified" }
            Nav { pills: true, justified: true, class: "mb-4",
                NavItem { NavLink { active: true, "Equal" } }
                NavItem { NavLink { "Width" } }
                NavItem { NavLink { "Items" } }
            }

            // Nav — Vertical
            h3 { class: "mb-3", "Nav — Vertical" }
            Row { class: "mb-4",
                Col { md: ColumnSize::Span(3),
                    Nav { pills: true, vertical: true,
                        NavItem { NavLink { active: true, "Home" } }
                        NavItem { NavLink { "Profile" } }
                        NavItem { NavLink { "Messages" } }
                        NavItem { NavLink { disabled: true, "Disabled" } }
                    }
                }
                Col { md: ColumnSize::Span(9),
                    Card {
                        body: rsx! { "Content area next to vertical nav." },
                    }
                }
            }

            // Breadcrumb
            h3 { class: "mb-3", "Breadcrumb" }
            Breadcrumb {
                BreadcrumbItem { href: "#", "Home" }
                BreadcrumbItem { href: "#", "Library" }
                BreadcrumbItem { active: true, "Data" }
            }

            // Scrollspy
            h3 { class: "mb-3 mt-4", "Scrollspy" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(6),
                    Scrollspy {
                        target: "#scrollspy-body-nav",
                        root: "body",
                        active: body_scrollspy_active,
                        offset: 96,
                        root_margin: "0px 0px -65%".to_string(),
                    }
                    Nav {
                        id: "scrollspy-body-nav",
                        pills: true,
                        vertical: true,
                        class: "gap-1 mb-2",
                        NavItem { NavLink { href: "#scrollspy-body-alpha", "Alpha" } }
                        NavItem { NavLink { href: "#scrollspy-body-beta", "Beta" } }
                        NavItem { NavLink { href: "#scrollspy-body-gamma", "Gamma" } }
                    }
                    div { class: "small text-muted mb-2",
                        "Body active: "
                        span { id: "scrollspy-body-active", "{body_scrollspy_active}" }
                    }
                    div { id: "scrollspy-body-content", class: "border rounded p-3",
                        section { id: "scrollspy-body-alpha", style: "min-height: 260px;",
                            h4 { "Alpha" }
                            p { "First body-scrolled section." }
                        }
                        section { id: "scrollspy-body-beta", style: "min-height: 260px;",
                            h4 { "Beta" }
                            p { "Second body-scrolled section." }
                        }
                        section { id: "scrollspy-body-gamma", style: "min-height: 260px;",
                            h4 { "Gamma" }
                            p { "Third body-scrolled section." }
                        }
                    }
                }
                Col { md: ColumnSize::Span(6),
                    Scrollspy {
                        target: "#scrollspy-custom-nav",
                        root: "#scrollspy-custom-root",
                        active: custom_scrollspy_active,
                        offset: 12,
                        root_margin: "0px 0px -60%".to_string(),
                        refresh_key: custom_refresh_key,
                    }
                    Nav {
                        id: "scrollspy-custom-nav",
                        pills: true,
                        class: "gap-1 mb-2",
                        NavItem { NavLink { href: "#scrollspy-custom-one", "One" } }
                        NavItem { NavLink { href: "#scrollspy-custom-two", "Two" } }
                        NavItem { NavLink { href: "#scrollspy-custom-three", "Three" } }
                        if *show_dynamic_scrollspy_section.read() {
                            NavItem { NavLink { href: "#scrollspy-custom-four", "Four" } }
                        }
                    }
                    div { class: "d-flex align-items-center gap-2 mb-2",
                        span { class: "small text-muted",
                            "Custom active: "
                            span { id: "scrollspy-custom-active", "{custom_scrollspy_active}" }
                        }
                        Button {
                            id: "scrollspy-add-section",
                            color: Color::Secondary,
                            size: Size::Sm,
                            onclick: move |_| show_dynamic_scrollspy_section.set(true),
                            "Add section"
                        }
                    }
                    div {
                        id: "scrollspy-custom-root",
                        class: "border rounded p-3",
                        tabindex: "0",
                        style: "height: 260px; overflow-y: auto; position: relative;",
                        section { id: "scrollspy-custom-one", style: "min-height: 230px;",
                            h4 { "One" }
                            p { "First custom-scroll section." }
                        }
                        section { id: "scrollspy-custom-two", style: "min-height: 230px;",
                            h4 { "Two" }
                            p { "Second custom-scroll section." }
                        }
                        section { id: "scrollspy-custom-three", style: "min-height: 230px;",
                            h4 { "Three" }
                            p { "Third custom-scroll section." }
                        }
                        if *show_dynamic_scrollspy_section.read() {
                            section { id: "scrollspy-custom-four", style: "min-height: 230px;",
                                h4 { "Four" }
                                p { "Dynamically added custom-scroll section." }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── More ────────────────────────────────────────────────────────────────────

#[component]
fn MoreSection() -> Element {
    rsx! {
        div { class: "mt-3",
            // Placeholders
            h3 { class: "mb-3", "Placeholders (Loading Skeletons)" }
            Row { class: "g-3 mb-4",
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            Placeholder { width: 7, glow: true }
                            Placeholder { width: 4, glow: true }
                            Placeholder { width: 6, glow: true }
                            Placeholder { width: 8, glow: true }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            Placeholder { width: 6, wave: true, color: Color::Primary }
                            Placeholder { width: 9, wave: true, color: Color::Primary }
                            Placeholder { width: 5, wave: true, color: Color::Primary }
                        },
                    }
                }
                Col { md: ColumnSize::Span(4),
                    Card {
                        body: rsx! {
                            h5 { class: "card-title", "Real Content" }
                            p { class: "card-text", "Compare this card with the loading placeholders on the left." }
                            Button { color: Color::Primary, size: Size::Sm, "Action" }
                        },
                    }
                }
            }
        }
    }
}
