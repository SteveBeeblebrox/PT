#![allow(dead_code)]
/// Match only at the first position
pub const ANCHORED:u32 = pcre2_sys::PCRE2_ANCHORED;

/// Pattern can match only at end of subject
pub const ENDANCHORED:u32 = pcre2_sys::PCRE2_ENDANCHORED;

/// Subject is not the beginning of a line
pub const NOTBOL:u32 = pcre2_sys::PCRE2_NOTBOL;

/// Subject is not the end of a line
pub const NOTEOL:u32 = pcre2_sys::PCRE2_NOTEOL;

/// An empty string is not a valid match
pub const NOTEMPTY:u32 = pcre2_sys::PCRE2_NOTEMPTY;

/// An empty string at the start of the subject is not a valid match
pub const NOTEMPTY_ATSTART:u32 = pcre2_sys::PCRE2_NOTEMPTY_ATSTART;

/// Do not use JIT matching
pub const NO_JIT:u32 = pcre2_sys::PCRE2_NO_JIT;

/// Do not check the subject or replacement for UTF validity (only relevant if PCRE2_UTF was set at compile time)
pub const NO_UTF_CHECK:u32 = pcre2_sys::PCRE2_NO_UTF_CHECK;

/// Do extended replacement processing
pub const SUBSTITUTE_EXTENDED:u32 = pcre2_sys::PCRE2_SUBSTITUTE_EXTENDED;

/// Replace all occurrences in the subject
pub const SUBSTITUTE_GLOBAL:u32 = pcre2_sys::PCRE2_SUBSTITUTE_GLOBAL;

/// If overflow, compute needed length
pub const SUBSTITUTE_OVERFLOW_LENGTH:u32 = pcre2_sys::PCRE2_SUBSTITUTE_OVERFLOW_LENGTH;

/// Treat unknown group as unset
pub const SUBSTITUTE_UNKNOWN_UNSET:u32 = pcre2_sys::PCRE2_SUBSTITUTE_UNKNOWN_UNSET;

/// Simple unset insert = empty string
pub const SUBSTITUTE_UNSET_EMPTY:u32 = pcre2_sys::PCRE2_SUBSTITUTE_UNSET_EMPTY;