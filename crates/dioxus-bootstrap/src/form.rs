use dioxus::prelude::*;

use crate::types::Size;

/// Bootstrap FormGroup — label + control wrapper.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="mb-3">
///   <label class="form-label">Email</label>
///   <input type="email" class="form-control" placeholder="you@example.com">
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     FormGroup { label: "Email",
///         Input { r#type: "email", placeholder: "you@example.com" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormGroupProps {
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Additional CSS classes for the wrapper div.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (form control).
    pub children: Element,
}

#[component]
pub fn FormGroup(props: FormGroupProps) -> Element {
    let full_class = if props.class.is_empty() {
        "mb-3".to_string()
    } else {
        format!("mb-3 {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            if !props.label.is_empty() {
                label { class: "form-label", "{props.label}" }
            }
            {props.children}
        }
    }
}

/// Bootstrap Input component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<input class="form-control" type="text">` | `Input { r#type: "text" }` |
/// | `<input class="form-control form-control-sm" type="email">` | `Input { r#type: "email", size: Size::Sm }` |
/// | `<input class="form-control" disabled>` | `Input { disabled: true }` |
/// | `<input class="form-control" list="opts">` | `Input { list: "opts" }` |
///
/// Bind a `<datalist>` for autocomplete with `list`, and observe focus with
/// `onfocus` / `onblur`:
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { r#type: "text", value: "hello", placeholder: "Enter text" }
///     Input { r#type: "email", size: Size::Sm, oninput: move |evt| { /* handle */ } }
///     Input { r#type: "password", disabled: true }
///     Input {
///         list: "icon-options",
///         onfocus: move |_| { /* open suggestions */ },
///         onblur: move |_| { /* close suggestions */ },
///     }
///     datalist { id: "icon-options",
///         option { value: "star" }
///         option { value: "heart" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
    /// Input type (text, email, password, number, etc.).
    #[props(default = "text".to_string())]
    pub r#type: String,
    /// Current value.
    #[props(default)]
    pub value: String,
    /// When `true`, the `value` attribute is omitted so the field is
    /// *uncontrolled*: the DOM keeps whatever value the user or an external
    /// script writes, instead of Dioxus forcing it back to `value` on every
    /// render. Use for a field another script streams into (e.g. a live
    /// transcript box).
    #[props(default)]
    pub uncontrolled: bool,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Minimum value for numeric/date inputs.
    #[props(default)]
    pub min: Option<String>,
    /// Maximum value for numeric/date inputs.
    #[props(default)]
    pub max: Option<String>,
    /// Browser autocomplete hint.
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Datalist id to bind for autocomplete (rendered as the input `list`
    /// attribute). `list` is not a `GlobalAttributes` attribute, so it needs
    /// its own typed prop rather than riding through `..attributes`.
    #[props(default)]
    pub list: Option<String>,
    /// Input size.
    #[props(default)]
    pub size: Size,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Readonly state.
    #[props(default)]
    pub readonly: bool,
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Key down event handler.
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Key up event handler.
    #[props(default)]
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-control-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("form-control{size_class}")
    } else {
        format!("form-control{size_class} {}", props.class)
    };

    rsx! {
        input {
            class: "{full_class}",
            r#type: "{props.r#type}",
            value: if props.uncontrolled { None } else { Some(props.value.clone()) },
            placeholder: "{props.placeholder}",
            min: props.min.clone(),
            max: props.max.clone(),
            autocomplete: props.autocomplete.clone(),
            list: props.list.clone(),
            disabled: props.disabled,
            readonly: props.readonly,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(handler) = &props.onfocus {
                    handler.call(evt);
                }
            },
            onblur: move |evt| {
                if let Some(handler) = &props.onblur {
                    handler.call(evt);
                }
            },
            onkeydown: move |evt| {
                if let Some(handler) = &props.onkeydown {
                    handler.call(evt);
                }
            },
            onkeyup: move |evt| {
                if let Some(handler) = &props.onkeyup {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
    }
}

/// Bootstrap Select (dropdown) component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <select class="form-select">
///   <option value="opt1">Option 1</option>
///   <option value="opt2" selected>Option 2</option>
/// </select>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Select { value: "opt2", onchange: move |evt| { /* handle */ },
///         option { value: "opt1", "Option 1" }
///         option { value: "opt2", "Option 2" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct SelectProps {
    /// Current selected value.
    #[props(default)]
    pub value: String,
    /// Select size.
    #[props(default)]
    pub size: Size,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements (option elements).
    pub children: Element,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    use wasm_bindgen::JsCast;

    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-select-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("form-select{size_class}")
    } else {
        format!("form-select{size_class} {}", props.class)
    };

    // A `<select>`'s selection is controlled by its `.value` property (or an
    // `<option selected>`), NOT by a `value` content attribute — browsers ignore
    // the latter on a select, so Dioxus's declarative `value` silently does
    // nothing and the element shows its first option. Hold the mounted element
    // and set `.value` imperatively on mount and whenever `value` changes, so the
    // control reflects the value it is given.
    let mut select_el = use_signal(|| None as Option<web_sys::HtmlSelectElement>);
    let value = props.value.clone();
    use_effect(use_reactive!(|value| {
        if let Some(el) = select_el.peek().clone() {
            el.set_value(&value);
        }
    }));

    let mount_value = props.value.clone();
    rsx! {
        select {
            class: "{full_class}",
            disabled: props.disabled,
            onmounted: move |evt: MountedEvent| {
                if let Some(el) = evt
                    .downcast::<web_sys::Element>()
                    .and_then(|e| e.clone().dyn_into::<web_sys::HtmlSelectElement>().ok())
                {
                    el.set_value(&mount_value);
                    select_el.set(Some(el));
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

/// Bootstrap Textarea component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<textarea class="form-control" rows="5">` | `Textarea { rows: 5 }` |
/// | `<textarea class="form-control form-control-sm">` | `Textarea { size: Size::Sm }` |
/// | `<textarea class="form-control" placeholder="..." disabled>` | `Textarea { placeholder: "...", disabled: true }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Textarea { rows: 5, placeholder: "Enter description..." }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// Current value.
    #[props(default)]
    pub value: String,
    /// When `true`, the `value` attribute is omitted so the field is
    /// *uncontrolled*: the DOM keeps whatever value the user or an external
    /// script writes, instead of Dioxus forcing it back to `value` on every
    /// render. Use for a field another script streams into (e.g. a live
    /// transcript box).
    #[props(default)]
    pub uncontrolled: bool,
    /// Number of visible rows.
    #[props(default = 3)]
    pub rows: u32,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Textarea size.
    #[props(default)]
    pub size: Size,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Readonly state.
    #[props(default)]
    pub readonly: bool,
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Focus event handler.
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Blur event handler.
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Key down event handler.
    #[props(default)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    /// Key up event handler.
    #[props(default)]
    pub onkeyup: Option<EventHandler<KeyboardEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-control-{s}"),
    };
    let full_class = if props.class.is_empty() {
        format!("form-control{size_class}")
    } else {
        format!("form-control{size_class} {}", props.class)
    };

    rsx! {
        textarea {
            class: "{full_class}",
            rows: "{props.rows}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            value: if props.uncontrolled { None } else { Some(props.value.clone()) },
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            onfocus: move |evt| {
                if let Some(handler) = &props.onfocus {
                    handler.call(evt);
                }
            },
            onblur: move |evt| {
                if let Some(handler) = &props.onblur {
                    handler.call(evt);
                }
            },
            onkeydown: move |evt| {
                if let Some(handler) = &props.onkeydown {
                    handler.call(evt);
                }
            },
            onkeyup: move |evt| {
                if let Some(handler) = &props.onkeyup {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
    }
}

/// Bootstrap Checkbox component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="form-check">
///   <input class="form-check-input" type="checkbox" checked>
///   <label class="form-check-label">Accept terms</label>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Checkbox { checked: true, label: "Accept terms",
///         onchange: move |evt| { /* handle */ },
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CheckboxProps {
    /// Whether the checkbox is checked.
    #[props(default)]
    pub checked: bool,
    /// Optional id applied to the checkbox input.
    #[props(default)]
    pub input_id: Option<String>,
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Click event handler for the checkbox input.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check".to_string()
    } else {
        format!("form-check {}", props.class)
    };
    let label_for = props.input_id.clone().unwrap_or_default();

    rsx! {
            div { class: "{full_class}",
                ..props.attributes,
    input {
    class: "form-check-input",
    r#type: "checkbox",
                    id: props.input_id.unwrap_or_default(),
                    checked: props.checked,
                    disabled: props.disabled,
                    onclick: move |evt| {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    },
                    onchange: move |evt| {
                        if let Some(handler) = &props.onchange {
                            handler.call(evt);
                        }
                    },
                }
    if !props.label.is_empty() {
    label { class: "form-check-label", r#for: "{label_for}", "{props.label}" }
    }
            }
        }
}

/// Bootstrap Switch (toggle) component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="form-check form-switch">
///   <input class="form-check-input" type="checkbox" role="switch" checked>
///   <label class="form-check-label">Enable notifications</label>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Switch { checked: true, label: "Enable notifications",
///         onchange: move |evt| { /* handle */ },
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct SwitchProps {
    /// Whether the switch is on.
    #[props(default)]
    pub checked: bool,
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check form-switch".to_string()
    } else {
        format!("form-check form-switch {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            input {
                class: "form-check-input",
                r#type: "checkbox",
                role: "switch",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
            }
            if !props.label.is_empty() {
                label { class: "form-check-label", "{props.label}" }
            }
        }
    }
}

/// Bootstrap Range (slider) input.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<input type="range" class="form-range" min="0" max="100">` | `Range { min: "0", max: "100" }` |
/// | `<input type="range" class="form-range" step="5" disabled>` | `Range { step: "5".into(), disabled: true }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Range { value: "50", min: "0", max: "100" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RangeProps {
    /// Current value.
    #[props(default)]
    pub value: String,
    /// Minimum value.
    #[props(default = "0".to_string())]
    pub min: String,
    /// Maximum value.
    #[props(default = "100".to_string())]
    pub max: String,
    /// Step increment.
    #[props(default)]
    pub step: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Input event handler.
    #[props(default)]
    pub oninput: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Range(props: RangeProps) -> Element {
    use wasm_bindgen::JsCast;

    let full_class = if props.class.is_empty() {
        "form-range".to_string()
    } else {
        format!("form-range {}", props.class)
    };

    // A range slider's thumb position is controlled by its `.value` DOM property,
    // NOT by a `value` content attribute (the attribute only seeds the default).
    // Dioxus's declarative `value` sets the attribute, so a server-reported value
    // that differs from the default leaves the thumb at the default — the same
    // property-vs-attribute gap the Select had. Hold the mounted element and set
    // `.value` imperatively on mount and whenever `value` changes.
    let mut range_el = use_signal(|| None as Option<web_sys::HtmlInputElement>);
    let value = props.value.clone();
    use_effect(use_reactive!(|value| {
        if let Some(el) = range_el.peek().clone() {
            el.set_value(&value);
        }
    }));

    let mount_value = props.value.clone();
    rsx! {
        input {
            class: "{full_class}",
            r#type: "range",
            min: "{props.min}",
            max: "{props.max}",
            step: if props.step.is_empty() { None } else { Some(props.step.clone()) },
            disabled: props.disabled,
            onmounted: move |evt: MountedEvent| {
                if let Some(el) = evt
                    .downcast::<web_sys::Element>()
                    .and_then(|e| e.clone().dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    el.set_value(&mount_value);
                    range_el.set(Some(el));
                }
            },
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            ..props.attributes,
        }
    }
}

/// Bootstrap Floating Label wrapper.
///
/// Wraps an Input or Textarea with a floating label that moves
/// above the control when focused or filled.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="form-floating"><input class="form-control" placeholder="..."><label>Email</label></div>` | `FloatingLabel { label: "Email", Input { placeholder: "..." } }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     FloatingLabel { label: "Email address",
///         Input { r#type: "email", placeholder: "name@example.com" }
///     }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FloatingLabelProps {
    /// Label text.
    pub label: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child element (Input or Textarea).
    pub children: Element,
}

#[component]
pub fn FloatingLabel(props: FloatingLabelProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-floating".to_string()
    } else {
        format!("form-floating {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            {props.children}
            label { "{props.label}" }
        }
    }
}

/// Bootstrap form validation feedback text.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="valid-feedback">Looks good!</div>` | `FormFeedback { valid: true, "Looks good!" }` |
/// | `<div class="invalid-feedback">Required.</div>` | `FormFeedback { "Required." }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { class: "is-valid".to_string(), value: "correct" }
///     FormFeedback { valid: true, "Looks good!" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormFeedbackProps {
    /// True for valid feedback, false for invalid.
    #[props(default)]
    pub valid: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Feedback text.
    pub children: Element,
}

#[component]
pub fn FormFeedback(props: FormFeedbackProps) -> Element {
    let base = if props.valid {
        "valid-feedback"
    } else {
        "invalid-feedback"
    };
    let full_class = if props.class.is_empty() {
        base.to_string()
    } else {
        format!("{base} {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap form text (help text below a control).
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<div class="form-text">Must be 8-20 characters.</div>` | `FormText { "Must be 8-20 characters." }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { r#type: "password" }
///     FormText { "Must be 8-20 characters long." }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FormTextProps {
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Help text content.
    pub children: Element,
}

#[component]
pub fn FormText(props: FormTextProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-text".to_string()
    } else {
        format!("form-text {}", props.class)
    };

    rsx! {
        div { class: "{full_class}", ..props.attributes, {props.children} }
    }
}

/// Bootstrap Radio button component.
///
/// # Bootstrap HTML → Dioxus
///
/// ```html
/// <!-- Bootstrap HTML -->
/// <div class="form-check">
///   <input class="form-check-input" type="radio" name="color" checked>
///   <label class="form-check-label">Red</label>
/// </div>
/// <div class="form-check">
///   <input class="form-check-input" type="radio" name="color">
///   <label class="form-check-label">Blue</label>
/// </div>
/// ```
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Radio { name: "color", label: "Red", checked: true }
///     Radio { name: "color", label: "Blue" }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RadioProps {
    /// Radio group name.
    pub name: String,
    /// Whether the radio is checked.
    #[props(default)]
    pub checked: bool,
    /// Label text.
    #[props(default)]
    pub label: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Change event handler.
    #[props(default)]
    pub onchange: Option<EventHandler<FormEvent>>,
    /// Additional CSS classes for the wrapper.
    #[props(default)]
    pub class: String,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Radio(props: RadioProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-check".to_string()
    } else {
        format!("form-check {}", props.class)
    };

    rsx! {
        div { class: "{full_class}",
            ..props.attributes,
            input {
                class: "form-check-input",
                r#type: "radio",
                name: "{props.name}",
                checked: props.checked,
                disabled: props.disabled,
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
            }
            if !props.label.is_empty() {
                label { class: "form-check-label", "{props.label}" }
            }
        }
    }
}
