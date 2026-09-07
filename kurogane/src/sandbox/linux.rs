use crate::chromium_flags::ChromiumFlags;

// Nothing to disable: the sandbox stays on (runtime.rs, no_sandbox), and the
// host decides when a machine cannot have it. --disable-setuid-sandbox on the
// command line was one of the switches Chrome flags with its "stability and
// security will suffer" bar, on a window that did not use the helper anyway.
pub(crate) fn apply_sandbox_flags(_flags: &mut ChromiumFlags) {}
