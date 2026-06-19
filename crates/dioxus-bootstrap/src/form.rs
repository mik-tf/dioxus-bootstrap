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
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Input { r#type: "text", value: "hello", placeholder: "Enter text" }
///     Input { r#type: "email", size: Size::Sm, oninput: move |evt| { /* handle */ } }
///     Input { r#type: "password", disabled: true }
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
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
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
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
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
    let size_class = match props.size {
        Size::Md => String::new(),
        s => format!(" form-select-{s}"),
    };

    let full_class = if props.class.is_empty() {
        format!("form-select{size_class}")
    } else {
        format!("form-select{size_class} {}", props.class)
    };

    rsx! {
        select {
            class: "{full_class}",
            value: "{props.value}",
            disabled: props.disabled,
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
    /// Number of visible rows.
    #[props(default = 3)]
    pub rows: u32,
    /// Placeholder text.
    #[props(default)]
    pub placeholder: String,
    /// Disabled state.
    #[props(default)]
    pub disabled: bool,
    /// Readonly state.
    #[props(default)]
    pub readonly: bool,
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
pub fn Textarea(props: TextareaProps) -> Element {
    let full_class = if props.class.is_empty() {
        "form-control".to_string()
    } else {
        format!("form-control {}", props.class)
    };

    rsx! {
        textarea {
            class: "{full_class}",
            rows: "{props.rows}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            readonly: props.readonly,
            value: "{props.value}",
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
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
pub fn Checkbox(props: CheckboxProps) -> Element {
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
                r#type: "checkbox",
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
    let full_class = if props.class.is_empty() {
        "form-range".to_string()
    } else {
        format!("form-range {}", props.class)
    };

    rsx! {
        input {
            class: "{full_class}",
            r#type: "range",
            value: "{props.value}",
            min: "{props.min}",
            max: "{props.max}",
            step: if props.step.is_empty() { None } else { Some(props.step.clone()) },
            disabled: props.disabled,
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
