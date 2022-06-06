use hash_ast::ast::*;
use hash_token::{keyword::Keyword, TokenKind};

use super::{error::AstGenErrorKind, AstGen, AstGenResult};

impl<'c, 'stream, 'resolver> AstGen<'c, 'stream, 'resolver> {
    /// This function is used to pickup 'glued' operator tokens to form more complex binary operators
    /// that might be made up of multiple tokens. The function will peek ahead (2 tokens at most since
    /// all binary operators are made of that many tokens). The function returns an optional derived
    /// operator, and the number of tokens that was consumed deriving the operator, it is the responsibility
    /// of the caller to increment the token stream by the provided number.
    pub(crate) fn parse_binary_operator(&self) -> (Option<BinaryOperator>, u8) {
        let token = self.peek();

        // check if there is a token that we can peek at ahead...
        if token.is_none() {
            return (None, 0);
        }

        match &(token.unwrap()).kind {
            // Since the 'as' keyword is also a binary operator, we have to handle it here...
            TokenKind::Keyword(Keyword::As) => (Some(BinaryOperator::As), 1),
            TokenKind::Eq => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Eq => (Some(BinaryOperator::EqEq), 2),
                _ => (None, 0),
            },
            TokenKind::Lt => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Eq => (Some(BinaryOperator::LtEq), 2),
                Some(token) if token.kind == TokenKind::Lt => (Some(BinaryOperator::Shl), 2),
                _ => (Some(BinaryOperator::Lt), 1),
            },
            TokenKind::Gt => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Eq => (Some(BinaryOperator::GtEq), 2),
                Some(token) if token.kind == TokenKind::Gt => (Some(BinaryOperator::Shr), 2),
                _ => (Some(BinaryOperator::Gt), 1),
            },
            TokenKind::Plus => (Some(BinaryOperator::Add), 1),
            TokenKind::Minus => (Some(BinaryOperator::Sub), 1),
            TokenKind::Star => (Some(BinaryOperator::Mul), 1),
            TokenKind::Slash => (Some(BinaryOperator::Div), 1),
            TokenKind::Percent => (Some(BinaryOperator::Mod), 1),
            TokenKind::Caret => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Caret => (Some(BinaryOperator::Exp), 2),
                _ => (Some(BinaryOperator::BitXor), 1),
            },
            TokenKind::Amp => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Amp => (Some(BinaryOperator::And), 2),
                _ => (Some(BinaryOperator::BitAnd), 1),
            },
            TokenKind::Pipe => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Pipe => (Some(BinaryOperator::Or), 2),
                _ => (Some(BinaryOperator::BitOr), 1),
            },
            TokenKind::Exclamation => match self.peek_second() {
                Some(token) if token.kind == TokenKind::Eq => (Some(BinaryOperator::NotEq), 2),
                _ => (None, 0), // this is a unary operator '!'
            },
            _ => (None, 0),
        }
    }

    /// Function to parse a fat arrow component '=>' in any given context.
    pub(crate) fn parse_arrow(&self) -> AstGenResult<'c, ()> {
        // Essentially, we want to re-map the error into a more concise one given
        // the parsing context.
        if self.parse_token_fast(TokenKind::Eq).is_none() {
            return self.error(AstGenErrorKind::ExpectedArrow, None, None)?;
        }

        if self.parse_token_fast(TokenKind::Gt).is_none() {
            return self.error(AstGenErrorKind::ExpectedArrow, None, None)?;
        }

        Ok(())
    }

    /// Function to parse a fat arrow component '=>' in any given context.
    pub(crate) fn parse_thin_arrow(&self) -> AstGenResult<'c, ()> {
        // Essentially, we want to re-map the error into a more concise one given
        // the parsing context.
        if self.parse_token_fast(TokenKind::Minus).is_none() {
            return self.error(AstGenErrorKind::ExpectedFnArrow, None, None)?;
        }

        if self.parse_token_fast(TokenKind::Gt).is_none() {
            return self.error(AstGenErrorKind::ExpectedFnArrow, None, None)?;
        }

        Ok(())
    }
}
