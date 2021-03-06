//! Hash Compiler lexer utilities for identifiers and other character sequences.

/// True if `c` is valid as a first character of an identifier.
// @@Future: Support unicode within idents?
pub(crate) fn is_id_start(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
}

/// True if `c` is valid as a non-first character of an identifier.
// @@Future: Support unicode within idents?
pub(crate) fn is_id_continue(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || ('0'..='9').contains(&c) || c == '_'
}
