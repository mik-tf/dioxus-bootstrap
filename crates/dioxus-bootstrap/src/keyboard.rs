//! Shared keyboard helpers for dismissible overlays.
//!
//! Bootstrap's Modal and Offcanvas close on the Escape key (their `keyboard:
//! true` option). With no Bootstrap JavaScript, the typed components reproduce
//! that by focusing the panel on mount and handling `onkeydown` on the panel;
//! this helper is the single place that decides which key dismisses, so Modal
//! and Offcanvas stay in agreement.

use dioxus::prelude::Key;

/// Whether a key press should dismiss a dismissible overlay (Modal, Offcanvas).
///
/// Escape dismisses; everything else is ignored — matching Bootstrap's
/// `keyboard: true` behaviour.
pub(crate) fn is_escape_key(key: &Key) -> bool {
    matches!(key, Key::Escape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_dismisses() {
        assert!(is_escape_key(&Key::Escape));
    }

    #[test]
    fn other_keys_do_not_dismiss() {
        assert!(!is_escape_key(&Key::Enter));
        assert!(!is_escape_key(&Key::ArrowLeft));
        assert!(!is_escape_key(&Key::Character("a".to_string())));
    }
}
