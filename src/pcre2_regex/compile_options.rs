#![allow(dead_code)]
/// Force pattern anchoring
pub const ANCHORED:u32 = pcre2_sys::PCRE2_ANCHORED;

/// Allow empty classes
pub const ALLOW_EMPTY_CLASS:u32 = pcre2_sys::PCRE2_ALLOW_EMPTY_CLASS;

/// Alternative handling of \u, \U, and \x
pub const ALT_BSUX:u32 = pcre2_sys::PCRE2_ALT_BSUX;

/// Alternative handling of ^ in multiline mode
pub const ALT_CIRCUMFLEX:u32 = pcre2_sys::PCRE2_ALT_CIRCUMFLEX;

/// Process backslashes in verb names
pub const ALT_VERBNAMES:u32 = pcre2_sys::PCRE2_ALT_VERBNAMES;

/// Compile automatic callouts
pub const AUTO_CALLOUT:u32 = pcre2_sys::PCRE2_AUTO_CALLOUT;

/// Do caseless matching
pub const CASELESS:u32 = pcre2_sys::PCRE2_CASELESS;

/// $ not to match newline at end
pub const DOLLAR_ENDONLY:u32 = pcre2_sys::PCRE2_DOLLAR_ENDONLY;

/// . matches anything including NL
pub const DOTALL:u32 = pcre2_sys::PCRE2_DOTALL;

/// Allow duplicate names for subpatterns
pub const DUPNAMES:u32 = pcre2_sys::PCRE2_DUPNAMES;

/// Pattern can match only at end of subject
pub const ENDANCHORED:u32 = pcre2_sys::PCRE2_ENDANCHORED;

/// Ignore white space and # comments
pub const EXTENDED:u32 = pcre2_sys::PCRE2_EXTENDED;

/// Force matching to be before newline
pub const FIRSTLINE:u32 = pcre2_sys::PCRE2_FIRSTLINE;

/// Pattern characters are all literal
pub const LITERAL:u32 = pcre2_sys::PCRE2_LITERAL;

/// Match unset backreferences
pub const MATCH_UNSET_BACKREF:u32 = pcre2_sys::PCRE2_MATCH_UNSET_BACKREF;

/// ^ and $ match newlines within data
pub const MULTILINE:u32 = pcre2_sys::PCRE2_MULTILINE;

/// Lock out the use of \C in patterns
pub const NEVER_BACKSLASH_C:u32 = pcre2_sys::PCRE2_NEVER_BACKSLASH_C;

/// Lock out PCRE2_UCP, e.g. via (*UCP)
pub const NEVER_UCP:u32 = pcre2_sys::PCRE2_NEVER_UCP;

/// Lock out PCRE2_UTF, e.g. via (*UTF)
pub const NEVER_UTF:u32 = pcre2_sys::PCRE2_NEVER_UTF;

/// Disable numbered capturing parentheses (named ones available)
pub const NO_AUTO_CAPTURE:u32 = pcre2_sys::PCRE2_NO_AUTO_CAPTURE;

/// Disable auto-possessification
pub const NO_AUTO_POSSESS:u32 = pcre2_sys::PCRE2_NO_AUTO_POSSESS;

/// Disable automatic anchoring for .*
pub const NO_DOTSTAR_ANCHOR:u32 = pcre2_sys::PCRE2_NO_DOTSTAR_ANCHOR;

/// Disable match-time start optimizations
pub const NO_START_OPTIMIZE:u32 = pcre2_sys::PCRE2_NO_START_OPTIMIZE;

/// Do not check the pattern for UTF validity (only relevant if PCRE2_UTF is set)
pub const NO_UTF_CHECK:u32 = pcre2_sys::PCRE2_NO_UTF_CHECK;

/// Use Unicode properties for \d, \w, etc.
pub const UCP:u32 = pcre2_sys::PCRE2_UCP;

/// Invert greediness of quantifiers
pub const UNGREEDY:u32 = pcre2_sys::PCRE2_UNGREEDY;

/// Enable offset limit for unanchored matching
pub const USE_OFFSET_LIMIT:u32 = pcre2_sys::PCRE2_USE_OFFSET_LIMIT;

/// Treat pattern and subjects as UTF strings
pub const UTF:u32 = pcre2_sys::PCRE2_UTF;