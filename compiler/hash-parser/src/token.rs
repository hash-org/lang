//! File describing all the variants of Tokens that can be present within a
//! Hash source file.
//!
//! All rights reserved 2021 (c) The Hash Language authors

use std::fmt;

use crate::caching::{StringIdentifier, STRING_LITERAL_MAP};
use hash_ast::ident::Identifier;
use hash_ast::ident::IDENTIFIER_MAP;
use hash_ast::keyword::Keyword;
use hash_ast::location::Location;

pub type TokenResult<T> = Result<T, TokenError>;

/// A Lexeme token that represents the smallest code unit of a hash source file. The
/// token contains a kind which is elaborated by [TokenKind] and a [Location] in the
/// source that is represented as a span. The span is the beginning byte offset, and the
/// number of bytes for the said token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Location,
}

impl Token {
    pub fn new(kind: TokenKind, span: Location) -> Self {
        Token { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TokenKind::Ident(ident) => write!(f, "Ident ({})", IDENTIFIER_MAP.ident_name(*ident)),
            TokenKind::Tree(delim, tree) => {
                writeln!(f, "Delimiter({})", delim.left())?;

                for token in tree {
                    writeln!(f, "{}", token)?;
                }

                write!(f, "Delimiter({})", delim.right())
            }
            TokenKind::StrLiteral(literal) => {
                write!(
                    f,
                    "StringLiteral (\"{}\")",
                    STRING_LITERAL_MAP.lookup(*literal)
                )
            }
            // We want to print the actual character, instead of a potential escape code
            TokenKind::CharLiteral(ch) => {
                write!(f, "CharLiteral ('{}')", ch)
            }
            kind => write!(f, "{:?}", kind),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Delimiter {
    Paren,
    Brace,
    Bracket,
}

impl Delimiter {
    pub fn from_left(ch: char) -> Option<Delimiter> {
        match ch {
            '(' => Some(Delimiter::Paren),
            '[' => Some(Delimiter::Bracket),
            '{' => Some(Delimiter::Brace),
            _ => None,
        }
    }

    /// Get the left-hand side variant of a corresponding delimiter
    pub fn left(&self) -> char {
        match self {
            Delimiter::Paren => '(',
            Delimiter::Bracket => '[',
            Delimiter::Brace => '{',
        }
    }

    /// Get the right-hand side variant of a corresponding delimiter
    pub fn right(&self) -> char {
        match self {
            Delimiter::Paren => ')',
            Delimiter::Bracket => ']',
            Delimiter::Brace => '}',
        }
    }
}

/// A TokenKind represents all variants of a token that can be present in a source file. Must of the
/// kinds only represents a single character, but some tokens account for an entire literal or an identifier.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// '='
    Eq,
    /// '<'
    Lt,
    /// '>'
    Gt,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Star,
    /// '/'
    Slash,
    /// '%'
    Percent,
    /// '^'
    Caret,
    /// '&'
    Amp,
    /// '~'
    Tilde,
    /// '|'
    Pipe,
    /// '!'
    Exclamation,
    /// '.'
    Dot,
    /// ':'
    Colon,
    /// ';'
    Semi,
    /// ','
    Comma,
    /// '"'
    Quote,
    /// "'"
    SingleQuote,
    /// Integer Literal
    IntLiteral(u64),
    /// Float literal
    FloatLiteral(f64),
    /// Character literal
    CharLiteral(char),
    /// StrLiteral,
    StrLiteral(StringIdentifier),
    /// Keyword
    Keyword(Keyword),
    /// Identifier
    Ident(Identifier),

    /// A token tree is represented by an arbitrary number of tokens that are surrounded by
    /// a given delimiter kind, the variants are specified in the [Delimiter] enum.
    Tree(Delimiter, Vec<Token>),
}

impl TokenKind {
    /// Check if a [TokenKind] can be considered in a situation as a unary operator.
    pub(crate) fn is_unary_op(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Amp
                | TokenKind::Tilde
                | TokenKind::Exclamation
        )
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Eq => write!(f, "="),
            TokenKind::Lt => write!(f, ">"),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Amp => write!(f, "&"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Exclamation => write!(f, "!"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Quote => write!(f, "\""),
            TokenKind::SingleQuote => write!(f, "'"),
            TokenKind::IntLiteral(num) => write!(f, "{}", num),
            TokenKind::FloatLiteral(num) => write!(f, "{}", num),
            TokenKind::CharLiteral(ch) => write!(f, "'{}'", ch),
            TokenKind::StrLiteral(str) => {
                write!(f, "\"{}\"", STRING_LITERAL_MAP.lookup(*str))
            }
            TokenKind::Keyword(kwd) => kwd.fmt(f),
            TokenKind::Ident(ident) => {
                write!(f, "{}", IDENTIFIER_MAP.ident_name(*ident))
            },
            TokenKind::Tree(delim, _) => {
                write!(f, "{}", delim.left())
            }
        }
    }
}

/// A [TokenError] represents a encountered error during tokenisation, which includes an optional message
/// with the error, the [TokenErrorKind] which classifies the error, and a [ast::Location] that represents
/// where the tokenisation error occurred.
#[derive(Debug)]
pub struct TokenError {
    pub(crate) message: Option<String>,
    kind: TokenErrorKind,
    location: Location,
}

/// A [TokenErrorKind] represents the kind of [TokenError] which gives additional context to the error
/// with the provided message in [TokenError]
#[derive(Debug)]
pub enum TokenErrorKind {
    /// Occurs when a escape sequence (within a character or a string) is malformed.
    BadEscapeSequence,
    /// Occurs when a numerical literal doesn't follow the language specification, or is too large.
    MalformedNumericalLiteral,
    /// Occurs when a char is unexpected in the current context
    Unexpected(char),
    /// Occurs when the tokeniser expects a particular token next, but could not derive one.
    Expected(TokenKind),
    /// Unclosed tree block
    Unclosed(Delimiter),
}

impl TokenError {
    pub fn new(message: Option<String>, kind: TokenErrorKind, location: Location) -> Self {
        TokenError {
            message,
            kind,
            location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_size() {
        println!("{:?}", std::mem::size_of::<Delimiter>());
        println!("{:?}", std::mem::size_of::<Token>());
        println!("{:?}", std::mem::size_of::<TokenKind>());
    }
}
