//! Hash compiler AST generation sources. This file contains the sources to the logic
//! that transforms tokens into an AST.
//!
//! All rights reserved 2022 (c) The Hash Language authors

use hash_alloc::{collections::row::Row, row};
use hash_ast::{ast::*, ast_nodes, ident::CORE_IDENTIFIERS};
use hash_source::location::Location;
use hash_token::{delimiter::Delimiter, keyword::Keyword, Token, TokenKind, TokenKindVector};

use crate::{disable_flag, enable_flag};

use super::{error::AstGenErrorKind, AstGen, AstGenResult};

impl<'c, 'stream, 'resolver> AstGen<'c, 'stream, 'resolver> {
    /// Parse a compound [Pattern]. A compound [Pattern] means that this could be a
    /// pattern that might be a combination of multiple patterns. Additionally, compound
    /// patterns are allowed to have `if-guard` syntax which permits for conditional matching
    /// of a pattern. There are only a few contexts where the full range of patterns is allowed
    /// (such as the `match` cases).
    pub fn parse_pattern(&self) -> AstGenResult<'c, AstNode<'c, Pattern<'c>>> {
        // attempt to get the next token location as we're starting a pattern here, if there is no token
        // we should exit and return an error
        let start = self.next_location();

        // Parse the first pattern, but throw away the location information since that will be
        // computed at the end anyway...
        let mut patterns = ast_nodes![&self.wall;];

        while self.has_token() {
            let pattern = self.parse_pattern_with_if()?;
            patterns.nodes.push(pattern, &self.wall);

            // Check if this is going to be another pattern following the current one.
            match self.peek() {
                Some(token) if token.has_kind(TokenKind::Pipe) => {
                    self.skip_token();
                }
                _ => break,
            }
        }

        // if the length of patterns is greater than one, we return an 'OR' pattern,
        // otherwise just the first pattern.
        if patterns.len() == 1 {
            Ok(patterns.nodes.pop().unwrap())
        } else {
            Ok(self.node_with_joined_span(Pattern::Or(OrPattern { variants: patterns }), &start))
        }
    }

    /// Parse a [Pattern] with an optional `if-guard` after the singular pattern.
    pub fn parse_pattern_with_if(&self) -> AstGenResult<'c, AstNode<'c, Pattern<'c>>> {
        let start = self.next_location();
        let pattern = self.parse_singular_pattern()?;

        match self.peek() {
            Some(token) if token.has_kind(TokenKind::Keyword(Keyword::If)) => {
                self.skip_token();

                let condition = self.parse_expression_with_precedence(0)?;

                Ok(self
                    .node_with_joined_span(Pattern::If(IfPattern { pattern, condition }), &start))
            }
            _ => Ok(pattern),
        }
    }

    /// Parse a singular [Pattern]. Singular [Pattern]s cannot have any grouped pattern
    /// operators such as a `|`, if guards or any form of compound pattern.
    pub(crate) fn parse_singular_pattern(&self) -> AstGenResult<'c, AstNode<'c, Pattern<'c>>> {
        let spread_patterns_allowed = self.spread_patterns_allowed.get();

        let start = self.next_location();
        let token = self
            .peek()
            .ok_or_else(|| self.make_error(AstGenErrorKind::EOF, None, None, None))?;

        let pattern = match token {
            Token {
                kind: TokenKind::Ident(ident),
                span,
            } => {
                // this could be either just a binding pattern, enum, or a struct pattern
                self.skip_token();

                // So here we try to parse an access name, if it is only made of a single binding
                // name, we'll just return this as a binding pattern, otherwise it must follow that
                // it is either a enum or struct pattern, if not we report it as an error since
                // access names cannot be used as binding patterns on their own...
                let name = self.parse_access_name(self.node_with_span(*ident, *span))?;

                match self.peek() {
                    // Destructuring pattern for either struct or namespace
                    Some(Token {
                        kind: TokenKind::Tree(Delimiter::Brace, tree_index),
                        span,
                    }) => {
                        self.skip_token();
                        let tree = self.token_trees.get(*tree_index).unwrap();

                        disable_flag!(self; spread_patterns_allowed;
                            let fields = self.parse_destructuring_patterns(tree, *span)?
                        );

                        Pattern::Struct(StructPattern { name, fields })
                    }
                    // enum pattern
                    Some(Token {
                        kind: TokenKind::Tree(Delimiter::Paren, tree_index),
                        span,
                    }) => {
                        self.skip_token();
                        let tree = self.token_trees.get(*tree_index).unwrap();
                        let gen = self.from_stream(tree, *span);

                        disable_flag!(gen; spread_patterns_allowed;
                            let fields = gen.parse_pattern_collection()?
                        );

                        Pattern::Enum(EnumPattern { name, fields })
                    }
                    Some(token) if name.path.len() > 1 => self.error(
                        AstGenErrorKind::Expected,
                        Some(TokenKindVector::begin_pattern_collection(&self.wall)),
                        Some(token.kind),
                    )?,
                    _ => {
                        if *ident == CORE_IDENTIFIERS.underscore {
                            Pattern::Ignore(IgnorePattern)
                        } else {
                            Pattern::Binding(BindingPattern(
                                self.node_with_span(Name { ident: *ident }, *span),
                            ))
                        }
                    }
                }
            }
            // Spread pattern
            token if spread_patterns_allowed && token.has_kind(TokenKind::Dot) => {
                Pattern::Spread(self.parse_spread_pattern()?)
            }

            // Literal patterns: which are disallowed within declarations. @@ErrorReporting: Parse it and maybe report it o?
            token if token.kind.is_literal() => {
                self.skip_token();
                Pattern::Literal(self.convert_literal_kind_into_pattern(&token.kind))
            }
            // Tuple patterns
            Token {
                kind: TokenKind::Tree(Delimiter::Paren, tree_index),
                span,
            } => {
                self.skip_token();
                return self.parse_tuple_pattern(*tree_index, *span);
            }
            // Namespace patterns
            Token {
                kind: TokenKind::Tree(Delimiter::Brace, tree_index),
                span,
            } => {
                self.skip_token();
                let tree = self.token_trees.get(*tree_index).unwrap();

                disable_flag!(self; spread_patterns_allowed;
                    let fields = self.parse_destructuring_patterns(tree, *span)?
                );

                Pattern::Namespace(NamespacePattern { fields })
            }
            // List pattern
            Token {
                kind: TokenKind::Tree(Delimiter::Bracket, tree_index),
                span,
            } => {
                self.skip_token();
                return self.parse_list_pattern(*tree_index, *span);
            }
            token => self.error_with_location(
                AstGenErrorKind::Expected,
                Some(TokenKindVector::begin_pattern(&self.wall)),
                Some(token.kind),
                token.span,
            )?,
        };

        Ok(self.node_with_joined_span(pattern, &start))
    }

    /// Parse an arbitrary number of [Pattern]s which are comma separated.
    pub fn parse_pattern_collection(&self) -> AstGenResult<'c, AstNodes<'c, Pattern<'c>>> {
        self.parse_separated_fn(
            || self.parse_pattern(),
            || self.parse_token_atom(TokenKind::Comma),
        )
    }

    /// Parse a [DestructuringPattern]. The [DestructuringPattern] refers to destructuring
    /// either a struct or a namespace to extract fields, exported members. The function
    /// takes in a token atom because both syntaxes use different operators as pattern
    /// assigners.
    pub(crate) fn parse_destructuring_pattern(
        &self,
    ) -> AstGenResult<'c, AstNode<'c, DestructuringPattern<'c>>> {
        let start = self.current_location();
        let name = self.parse_name()?;

        // if the next token is the correct assigning operator, attempt to parse a
        // pattern here, if not then we copy the parsed ident and make a binding
        // pattern.
        let pattern = match self.peek_resultant_fn(|| self.parse_token_atom(TokenKind::Eq)) {
            Some(_) => self.parse_pattern()?,
            None => {
                let span = name.location();
                let copy = self.node(Name { ..*name.body() });

                self.node_with_span(Pattern::Binding(BindingPattern(copy)), span)
            }
        };

        Ok(self.node_with_joined_span(DestructuringPattern { name, pattern }, &start))
    }

    /// Parse a collection of [DestructuringPattern]s that are comma separated.
    pub(crate) fn parse_destructuring_patterns(
        &self,
        tree: &'stream Row<'stream, Token>,
        span: Location,
    ) -> AstGenResult<'c, AstNodes<'c, DestructuringPattern<'c>>> {
        let gen = self.from_stream(tree, span);

        let mut patterns = AstNodes::new(row![&self.wall;], Some(span));

        while gen.has_token() {
            match gen.peek_resultant_fn(|| gen.parse_destructuring_pattern()) {
                Some(pat) => patterns.nodes.push(pat, &self.wall),
                None => break,
            }

            if gen.has_token() {
                gen.parse_token_atom(TokenKind::Comma)?;
            }
        }

        // @@ErrorReporting: So here, there is a problem because we do actually want to report
        //                   that this should have been the end of the pattern but because in some
        //                   contexts the function is being peeked and the error is being ignored,
        //                   maybe there should be some mechanism to cause the function to hard error?
        if gen.has_token() {
            gen.expected_eof()?;
        }

        Ok(patterns)
    }

    /// Parse a [Pattern::List] pattern from the token vector. A list [Pattern] consists
    /// of a list of comma separated within a square brackets .e.g `[x, 1, ..]`
    pub(crate) fn parse_list_pattern(
        &self,
        tree_index: usize,
        parent_span: Location,
    ) -> AstGenResult<'c, AstNode<'c, Pattern<'c>>> {
        let tree = self.token_trees.get(tree_index).unwrap();
        let gen = self.from_stream(tree, parent_span);

        enable_flag!(gen; spread_patterns_allowed;
            let fields = gen.parse_pattern_collection()?
        );

        Ok(self.node_with_span(Pattern::List(ListPattern { fields }), parent_span))
    }

    /// Parse a [Pattern::Tuple] from the token vector. A tuple pattern consists of
    /// nested patterns within parenthesees which might also have an optional
    /// named fields.
    ///
    /// If only a singular pattern is parsed and it doesn't have a name, then the
    /// function will assume that this is not a tuple pattern and simply a pattern
    /// wrapped within parenthesees.
    pub(crate) fn parse_tuple_pattern(
        &self,
        tree_index: usize,
        parent_span: Location,
    ) -> AstGenResult<'c, AstNode<'c, Pattern<'c>>> {
        let tree = self.token_trees.get(tree_index).unwrap();

        // check here if the tree length is 1, and the first token is the comma to check if it is an
        // empty tuple pattern...
        if let Some(token) = tree.get(0) {
            if token.has_kind(TokenKind::Comma) {
                return Ok(self.node_with_span(
                    Pattern::Tuple(TuplePattern {
                        fields: AstNodes::empty(),
                    }),
                    parent_span,
                ));
            }
        }

        // @@Hack: here it might actually be a nested pattern in parenthesees. So we perform a slight
        // transformation if the number of parsed patterns is only one. So essentially we handle the case
        // where a pattern is wrapped in parentheses and so we just unwrap it.
        let gen = self.from_stream(tree, parent_span);

        enable_flag!(gen; spread_patterns_allowed;
            // @@Cleanup: In the case that there is a single pattern and the user writes a `,` does this
            //            mean that this is still a singular pattern or if it is now treated as a tuple pattern?
            let mut elements = gen.parse_separated_fn(
                || gen.parse_tuple_pattern_entry(),
                || gen.parse_token_atom(TokenKind::Comma),
            )?
        );

        // If there is no associated name with the entry and there is only one entry
        // then we can be sure that it is only a nested entry.
        if elements.len() == 1 && elements[0].name.is_none() {
            let element = elements.nodes.pop().unwrap();

            Ok(element.into_body().move_out().pattern)
        } else {
            Ok(self.node_with_span(
                Pattern::Tuple(TuplePattern { fields: elements }),
                parent_span,
            ))
        }
    }

    /// Parse an entry within a tuple pattern which might contain an optional [Name] node.
    pub(crate) fn parse_tuple_pattern_entry(
        &self,
    ) -> AstGenResult<'c, AstNode<'c, TuplePatternEntry<'c>>> {
        let start = self.current_location();

        let (name, pattern) = match self.peek() {
            Some(Token {
                kind: TokenKind::Ident(_),
                ..
            }) => {
                // Here if there is a '=', this means that there is a name attached to the entry within the
                // tuple pattern...
                match self.peek_second() {
                    Some(token) if token.has_kind(TokenKind::Eq) => {
                        let name = self.parse_name()?;
                        self.skip_token(); // '='

                        (Some(name), self.parse_pattern()?)
                    }
                    _ => (None, self.parse_pattern()?),
                }
            }
            _ => (None, self.parse_pattern()?),
        };

        Ok(self.node_with_joined_span(TuplePatternEntry { name, pattern }, &start))
    }

    /// Convert a [Literal] into a [LiteralPattern].
    pub(crate) fn convert_literal_kind_into_pattern(&self, kind: &TokenKind) -> LiteralPattern {
        match kind {
            TokenKind::StrLiteral(s) => LiteralPattern::Str(StrLiteralPattern(*s)),
            TokenKind::CharLiteral(s) => LiteralPattern::Char(CharLiteralPattern(*s)),
            TokenKind::IntLiteral(s) => LiteralPattern::Int(IntLiteralPattern(*s)),
            TokenKind::FloatLiteral(s) => LiteralPattern::Float(FloatLiteralPattern(*s)),
            TokenKind::Keyword(Keyword::False) => LiteralPattern::Bool(BoolLiteralPattern(false)),
            TokenKind::Keyword(Keyword::True) => LiteralPattern::Bool(BoolLiteralPattern(true)),
            _ => unreachable!(),
        }
    }

    /// Parse a spread operator from the current token tree. A spread operator can have an
    /// optional name attached to the spread operator on the right hand-side.
    ///
    /// ## Allowed locations
    /// So the spread operator can only appear within either `list`, `tuple` patterns at the moment
    /// which means that any other location will mark it as `invalid` in the current implementation.
    ///
    pub(crate) fn parse_spread_pattern(&self) -> AstGenResult<'c, SpreadPattern<'c>> {
        for _ in 0..3 {
            self.parse_token_atom(TokenKind::Dot)?;
        }

        // Try and see if there is a identifier that is followed by the spread to try and
        // bind the capture to a variable
        let name = self.peek_resultant_fn(|| self.parse_name());

        Ok(SpreadPattern { name })
    }
}