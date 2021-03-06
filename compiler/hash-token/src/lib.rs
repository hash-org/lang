//! Hash Compiler token definitions that are used by the lexer when lexing
//! the input sources.
pub mod delimiter;
pub mod keyword;

use delimiter::Delimiter;
use hash_source::{identifier::Identifier, location::Span, string::Str};
use keyword::Keyword;

/// A Lexeme token that represents the smallest code unit of a hash source file.
/// The token contains a kind which is elaborated by [TokenKind] and a [Span] in
/// the source that is represented as a span. The span is the beginning byte
/// offset, and the number of bytes for the said token.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The current token type.
    pub kind: TokenKind,
    /// The span of the current token.
    pub span: Span,
}

impl Token {
    /// Create a new token from a kind and a provided [Span].
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    /// Check if the token has the specified token kind.
    pub fn has_kind(&self, right: TokenKind) -> bool {
        self.kind == right
    }

    /// Check if the token is a tree and the tree beginning character
    /// is a brace.
    pub fn is_brace_tree(&self) -> bool {
        matches!(self.kind, TokenKind::Tree(Delimiter::Brace, _))
    }

    /// Check if the token is a tree and the tree beginning character
    /// is a parenthesis.
    pub fn is_paren_tree(&self) -> bool {
        matches!(self.kind, TokenKind::Tree(Delimiter::Paren, _))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::Ident(ident) => {
                write!(f, "Ident ({})", String::from(*ident))
            }
            TokenKind::StrLit(lit) => {
                write!(f, "String (\"{}\")", String::from(*lit))
            }
            // We want to print the actual character, instead of a potential escape code
            TokenKind::CharLit(ch) => {
                write!(f, "Char ('{}')", ch)
            }
            kind => write!(f, "{:?}", kind),
        }
    }
}

impl TokenKind {
    /// Check if a [TokenKind] can be considered in a situation as a unary
    /// operator.
    pub fn is_unary_op(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::Hash // directives
                    | TokenKind::Amp
                    | TokenKind::Tilde
                    | TokenKind::Exclamation
                    | TokenKind::Keyword(Keyword::Unsafe)
        )
    }

    /// Check if the current token can begin a pattern

    /// Checks if the [TokenKind] must begin a block, as in the specified
    /// keywords that follow a specific syntax, and must be statements.
    pub fn begins_block(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(Keyword::For)
                | TokenKind::Keyword(Keyword::While)
                | TokenKind::Keyword(Keyword::Loop)
                | TokenKind::Keyword(Keyword::Mod)
                | TokenKind::Keyword(Keyword::If)
                | TokenKind::Keyword(Keyword::Match)
                | TokenKind::Keyword(Keyword::Impl)
        )
    }

    /// Check if the [TokenKind] is a primitive literal; either a 'char', 'int',
    /// 'float' or a 'string'
    pub fn is_lit(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(Keyword::False)
                | TokenKind::Keyword(Keyword::True)
                | TokenKind::IntLit(_)
                | TokenKind::FloatLit(_)
                | TokenKind::CharLit(_)
                | TokenKind::StrLit(_)
        )
    }
}

/// An Atom represents all variants of a token that can be present in a source
/// file. Atom token kinds can represent a single character, literal or an
/// identifier.
#[derive(Debug, PartialEq, Copy, Clone)]
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
    /// '?'
    Question,
    /// '!'
    Exclamation,
    /// '.'
    Dot,
    /// ':'
    Colon,
    /// ';'
    Semi,
    /// '#'
    Hash,
    /// '$'
    Dollar,
    /// ','
    Comma,
    /// '"'
    Quote,
    /// "'"
    SingleQuote,
    /// Integer Literal
    IntLit(u64),
    /// Float literal
    FloatLit(f64),
    /// Character literal
    CharLit(char),
    /// StrLiteral,
    StrLit(Str),
    /// Identifier
    Ident(Identifier),

    /// Tree
    Tree(Delimiter, usize),

    /// Keyword
    Keyword(Keyword),

    /// Delimiter: '(' '{', '[' and right hand-side variants, useful for error
    /// reporting and messages. The boolean flag represents if the delimiter
    /// is left or right, If it's true, then it is the left variant.
    Delimiter(Delimiter, bool),

    /// A token that was unexpected by the lexer, e.g. a unicode symbol not
    /// within string literal.
    Unexpected(char),
}

impl TokenKind {
    /// This function is used to create an error message representing when a
    /// token was unexpectedly encountered or was expected in a particular
    /// context.
    pub fn as_error_string(&self) -> String {
        match self {
            TokenKind::Unexpected(ch) => format!("an unknown character `{}`", ch),
            TokenKind::IntLit(num) => format!("`{}`", num),
            TokenKind::FloatLit(num) => format!("`{}`", num),
            TokenKind::CharLit(ch) => format!("`{}`", ch),
            TokenKind::StrLit(str) => {
                format!("the string `{}`", *str)
            }
            TokenKind::Keyword(kwd) => format!("`{}`", kwd),
            TokenKind::Ident(ident) => {
                format!("the identifier `{}`", *ident)
            }
            kind => format!("a `{}`", kind),
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Eq => write!(f, "="),
            TokenKind::Lt => write!(f, "<"),
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
            TokenKind::Question => write!(f, "?"),
            TokenKind::Exclamation => write!(f, "!"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::Dollar => write!(f, "$"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Quote => write!(f, "\""),
            TokenKind::SingleQuote => write!(f, "'"),
            TokenKind::Unexpected(ch) => write!(f, "{}", ch),
            TokenKind::IntLit(num) => write!(f, "{}", num),
            TokenKind::FloatLit(num) => write!(f, "{}", num),
            TokenKind::CharLit(ch) => write!(f, "'{}'", ch),
            TokenKind::Delimiter(delim, left) => {
                if *left {
                    write!(f, "{}", delim.left())
                } else {
                    write!(f, "{}", delim.right())
                }
            }
            TokenKind::Tree(delim, _) => write!(f, "{}...{}", delim.left(), delim.right()),
            TokenKind::StrLit(str) => {
                write!(f, "\"{}\"", *str)
            }
            TokenKind::Keyword(kwd) => kwd.fmt(f),
            TokenKind::Ident(ident) => {
                write!(f, "{}", String::from(*ident))
            }
        }
    }
}

/// This is a wrapper around a vector of token atoms that can represent the
/// expected tokens in a given context when transforming the token tree into and
/// an AST. The wrapper exists because once again you cannot specify
/// implementations for types that don't originate from the current crate.
///
/// @@TODO(alex): Instead of using a [TokenKind], we should use an enum to
/// custom variants or descriptors such as 'operator'. Instead of token atoms we
/// can just the display representations of the token atoms. Or even better, we
/// can use the [`ToString`] trait and just auto cast into a string, whilst
/// holding a vector of strings.
#[derive(Debug)]
pub struct TokenKindVector(Vec<TokenKind>);

impl TokenKindVector {
    /// Create a new empty [TokenKindVector].
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn inner(&self) -> &Vec<TokenKind> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<TokenKind> {
        self.0
    }

    /// Create a [TokenKindVector] from a provided row of expected atoms.
    pub fn from_row(items: Vec<TokenKind>) -> Self {
        Self(items)
    }

    /// Check if the current [TokenKindVector] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Create a [TokenKindVector] with a single atom.
    pub fn singleton(kind: TokenKind) -> Self {
        Self(vec![kind])
    }

    #[inline(always)]
    pub fn begin_visibility() -> Self {
        Self(vec![TokenKind::Keyword(Keyword::Pub), TokenKind::Keyword(Keyword::Priv)])
    }

    /// Tokens expected when the parser expects a collection of patterns to be
    /// present.
    pub fn begin_pat_collection() -> Self {
        Self(vec![TokenKind::Delimiter(Delimiter::Paren, true), TokenKind::Colon])
    }

    /// Tokens expected when a pattern begins in a match statement.
    pub fn begin_pat() -> Self {
        Self(vec![
            TokenKind::Delimiter(Delimiter::Paren, true),
            TokenKind::Delimiter(Delimiter::Brace, true),
            TokenKind::Delimiter(Delimiter::Bracket, true),
        ])
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
