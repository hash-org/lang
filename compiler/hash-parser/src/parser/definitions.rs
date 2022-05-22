//! Hash compiler AST generation sources. This file contains the sources to the logic
//! that transforms tokens into an AST.
//!
//! All rights reserved 2022 (c) The Hash Language authors

use hash_ast::ast::*;
use hash_token::{delimiter::Delimiter, keyword::Keyword, Token, TokenKind, TokenKindVector};

use crate::parser::error::TyArgumentKind;

use super::{error::AstGenErrorKind, AstGen, AstGenResult};

impl<'c, 'stream, 'resolver> AstGen<'c, 'stream, 'resolver> {
    /// Parse a [StructDef]. The keyword `struct` begins the construct and is followed
    /// by parenthesees with inner struct fields defined.
    pub fn parse_struct_def(&self) -> AstGenResult<'c, StructDef<'c>> {
        debug_assert!(self
            .current_token()
            .has_kind(TokenKind::Keyword(Keyword::Struct)));

        let entries = match self.peek() {
            Some(Token {
                kind: TokenKind::Tree(Delimiter::Paren, tree_index),
                span,
            }) => {
                self.skip_token();
                let tree = self.token_trees.get(*tree_index).unwrap();
                let gen = self.from_stream(tree, *span);

                gen.parse_separated_fn(
                    || gen.parse_struct_def_entry(),
                    || gen.parse_token_atom(TokenKind::Comma),
                )?
            }
            token => self.error_with_location(
                AstGenErrorKind::TypeDefinition(TyArgumentKind::Struct),
                None,
                token.map(|t| t.kind),
                token.map_or_else(|| self.next_location(), |t| t.span),
            )?,
        };

        Ok(StructDef { entries })
    }

    /// Parse a [StructDefEntry].
    pub fn parse_struct_def_entry(&self) -> AstGenResult<'c, AstNode<'c, StructDefEntry<'c>>> {
        let start = self.current_location();
        let name = self.parse_name()?;

        let ty = match self.peek() {
            Some(token) if token.has_kind(TokenKind::Colon) => {
                self.skip_token();
                Some(self.parse_type()?)
            }
            _ => None,
        };

        let default = match self.peek() {
            Some(token) if token.has_kind(TokenKind::Eq) => {
                self.skip_token();

                Some(self.parse_expression_with_precedence(0)?)
            }
            _ => None,
        };

        Ok(self.node_with_joined_span(StructDefEntry { name, ty, default }, &start))
    }

    /// Parse an [EnumDef]. The keyword `enum` begins the construct and is followed
    /// by parenthesees with inner enum fields defined.
    pub fn parse_enum_def(&self) -> AstGenResult<'c, EnumDef<'c>> {
        debug_assert!(self
            .current_token()
            .has_kind(TokenKind::Keyword(Keyword::Enum)));

        let entries = match self.peek() {
            Some(Token {
                kind: TokenKind::Tree(Delimiter::Paren, tree_index),
                span,
            }) => {
                self.skip_token();
                let tree = self.token_trees.get(*tree_index).unwrap();
                let gen = self.from_stream(tree, *span);

                gen.parse_separated_fn(
                    || gen.parse_enum_def_entry(),
                    || gen.parse_token_atom(TokenKind::Comma),
                )?
            }
            token => self.error_with_location(
                AstGenErrorKind::TypeDefinition(TyArgumentKind::Enum),
                None,
                token.map(|t| t.kind),
                token.map_or_else(|| self.next_location(), |t| t.span),
            )?,
        };

        Ok(EnumDef { entries })
    }

    /// Parse an [EnumDefEntry].
    pub fn parse_enum_def_entry(&self) -> AstGenResult<'c, AstNode<'c, EnumDefEntry<'c>>> {
        let name = self.parse_name()?;
        let name_span = name.location();

        let mut args = AstNodes::empty();

        if let Some(Token {
            kind: TokenKind::Tree(Delimiter::Paren, tree_index),
            span,
        }) = self.peek()
        {
            self.skip_token();
            args.span = Some(*span);
            let tree = self.token_trees.get(*tree_index).unwrap();

            let gen = self.from_stream(tree, *span);
            while gen.has_token() {
                let ty = gen.parse_type()?;
                args.nodes.push(ty, &self.wall);

                if gen.has_token() {
                    gen.parse_token_atom(TokenKind::Comma)?;
                }
            }
        }

        Ok(self.node_with_joined_span(EnumDefEntry { name, args }, &name_span))
    }

    /// Parse a [TypeFunctionDef]. Type functions specify logic at the type level on expressions such as
    /// struct, enum, function, and trait definitions.
    pub fn parse_type_function_def(&self) -> AstGenResult<'c, TypeFunctionDef<'c>> {
        // @@Hack: Since we already parsed the `<`, we need to notify the
        //         type_args parser function that it doesn't need to parse this
        let type_args = self.parse_type_args(true)?;

        // Now that we parse the bound, we're expecting a fat-arrow and then some expression
        self.parse_arrow()?;
        let expr = self.parse_expression_with_precedence(0)?;

        Ok(TypeFunctionDef {
            args: type_args,
            expr,
        })
    }

    /// Parse a [TraitDef]. A [TraitDef] is essentially a block prefixed with `trait` that contains
    /// definitions or attach expressions to a trait.
    pub fn parse_trait_def(&self) -> AstGenResult<'c, TraitDef<'c>> {
        debug_assert!(self
            .current_token()
            .has_kind(TokenKind::Keyword(Keyword::Trait)));

        let members = match self.peek() {
            Some(Token {
                kind: TokenKind::Tree(Delimiter::Brace, tree_index),
                span,
            }) => {
                self.skip_token();
                let tree = self.token_trees.get(*tree_index).unwrap();
                let gen = self.from_stream(tree, *span);

                gen.parse_separated_fn(
                    || gen.parse_expression(),
                    || gen.parse_token_atom(TokenKind::Semi),
                )?
            }
            token => self.error_with_location(
                AstGenErrorKind::Expected,
                Some(TokenKindVector::singleton(
                    &self.wall,
                    TokenKind::Delimiter(Delimiter::Brace, true),
                )),
                token.map(|t| t.kind),
                token.map_or_else(|| self.next_location(), |t| t.span),
            )?,
        };

        Ok(TraitDef { members })
    }
}
