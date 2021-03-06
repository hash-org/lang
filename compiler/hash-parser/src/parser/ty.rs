//! Hash Compiler AST generation sources. This file contains the sources to the
//! logic that transforms tokens into an AST.
use hash_ast::ast::*;
use hash_token::{delimiter::Delimiter, keyword::Keyword, Token, TokenKind, TokenKindVector};

use super::{error::AstGenErrorKind, AstGen, AstGenResult};

impl<'stream, 'resolver> AstGen<'stream, 'resolver> {
    /// Parse a [Ty]. This includes all forms of a [Ty]. This function
    /// does not deal with any kind of [Ty] annotation or [TyFnDef]
    /// syntax.
    pub(crate) fn parse_type(&self) -> AstGenResult<AstNode<Ty>> {
        self.parse_type_with_precedence(0)
    }

    /// Parse a [Ty] with precedence in mind. The type notation within hash
    /// contains [TyOp] operators which are binary operators for type terms.
    /// This function implements the same precedence algorithm for correctly
    /// forming binary type expressions.
    fn parse_type_with_precedence(&self, min_prec: u8) -> AstGenResult<AstNode<Ty>> {
        let mut lhs = self.parse_singular_type()?;
        let lhs_span = lhs.span();

        loop {
            let (op, consumed_tokens) = self.parse_type_operator();

            match op {
                Some(op) => {
                    let (l_prec, r_prec) = op.infix_binding_power();

                    // If the precedence from this operator is lower than our `min_prec` we
                    // need to backtrack...
                    if l_prec < min_prec {
                        break;
                    }

                    self.offset.update(|x| x + consumed_tokens as usize);

                    // Now recurse and get the `rhs` of the operator.
                    let rhs = self.parse_type_with_precedence(r_prec)?;

                    // transform the operator into an `UnaryTy` or `MergeTy` based on the operator
                    lhs =
                        match op {
                            BinTyOp::Union => self
                                .node_with_joined_span(Ty::Union(UnionTy { lhs, rhs }), &lhs_span),
                            BinTyOp::Merge => self
                                .node_with_joined_span(Ty::Merge(MergeTy { lhs, rhs }), &lhs_span),
                        }
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Parse a [Ty]. This includes only singular forms of a type.
    fn parse_singular_type(&self) -> AstGenResult<AstNode<Ty>> {
        let token = self.peek().ok_or_else(|| {
            self.make_error(AstGenErrorKind::ExpectedType, None, None, Some(self.next_location()))
        })?;

        let mut multi_ty_components = true;
        let span = token.span;

        // Parse the initial part of the type
        let mut ty = match &token.kind {
            TokenKind::Amp => {
                self.skip_token();

                // Check if this is a raw ref
                let kind = self
                    .parse_token_fast(TokenKind::Keyword(Keyword::Raw))
                    .map(|_| self.node_with_span(RefKind::Raw, self.current_location()));

                // Parse a mutability modifier if any
                let mutability = self
                    .parse_token_fast(TokenKind::Keyword(Keyword::Mut))
                    .map(|_| self.node_with_span(Mutability::Mutable, self.current_location()));

                Ty::Ref(RefTy { inner: self.parse_type()?, kind, mutability })
            }
            TokenKind::Ident(id) => {
                self.skip_token();

                let name = self.node_with_span(Name { ident: *id }, span);
                Ty::Named(NamedTy { name })
            }

            // Map or set type
            TokenKind::Tree(Delimiter::Brace, tree_index) => {
                self.skip_token();

                let tree = self.token_trees.get(*tree_index).unwrap();
                let gen = self.from_stream(tree, token.span);

                let key_ty = gen.parse_type()?;

                match gen.peek() {
                    // This must be a map
                    Some(token) if token.has_kind(TokenKind::Colon) => {
                        gen.skip_token();

                        let value_ty = gen.parse_type()?;
                        gen.verify_is_empty()?;

                        Ty::Map(MapTy { key: key_ty, value: value_ty })
                    }
                    None => Ty::Set(SetTy { inner: key_ty }),
                    Some(_) => gen.expected_eof()?,
                }
            }

            // List type
            TokenKind::Tree(Delimiter::Bracket, tree_index) => {
                self.skip_token();

                let tree = self.token_trees.get(*tree_index).unwrap();
                let gen = self.from_stream(tree, token.span);

                let inner_type = gen.parse_type()?;
                gen.verify_is_empty()?;

                Ty::List(ListTy { inner: inner_type })
            }

            // Tuple or function type
            TokenKind::Tree(Delimiter::Paren, _) => {
                multi_ty_components = false;
                self.parse_fn_or_tuple_ty()?
            }

            // Type function, which is a collection of arguments enclosed in `<...>` and then
            // followed by a return type
            TokenKind::Lt => {
                multi_ty_components = false;
                self.parse_ty_fn()?
            }

            kind => {
                self.error_with_location(AstGenErrorKind::ExpectedType, None, Some(*kind), span)?
            }
        };

        // Deal with type function call, or type access
        while multi_ty_components && let Some(token) = self.peek() {
            ty = match token.kind {
                TokenKind::Lt => {
                    self.skip_token();

                    Ty::TyFnCall(TyFnCall {
                        subject: self.node_with_joined_span(
                            Expr::new(ExprKind::Ty(TyExpr(self.node_with_joined_span(ty, &span)))),
                            &span,
                        ),
                        args: self.parse_ty_args(true)?,
                    })
                }
                TokenKind::Colon if matches!(self.peek_second(), Some(token) if token.has_kind(TokenKind::Colon)) => {
                    self.offset.update(|offset| offset + 2);

                    Ty::Access(AccessTy {
                        subject: self.node_with_joined_span(ty, &span),
                        property: self.parse_name()?
                    })
                }
                _ => break
            }
        }

        Ok(self.node_with_joined_span(ty, &span))
    }

    /// This parses some type arguments after an [AccessName], however due to
    /// the nature of the language grammar, since the [TokenKind] could be a
    /// [`TokenKind::Lt`] or `<`, it could also be a comparison rather than
    /// the beginning of a type argument. Therefore, we have to lookahead to
    /// see if there is another type followed by either a comma (which locks the
    /// `type_args`) or a closing [`TokenKind::Gt`].
    pub(crate) fn parse_ty_args(&self, lt_eaten: bool) -> AstGenResult<AstNodes<TyArg>> {
        // Only parse is if the caller specifies that they haven't eaten an `lt`
        if !lt_eaten {
            self.parse_token(TokenKind::Lt)?;
        }

        let start = self.current_location();
        let mut type_args = vec![];

        loop {
            // The name part of the `NamedFieldTypeEntry` is an identifier followed by an
            // `=`, which will then should contain a type setting.
            let name = match (self.peek(), self.peek_second()) {
                (
                    Some(Token { kind: TokenKind::Ident(_), .. }),
                    Some(Token { kind: TokenKind::Eq, .. }),
                ) => {
                    let name = self.parse_name()?;
                    self.skip_token(); // '='

                    Some(name)
                }
                _ => None,
            };

            // Either way, even if the name is not specified, we still want to parse a name
            // here and hard-error if we don't encounter a type.
            let ty = self.parse_type()?;

            // Here, we want to use either a joined span between the name or just the span
            // of the parsed type
            let arg_span =
                name.as_ref().map_or_else(|| ty.span(), |node| node.span().join(ty.span()));

            type_args.push(self.node_with_span(TyArg { name, ty }, arg_span));

            // Now consider if the bound is closing or continuing with a comma...
            match self.peek() {
                Some(token) if token.has_kind(TokenKind::Comma) => {
                    self.skip_token();
                }
                Some(token) if token.has_kind(TokenKind::Gt) => {
                    self.skip_token();
                    break;
                }
                Some(token) => self.error(
                    AstGenErrorKind::Expected,
                    Some(TokenKindVector::from_row(vec![TokenKind::Comma, TokenKind::Gt])),
                    Some(token.kind),
                )?,
                None => self.unexpected_eof()?,
            }
        }

        // Update the location of the type bound to reflect the '<' and '>' tokens...
        // type_args.set_span(start.join(self.current_location()));
        Ok(AstNodes::new(type_args, Some(start.join(self.current_location()))))
    }

    /// Parses a [Ty::Fn] which involves a parenthesis token tree with some
    /// arbitrary number of comma separated types followed by a return
    /// [Ty] that is preceded by an `thin-arrow` (->) after the
    /// parentheses. e.g. `(i32) -> str`
    fn parse_fn_or_tuple_ty(&self) -> AstGenResult<Ty> {
        let mut params = AstNodes::empty();

        let gen = self.parse_delim_tree(Delimiter::Paren, None)?;
        params.span = gen.parent_span;

        match gen.peek() {
            // Handle special case where there is only one comma and no following items...
            // Special edge case for '(,)' or an empty tuple type...
            Some(token) if token.has_kind(TokenKind::Comma) && gen.stream.len() == 1 => {
                gen.skip_token();
            }
            _ => {
                params = gen.parse_separated_fn(
                    || {
                        let start = gen.next_location();

                        // Here we have to essentially try and parse a identifier. If this is the
                        // case and then there is a colon present then we
                        // have a named field.
                        let (name, ty) = match gen.peek_second() {
                            Some(token) if token.has_kind(TokenKind::Colon) => {
                                let ident = gen.parse_name()?;
                                gen.skip_token(); // :

                                (Some(ident), gen.parse_type()?)
                            }
                            _ => (None, gen.parse_type()?),
                        };

                        Ok(gen.node_with_joined_span(TyArg { name, ty }, &start))
                    },
                    || gen.parse_token(TokenKind::Comma),
                )?;
            }
        };

        // Here we check that the token tree has a comma at the end to later determine
        // if this is a `TupleType`...
        let gen_has_comma =
            !gen.stream.is_empty() && gen.token_at(gen.offset() - 1).has_kind(TokenKind::Comma);

        // If there is an arrow '=>', then this must be a function type
        match self.peek_resultant_fn(|| self.parse_thin_arrow()) {
            Some(_) => {
                // Parse the return type here, and then give the function name
                Ok(Ty::Fn(FnTy { params, return_ty: self.parse_type()? }))
            }
            None => {
                // If there is only one entry in the params, and the last token in the entry is
                // not a comma then we can just return the inner type
                if gen_has_comma && params.len() == 1 && params[0].name.is_none() {
                    let field = params.nodes.pop().unwrap().into_body();
                    return Ok(field.ty.into_body());
                }

                Ok(Ty::Tuple(TupleTy { entries: params }))
            }
        }
    }

    /// Parses a [Ty::TyFn] with the pre-condition that the initial
    /// subject type is parsed and passed into the function. This function
    /// only deals with the argument part of the function.
    fn parse_ty_fn(&self) -> AstGenResult<Ty> {
        // Since this is only called from `parse_singular_type` we know that this should
        // only be fired when the next token is a an `<`
        debug_assert!(matches!(self.next_token(), Some(Token { kind: TokenKind::Lt, .. })));

        let mut arg_span = self.current_location();
        let mut args = vec![];

        loop {
            let span = self.current_location();
            let name = self.parse_name()?;

            let ty = match self.parse_token_fast(TokenKind::Colon) {
                Some(_) => match self.peek() {
                    // Don't try and parse a type if an '=' is followed straight after
                    Some(tok) if tok.has_kind(TokenKind::Eq) => None,
                    _ => Some(self.parse_type()?),
                },
                None => None,
            };

            let default = match self.parse_token_fast(TokenKind::Eq) {
                Some(_) => Some(self.parse_type()?),
                None => None,
            };

            args.push(self.node_with_span(
                Param {
                    name,
                    ty,
                    default: default.map(|node| {
                        let span = node.span();
                        self.node_with_span(Expr::new(ExprKind::Ty(TyExpr(node))), span)
                    }),
                    origin: ParamOrigin::TyFn,
                },
                span,
            ));

            // Now consider if the bound is closing or continuing with a comma...
            match self.peek() {
                Some(token) if token.has_kind(TokenKind::Comma) => {
                    self.skip_token();
                }
                Some(token) if token.has_kind(TokenKind::Gt) => {
                    self.skip_token();
                    arg_span = arg_span.join(self.current_location());

                    break;
                }
                Some(token) => self.error(
                    AstGenErrorKind::Expected,
                    Some(TokenKindVector::from_row(vec![TokenKind::Comma, TokenKind::Gt])),
                    Some(token.kind),
                )?,
                None => self.unexpected_eof()?,
            }
        }

        // Now pass the return type
        self.parse_thin_arrow()?;
        let return_ty = self.parse_type()?;

        Ok(Ty::TyFn(TyFn { params: AstNodes::new(args, Some(arg_span)), return_ty }))
    }
}
