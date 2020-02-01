use std::cell::RefCell;

use crate::{abort_now, check_correctness, Diagnostic};

pub fn abort_if_dirty() {
    check_correctness();
    ERR_STORAGE.with(|storage| {
        if !storage.borrow().is_empty() {
            abort_now()
        }
    });
}

/// Clear the global error storage, returning the errors contained.
pub(crate) fn cleanup() -> Vec<Diagnostic> {
    ERR_STORAGE.with(|storage| storage.replace(Vec::new()))
}

pub(crate) fn emit_diagnostic(diag: Diagnostic) {
    ERR_STORAGE.with(|storage| storage.borrow_mut().push(diag));
}

thread_local! {
    static ERR_STORAGE: RefCell<Vec<Diagnostic>> = RefCell::new(Vec::new());
}
