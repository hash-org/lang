//! Visitor implementation for [crate::ast] nodes.
//!
//! All rights reserved 2021 (c) The Hash Language authors
use crate::ast;
use std::convert::Infallible;

/// The main visitor trait for [crate::ast] nodes.
///
/// This contains a method for each AST structure, as well as a dedicated return type for it.
/// These can be implemented using the functions defined in [walk] that can traverse the children
/// of each node.
pub trait AstVisitor<'c>: Sized {
    /// Context type immutably passed to each visitor method for separating mutable from immutable context.
    type Ctx: 'c;

    /// What container to use to collect multiple children, used by [walk].
    type CollectionContainer<T: 'c>: Sized + 'c;

    /// Try collect an iterator of results into a container specified by [Self::CollectionContainer].
    fn try_collect_items<T: 'c, E, I: Iterator<Item = Result<T, E>>>(
        ctx: &Self::Ctx,
        items: I,
    ) -> Result<Self::CollectionContainer<T>, E>;

    /// Collect an iterator of items into a container specified by [Self::CollectionContainer].
    fn collect_items<T: 'c, E, I: Iterator<Item = T>>(
        ctx: &Self::Ctx,
        items: I,
    ) -> Self::CollectionContainer<T> {
        Self::try_collect_items::<T, Infallible, _>(ctx, items.map(|item| Ok(item))).unwrap()
    }

    /// The error type to use for each visit method.
    type Error: 'c;

    type ImportRet: 'c;
    fn visit_import(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Import>,
    ) -> Result<Self::ImportRet, Self::Error>;

    type NameRet: 'c;
    fn visit_name(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Name>,
    ) -> Result<Self::NameRet, Self::Error>;

    type AccessNameRet: 'c;
    fn visit_access_name(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AccessName<'c>>,
    ) -> Result<Self::AccessNameRet, Self::Error>;

    type LiteralRet: 'c;
    fn visit_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Literal<'c>>,
    ) -> Result<Self::LiteralRet, Self::Error>;

    type ExpressionRet: 'c;
    fn visit_expression(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Expression<'c>>,
    ) -> Result<Self::ExpressionRet, Self::Error>;

    type VariableExprRet: 'c;
    fn visit_variable_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::VariableExpr<'c>>,
    ) -> Result<Self::VariableExprRet, Self::Error>;

    type IntrinsicKeyRet: 'c;
    fn visit_intrinsic_key(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IntrinsicKey>,
    ) -> Result<Self::IntrinsicKeyRet, Self::Error>;

    type FunctionCallArgsRet: 'c;
    fn visit_function_call_args(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FunctionCallArgs<'c>>,
    ) -> Result<Self::FunctionCallArgsRet, Self::Error>;

    type FunctionCallExprRet: 'c;
    fn visit_function_call_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FunctionCallExpr<'c>>,
    ) -> Result<Self::FunctionCallExprRet, Self::Error>;

    type PropertyAccessExprRet: 'c;
    fn visit_property_access_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::PropertyAccessExpr<'c>>,
    ) -> Result<Self::PropertyAccessExprRet, Self::Error>;

    type RefExprRet: 'c;
    fn visit_ref_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::RefExpr<'c>>,
    ) -> Result<Self::RefExprRet, Self::Error>;

    type DerefExprRet: 'c;
    fn visit_deref_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::DerefExpr<'c>>,
    ) -> Result<Self::DerefExprRet, Self::Error>;

    type LiteralExprRet: 'c;
    fn visit_literal_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LiteralExpr<'c>>,
    ) -> Result<Self::LiteralExprRet, Self::Error>;

    type TypedExprRet: 'c;
    fn visit_typed_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TypedExpr<'c>>,
    ) -> Result<Self::TypedExprRet, Self::Error>;

    type BlockExprRet: 'c;
    fn visit_block_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BlockExpr<'c>>,
    ) -> Result<Self::BlockExprRet, Self::Error>;

    type ImportExprRet: 'c;
    fn visit_import_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ImportExpr<'c>>,
    ) -> Result<Self::ImportExprRet, Self::Error>;

    type TypeRet: 'c;
    fn visit_type(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Type<'c>>,
    ) -> Result<Self::TypeRet, Self::Error>;

    type NamedTypeRet: 'c;
    fn visit_named_type(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::NamedType<'c>>,
    ) -> Result<Self::NamedTypeRet, Self::Error>;

    type RefTypeRet: 'c;
    fn visit_ref_type(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::RefType<'c>>,
    ) -> Result<Self::RefTypeRet, Self::Error>;

    type RawRefTypeRet: 'c;
    fn visit_raw_ref_type(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::RawRefType<'c>>,
    ) -> Result<Self::RawRefTypeRet, Self::Error>;

    type TypeVarRet: 'c;
    fn visit_type_var(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TypeVar<'c>>,
    ) -> Result<Self::TypeVarRet, Self::Error>;

    type ExistentialTypeRet: 'c;
    fn visit_existential_type(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ExistentialType>,
    ) -> Result<Self::ExistentialTypeRet, Self::Error>;

    type InferTypeRet: 'c;
    fn visit_infer_type(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::InferType>,
    ) -> Result<Self::InferTypeRet, Self::Error>;

    type MapLiteralRet: 'c;
    fn visit_map_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MapLiteral<'c>>,
    ) -> Result<Self::MapLiteralRet, Self::Error>;

    type MapLiteralEntryRet: 'c;
    fn visit_map_literal_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MapLiteralEntry<'c>>,
    ) -> Result<Self::MapLiteralEntryRet, Self::Error>;

    type ListLiteralRet: 'c;
    fn visit_list_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ListLiteral<'c>>,
    ) -> Result<Self::ListLiteralRet, Self::Error>;

    type SetLiteralRet: 'c;
    fn visit_set_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::SetLiteral<'c>>,
    ) -> Result<Self::SetLiteralRet, Self::Error>;

    type TupleLiteralRet: 'c;
    fn visit_tuple_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TupleLiteral<'c>>,
    ) -> Result<Self::TupleLiteralRet, Self::Error>;

    type StrLiteralRet: 'c;
    fn visit_str_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StrLiteral>,
    ) -> Result<Self::StrLiteralRet, Self::Error>;

    type CharLiteralRet: 'c;
    fn visit_char_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::CharLiteral>,
    ) -> Result<Self::CharLiteralRet, Self::Error>;

    type FloatLiteralRet: 'c;
    fn visit_float_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FloatLiteral>,
    ) -> Result<Self::FloatLiteralRet, Self::Error>;

    type IntLiteralRet: 'c;
    fn visit_int_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IntLiteral>,
    ) -> Result<Self::IntLiteralRet, Self::Error>;

    type StructLiteralRet: 'c;
    fn visit_struct_literal(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StructLiteral<'c>>,
    ) -> Result<Self::StructLiteralRet, Self::Error>;

    type StructLiteralEntryRet: 'c;
    fn visit_struct_literal_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StructLiteralEntry<'c>>,
    ) -> Result<Self::StructLiteralEntryRet, Self::Error>;

    type FunctionDefRet: 'c;
    fn visit_function_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FunctionDef<'c>>,
    ) -> Result<Self::FunctionDefRet, Self::Error>;

    type FunctionDefArgRet: 'c;
    fn visit_function_def_arg(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FunctionDefArg<'c>>,
    ) -> Result<Self::FunctionDefArgRet, Self::Error>;

    type BlockRet: 'c;
    fn visit_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Block<'c>>,
    ) -> Result<Self::BlockRet, Self::Error>;

    type MatchCaseRet: 'c;
    fn visit_match_case(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MatchCase<'c>>,
    ) -> Result<Self::MatchCaseRet, Self::Error>;

    type MatchBlockRet: 'c;
    fn visit_match_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MatchBlock<'c>>,
    ) -> Result<Self::MatchBlockRet, Self::Error>;

    type LoopBlockRet: 'c;
    fn visit_loop_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LoopBlock<'c>>,
    ) -> Result<Self::LoopBlockRet, Self::Error>;

    type BodyBlockRet: 'c;
    fn visit_body_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BodyBlock<'c>>,
    ) -> Result<Self::BodyBlockRet, Self::Error>;

    type StatementRet: 'c;
    fn visit_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Statement<'c>>,
    ) -> Result<Self::StatementRet, Self::Error>;

    type ExprStatementRet: 'c;
    fn visit_expr_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ExprStatement<'c>>,
    ) -> Result<Self::ExprStatementRet, Self::Error>;

    type ReturnStatementRet: 'c;
    fn visit_return_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ReturnStatement<'c>>,
    ) -> Result<Self::ReturnStatementRet, Self::Error>;

    type BlockStatementRet: 'c;
    fn visit_block_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BlockStatement<'c>>,
    ) -> Result<Self::BlockStatementRet, Self::Error>;

    type BreakStatementRet: 'c;
    fn visit_break_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BreakStatement>,
    ) -> Result<Self::BreakStatementRet, Self::Error>;

    type ContinueStatementRet: 'c;
    fn visit_continue_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ContinueStatement>,
    ) -> Result<Self::ContinueStatementRet, Self::Error>;

    type LetStatementRet: 'c;
    fn visit_let_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LetStatement<'c>>,
    ) -> Result<Self::LetStatementRet, Self::Error>;

    type AssignStatementRet: 'c;
    fn visit_assign_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AssignStatement<'c>>,
    ) -> Result<Self::AssignStatementRet, Self::Error>;

    type StructDefEntryRet: 'c;
    fn visit_struct_def_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StructDefEntry<'c>>,
    ) -> Result<Self::StructDefEntryRet, Self::Error>;

    type StructDefRet: 'c;
    fn visit_struct_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StructDef<'c>>,
    ) -> Result<Self::StructDefRet, Self::Error>;

    type EnumDefEntryRet: 'c;
    fn visit_enum_def_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::EnumDefEntry<'c>>,
    ) -> Result<Self::EnumDefEntryRet, Self::Error>;

    type EnumDefRet: 'c;
    fn visit_enum_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::EnumDef<'c>>,
    ) -> Result<Self::EnumDefRet, Self::Error>;

    type TraitDefRet: 'c;
    fn visit_trait_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TraitDef<'c>>,
    ) -> Result<Self::TraitDefRet, Self::Error>;

    type PatternRet: 'c;
    fn visit_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Pattern<'c>>,
    ) -> Result<Self::PatternRet, Self::Error>;

    type TraitBoundRet: 'c;
    fn visit_trait_bound(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TraitBound<'c>>,
    ) -> Result<Self::TraitBoundRet, Self::Error>;

    type BoundRet: 'c;
    fn visit_bound(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Bound<'c>>,
    ) -> Result<Self::BoundRet, Self::Error>;

    type EnumPatternRet: 'c;
    fn visit_enum_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::EnumPattern<'c>>,
    ) -> Result<Self::EnumPatternRet, Self::Error>;

    type StructPatternRet: 'c;
    fn visit_struct_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StructPattern<'c>>,
    ) -> Result<Self::StructPatternRet, Self::Error>;

    type NamespacePatternRet: 'c;
    fn visit_namespace_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::NamespacePattern<'c>>,
    ) -> Result<Self::NamespacePatternRet, Self::Error>;

    type TuplePatternRet: 'c;
    fn visit_tuple_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TuplePattern<'c>>,
    ) -> Result<Self::TuplePatternRet, Self::Error>;

    type StrLiteralPatternRet: 'c;
    fn visit_str_literal_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StrLiteralPattern>,
    ) -> Result<Self::StrLiteralPatternRet, Self::Error>;

    type CharLiteralPatternRet: 'c;
    fn visit_char_literal_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::CharLiteralPattern>,
    ) -> Result<Self::CharLiteralPatternRet, Self::Error>;

    type IntLiteralPatternRet: 'c;
    fn visit_int_literal_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IntLiteralPattern>,
    ) -> Result<Self::IntLiteralPatternRet, Self::Error>;

    type FloatLiteralPatternRet: 'c;
    fn visit_float_literal_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FloatLiteralPattern>,
    ) -> Result<Self::FloatLiteralPatternRet, Self::Error>;

    type LiteralPatternRet: 'c;
    fn visit_literal_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LiteralPattern>,
    ) -> Result<Self::LiteralPatternRet, Self::Error>;

    type OrPatternRet: 'c;
    fn visit_or_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::OrPattern<'c>>,
    ) -> Result<Self::OrPatternRet, Self::Error>;

    type IfPatternRet: 'c;
    fn visit_if_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IfPattern<'c>>,
    ) -> Result<Self::IfPatternRet, Self::Error>;

    type BindingPatternRet: 'c;
    fn visit_binding_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BindingPattern<'c>>,
    ) -> Result<Self::BindingPatternRet, Self::Error>;

    type IgnorePatternRet: 'c;
    fn visit_ignore_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IgnorePattern>,
    ) -> Result<Self::IgnorePatternRet, Self::Error>;

    type DestructuringPatternRet: 'c;
    fn visit_destructuring_pattern(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::DestructuringPattern<'c>>,
    ) -> Result<Self::DestructuringPatternRet, Self::Error>;

    type ModuleRet: 'c;
    fn visit_module(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Module<'c>>,
    ) -> Result<Self::ModuleRet, Self::Error>;
}

/// Contains helper functions and structures to traverse AST nodes using a given visitor.
///
/// Structures are defined which mirror the layout of the AST nodes, but instead of having AST
/// nodes as children, they have the [AstVisitor] output type for each node.
///
/// For enums, there is an additional `*_same_children` function, which traverses the member of
/// each variant and returns the inner type, given that all variants have the same declared type
/// within the visitor.
pub mod walk {
    use super::ast;
    use super::AstVisitor;

    pub struct FunctionDefArg<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub ty: Option<V::TypeRet>,
    }

    pub fn walk_function_def_arg<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::FunctionDefArg<'c>>,
    ) -> Result<FunctionDefArg<'c, V>, V::Error> {
        Ok(FunctionDefArg {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            ty: node
                .ty
                .as_ref()
                .map(|t| visitor.visit_type(ctx, t.ast_ref()))
                .transpose()?,
        })
    }

    pub struct FunctionDef<'c, V: AstVisitor<'c>> {
        pub args: V::CollectionContainer<V::FunctionDefArgRet>,
        pub return_ty: Option<V::TypeRet>,
        pub fn_body: V::ExpressionRet,
    }

    pub fn walk_function_def<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::FunctionDef<'c>>,
    ) -> Result<FunctionDef<'c, V>, V::Error> {
        Ok(FunctionDef {
            args: V::try_collect_items(
                ctx,
                node.args
                    .iter()
                    .map(|a| visitor.visit_function_def_arg(ctx, a.ast_ref())),
            )?,
            return_ty: node
                .return_ty
                .as_ref()
                .map(|t| visitor.visit_type(ctx, t.ast_ref()))
                .transpose()?,
            fn_body: visitor.visit_expression(ctx, node.fn_body.ast_ref())?,
        })
    }

    pub struct StructLiteral<'c, V: AstVisitor<'c>> {
        pub name: V::AccessNameRet,
        pub type_args: V::CollectionContainer<V::TypeRet>,
        pub entries: V::CollectionContainer<V::StructLiteralEntryRet>,
    }

    pub fn walk_struct_literal<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::StructLiteral<'c>>,
    ) -> Result<StructLiteral<'c, V>, V::Error> {
        Ok(StructLiteral {
            name: visitor.visit_access_name(ctx, node.name.ast_ref())?,
            type_args: V::try_collect_items(
                ctx,
                node.type_args
                    .iter()
                    .map(|a| visitor.visit_type(ctx, a.ast_ref())),
            )?,
            entries: V::try_collect_items(
                ctx,
                node.entries
                    .iter()
                    .map(|e| visitor.visit_struct_literal_entry(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct StructLiteralEntry<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub value: V::ExpressionRet,
    }

    pub fn walk_struct_literal_entry<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::StructLiteralEntry<'c>>,
    ) -> Result<StructLiteralEntry<'c, V>, V::Error> {
        Ok(StructLiteralEntry {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            value: visitor.visit_expression(ctx, node.value.ast_ref())?,
        })
    }

    pub enum Expression<'c, V: AstVisitor<'c>> {
        FunctionCall(V::FunctionCallExprRet),
        Intrinsic(V::IntrinsicKeyRet),
        Variable(V::VariableExprRet),
        PropertyAccess(V::PropertyAccessExprRet),
        Ref(V::RefExprRet),
        Deref(V::DerefExprRet),
        LiteralExpr(V::LiteralExprRet),
        Typed(V::TypedExprRet),
        Block(V::BlockExprRet),
        Import(V::ImportExprRet),
    }

    pub fn walk_expression<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Expression<'c>>,
    ) -> Result<Expression<'c, V>, V::Error> {
        Ok(match node.kind() {
            ast::ExpressionKind::FunctionCall(inner) => Expression::FunctionCall(
                visitor.visit_function_call_expr(ctx, node.with_body(inner))?,
            ),
            ast::ExpressionKind::Intrinsic(inner) => {
                Expression::Intrinsic(visitor.visit_intrinsic_key(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::Variable(inner) => {
                Expression::Variable(visitor.visit_variable_expr(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::PropertyAccess(inner) => Expression::PropertyAccess({
                visitor.visit_property_access_expr(ctx, node.with_body(inner))?
            }),
            ast::ExpressionKind::Ref(inner) => {
                Expression::Ref(visitor.visit_ref_expr(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::Deref(inner) => {
                Expression::Deref(visitor.visit_deref_expr(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::LiteralExpr(inner) => {
                Expression::LiteralExpr(visitor.visit_literal_expr(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::Typed(inner) => {
                Expression::Typed(visitor.visit_typed_expr(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::Block(inner) => {
                Expression::Block(visitor.visit_block_expr(ctx, node.with_body(inner))?)
            }
            ast::ExpressionKind::Import(inner) => {
                Expression::Import(visitor.visit_import_expr(ctx, node.with_body(inner))?)
            }
        })
    }

    pub fn walk_expression_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Expression<'c>>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<
            'c,
            FunctionCallExprRet = Ret,
            IntrinsicKeyRet = Ret,
            VariableExprRet = Ret,
            PropertyAccessExprRet = Ret,
            RefExprRet = Ret,
            DerefExprRet = Ret,
            LiteralExprRet = Ret,
            TypedExprRet = Ret,
            BlockExprRet = Ret,
            ImportExprRet = Ret,
        >,
    {
        Ok(match walk_expression(visitor, ctx, node)? {
            Expression::FunctionCall(r) => r,
            Expression::Intrinsic(r) => r,
            Expression::Variable(r) => r,
            Expression::PropertyAccess(r) => r,
            Expression::Ref(r) => r,
            Expression::Deref(r) => r,
            Expression::LiteralExpr(r) => r,
            Expression::Typed(r) => r,
            Expression::Block(r) => r,
            Expression::Import(r) => r,
        })
    }

    pub struct VariableExpr<'c, V: AstVisitor<'c>> {
        pub name: V::AccessNameRet,
        pub type_args: V::CollectionContainer<V::TypeRet>,
    }

    pub fn walk_variable_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::VariableExpr<'c>>,
    ) -> Result<VariableExpr<'c, V>, V::Error> {
        Ok(VariableExpr {
            name: visitor.visit_access_name(ctx, node.name.ast_ref())?,
            type_args: V::try_collect_items(
                ctx,
                node.type_args
                    .iter()
                    .map(|t| visitor.visit_type(ctx, t.ast_ref())),
            )?,
        })
    }

    pub struct FunctionCallArgs<'c, V: AstVisitor<'c>> {
        pub entries: V::CollectionContainer<V::ExpressionRet>,
    }

    pub fn walk_function_call_args<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::FunctionCallArgs<'c>>,
    ) -> Result<FunctionCallArgs<'c, V>, V::Error> {
        Ok(FunctionCallArgs {
            entries: V::try_collect_items(
                ctx,
                node.entries
                    .iter()
                    .map(|e| visitor.visit_expression(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct FunctionCallExpr<'c, V: AstVisitor<'c>> {
        pub subject: V::ExpressionRet,
        pub args: V::FunctionCallArgsRet,
    }

    pub fn walk_function_call_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::FunctionCallExpr<'c>>,
    ) -> Result<FunctionCallExpr<'c, V>, V::Error> {
        Ok(FunctionCallExpr {
            subject: visitor.visit_expression(ctx, node.subject.ast_ref())?,
            args: visitor.visit_function_call_args(ctx, node.args.ast_ref())?,
        })
    }

    pub struct PropertyAccessExpr<'c, V: AstVisitor<'c>> {
        pub subject: V::ExpressionRet,
        pub property: V::NameRet,
    }

    pub fn walk_property_access_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::PropertyAccessExpr<'c>>,
    ) -> Result<PropertyAccessExpr<'c, V>, V::Error> {
        Ok(PropertyAccessExpr {
            subject: visitor.visit_expression(ctx, node.subject.ast_ref())?,
            property: visitor.visit_name(ctx, node.property.ast_ref())?,
        })
    }

    pub struct RefExpr<'c, V: AstVisitor<'c>> {
        pub inner_expr: V::ExpressionRet,
    }

    pub fn walk_ref_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::RefExpr<'c>>,
    ) -> Result<RefExpr<'c, V>, V::Error> {
        Ok(RefExpr {
            inner_expr: visitor.visit_expression(ctx, node.inner_expr.ast_ref())?,
        })
    }

    pub struct DerefExpr<'c, V: AstVisitor<'c>>(pub V::ExpressionRet);

    pub fn walk_deref_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::DerefExpr<'c>>,
    ) -> Result<DerefExpr<'c, V>, V::Error> {
        Ok(DerefExpr(visitor.visit_expression(ctx, node.0.ast_ref())?))
    }

    pub struct LiteralExpr<'c, V: AstVisitor<'c>>(pub V::LiteralRet);

    pub fn walk_literal_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::LiteralExpr<'c>>,
    ) -> Result<LiteralExpr<'c, V>, V::Error> {
        Ok(LiteralExpr(visitor.visit_literal(ctx, node.0.ast_ref())?))
    }

    pub struct TypedExpr<'c, V: AstVisitor<'c>> {
        pub ty: V::TypeRet,
        pub expr: V::ExpressionRet,
    }

    pub fn walk_typed_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::TypedExpr<'c>>,
    ) -> Result<TypedExpr<'c, V>, V::Error> {
        Ok(TypedExpr {
            ty: visitor.visit_type(ctx, node.ty.ast_ref())?,
            expr: visitor.visit_expression(ctx, node.expr.ast_ref())?,
        })
    }

    pub struct BlockExpr<'c, V: AstVisitor<'c>>(pub V::BlockRet);

    pub fn walk_block_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::BlockExpr<'c>>,
    ) -> Result<BlockExpr<'c, V>, V::Error> {
        Ok(BlockExpr(visitor.visit_block(ctx, node.0.ast_ref())?))
    }

    pub struct ImportExpr<'c, V: AstVisitor<'c>>(pub V::ImportRet);

    pub fn walk_import_expr<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::ImportExpr<'c>>,
    ) -> Result<ImportExpr<'c, V>, V::Error> {
        Ok(ImportExpr(visitor.visit_import(ctx, node.0.ast_ref())?))
    }

    pub enum Literal<'c, V: AstVisitor<'c>> {
        Str(V::StrLiteralRet),
        Char(V::CharLiteralRet),
        Int(V::IntLiteralRet),
        Float(V::FloatLiteralRet),
        Set(V::SetLiteralRet),
        Map(V::MapLiteralRet),
        List(V::ListLiteralRet),
        Tuple(V::TupleLiteralRet),
        Struct(V::StructLiteralRet),
        Function(V::FunctionDefRet),
    }

    pub fn walk_literal<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Literal<'c>>,
    ) -> Result<Literal<'c, V>, V::Error> {
        Ok(match &*node {
            ast::Literal::Str(r) => {
                Literal::Str(visitor.visit_str_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Char(r) => {
                Literal::Char(visitor.visit_char_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Int(r) => {
                Literal::Int(visitor.visit_int_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Float(r) => {
                Literal::Float(visitor.visit_float_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Set(r) => {
                Literal::Set(visitor.visit_set_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Map(r) => {
                Literal::Map(visitor.visit_map_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::List(r) => {
                Literal::List(visitor.visit_list_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Tuple(r) => {
                Literal::Tuple(visitor.visit_tuple_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Struct(r) => {
                Literal::Struct(visitor.visit_struct_literal(ctx, node.with_body(r))?)
            }
            ast::Literal::Function(r) => {
                Literal::Function(visitor.visit_function_def(ctx, node.with_body(r))?)
            }
        })
    }

    pub fn walk_literal_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Literal<'c>>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<
            'c,
            StrLiteralRet = Ret,
            CharLiteralRet = Ret,
            IntLiteralRet = Ret,
            FloatLiteralRet = Ret,
            SetLiteralRet = Ret,
            MapLiteralRet = Ret,
            ListLiteralRet = Ret,
            TupleLiteralRet = Ret,
            StructLiteralRet = Ret,
            FunctionDefRet = Ret,
        >,
    {
        Ok(match walk_literal(visitor, ctx, node)? {
            Literal::Str(r) => r,
            Literal::Char(r) => r,
            Literal::Int(r) => r,
            Literal::Float(r) => r,
            Literal::Set(r) => r,
            Literal::Map(r) => r,
            Literal::List(r) => r,
            Literal::Tuple(r) => r,
            Literal::Struct(r) => r,
            Literal::Function(r) => r,
        })
    }

    pub struct MatchCase<'c, V: AstVisitor<'c>> {
        pub pattern: V::PatternRet,
        pub expr: V::ExpressionRet,
    }

    pub fn walk_match_case<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::MatchCase<'c>>,
    ) -> Result<MatchCase<'c, V>, V::Error> {
        Ok(MatchCase {
            pattern: visitor.visit_pattern(ctx, node.pattern.ast_ref())?,
            expr: visitor.visit_expression(ctx, node.expr.ast_ref())?,
        })
    }

    pub struct MatchBlock<'c, V: AstVisitor<'c>> {
        pub subject: V::ExpressionRet,
        pub cases: V::CollectionContainer<V::MatchCaseRet>,
    }

    pub fn walk_match_block<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::MatchBlock<'c>>,
    ) -> Result<MatchBlock<'c, V>, V::Error> {
        Ok(MatchBlock {
            subject: visitor.visit_expression(ctx, node.subject.ast_ref())?,
            cases: V::try_collect_items(
                ctx,
                node.cases
                    .iter()
                    .map(|c| visitor.visit_match_case(ctx, c.ast_ref())),
            )?,
        })
    }

    pub struct LoopBlock<'c, V: AstVisitor<'c>>(pub V::BlockRet);

    pub fn walk_loop_block<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::LoopBlock<'c>>,
    ) -> Result<LoopBlock<'c, V>, V::Error> {
        Ok(LoopBlock(visitor.visit_block(ctx, node.0.ast_ref())?))
    }

    pub struct BodyBlock<'c, V: AstVisitor<'c>> {
        pub statements: V::CollectionContainer<V::StatementRet>,
        pub expr: Option<V::ExpressionRet>,
    }

    pub fn walk_body_block<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::BodyBlock<'c>>,
    ) -> Result<BodyBlock<'c, V>, V::Error> {
        Ok(BodyBlock {
            statements: V::try_collect_items(
                ctx,
                node.statements
                    .iter()
                    .map(|s| visitor.visit_statement(ctx, s.ast_ref())),
            )?,
            expr: node
                .expr
                .as_ref()
                .map(|e| visitor.visit_expression(ctx, e.ast_ref()))
                .transpose()?,
        })
    }

    pub enum Block<'c, V: AstVisitor<'c>> {
        Match(V::MatchBlockRet),
        Loop(V::LoopBlockRet),
        Body(V::BodyBlockRet),
    }

    pub fn walk_block<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Block<'c>>,
    ) -> Result<Block<'c, V>, V::Error> {
        Ok(match &*node {
            ast::Block::Match(r) => {
                Block::Match(visitor.visit_match_block(ctx, node.with_body(r))?)
            }
            ast::Block::Loop(r) => Block::Loop(visitor.visit_loop_block(ctx, node.with_body(r))?),
            ast::Block::Body(r) => Block::Body(visitor.visit_body_block(ctx, node.with_body(r))?),
        })
    }

    pub fn walk_block_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Block<'c>>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<'c, MatchBlockRet = Ret, LoopBlockRet = Ret, BodyBlockRet = Ret>,
    {
        Ok(match walk_block(visitor, ctx, node)? {
            Block::Match(r) => r,
            Block::Loop(r) => r,
            Block::Body(r) => r,
        })
    }

    pub struct SetLiteral<'c, V: AstVisitor<'c>> {
        pub elements: V::CollectionContainer<V::ExpressionRet>,
    }

    pub fn walk_set_literal<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::SetLiteral<'c>>,
    ) -> Result<SetLiteral<'c, V>, V::Error> {
        Ok(SetLiteral {
            elements: V::try_collect_items(
                ctx,
                node.elements
                    .iter()
                    .map(|e| visitor.visit_expression(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct MapLiteralEntry<'c, V: AstVisitor<'c>> {
        pub key: V::ExpressionRet,
        pub value: V::ExpressionRet,
    }

    pub fn walk_map_literal_entry<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::MapLiteralEntry<'c>>,
    ) -> Result<MapLiteralEntry<'c, V>, V::Error> {
        Ok(MapLiteralEntry {
            key: visitor.visit_expression(ctx, node.key.ast_ref())?,
            value: visitor.visit_expression(ctx, node.value.ast_ref())?,
        })
    }

    pub struct MapLiteral<'c, V: AstVisitor<'c>> {
        pub entries: V::CollectionContainer<V::MapLiteralEntryRet>,
    }

    pub fn walk_map_literal<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::MapLiteral<'c>>,
    ) -> Result<MapLiteral<'c, V>, V::Error> {
        Ok(MapLiteral {
            entries: V::try_collect_items(
                ctx,
                node.elements
                    .iter()
                    .map(|e| visitor.visit_map_literal_entry(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct ListLiteral<'c, V: AstVisitor<'c>> {
        pub elements: V::CollectionContainer<V::ExpressionRet>,
    }

    pub fn walk_list_literal<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::ListLiteral<'c>>,
    ) -> Result<ListLiteral<'c, V>, V::Error> {
        Ok(ListLiteral {
            elements: V::try_collect_items(
                ctx,
                node.elements
                    .iter()
                    .map(|e| visitor.visit_expression(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct TupleLiteral<'c, V: AstVisitor<'c>> {
        pub elements: V::CollectionContainer<V::ExpressionRet>,
    }

    pub fn walk_tuple_literal<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::TupleLiteral<'c>>,
    ) -> Result<TupleLiteral<'c, V>, V::Error> {
        Ok(TupleLiteral {
            elements: V::try_collect_items(
                ctx,
                node.elements
                    .iter()
                    .map(|e| visitor.visit_expression(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct NamedType<'c, V: AstVisitor<'c>> {
        pub name: V::AccessNameRet,
        pub type_args: V::CollectionContainer<V::TypeRet>,
    }

    pub fn walk_named_type<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::NamedType<'c>>,
    ) -> Result<NamedType<'c, V>, V::Error> {
        Ok(NamedType {
            name: visitor.visit_access_name(ctx, node.name.ast_ref())?,
            type_args: V::try_collect_items(
                ctx,
                node.type_args
                    .iter()
                    .map(|e| visitor.visit_type(ctx, e.ast_ref())),
            )?,
        })
    }

    pub struct RefType<'c, V: AstVisitor<'c>>(pub V::TypeRet);

    pub fn walk_ref_type<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::RefType<'c>>,
    ) -> Result<RefType<'c, V>, V::Error> {
        Ok(RefType(visitor.visit_type(ctx, node.0.ast_ref())?))
    }

    pub struct RawRefType<'c, V: AstVisitor<'c>>(pub V::TypeRet);

    pub fn walk_raw_ref_type<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::RawRefType<'c>>,
    ) -> Result<RawRefType<'c, V>, V::Error> {
        Ok(RawRefType(visitor.visit_type(ctx, node.0.ast_ref())?))
    }

    pub struct TypeVar<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
    }

    pub fn walk_type_var<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::TypeVar<'c>>,
    ) -> Result<TypeVar<'c, V>, V::Error> {
        Ok(TypeVar {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
        })
    }

    pub enum Type<'c, V: AstVisitor<'c>> {
        Named(V::NamedTypeRet),
        Ref(V::RefTypeRet),
        RawRef(V::RawRefTypeRet),
        TypeVar(V::TypeVarRet),
        Existential(V::ExistentialTypeRet),
        Infer(V::InferTypeRet),
    }

    pub fn walk_type<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Type<'c>>,
    ) -> Result<Type<'c, V>, V::Error> {
        Ok(match &*node {
            ast::Type::Named(r) => Type::Named(visitor.visit_named_type(ctx, node.with_body(r))?),
            ast::Type::Ref(r) => Type::Ref(visitor.visit_ref_type(ctx, node.with_body(r))?),
            ast::Type::RawRef(r) => {
                Type::RawRef(visitor.visit_raw_ref_type(ctx, node.with_body(r))?)
            }
            ast::Type::TypeVar(r) => Type::TypeVar(visitor.visit_type_var(ctx, node.with_body(r))?),
            ast::Type::Existential(r) => {
                Type::Existential(visitor.visit_existential_type(ctx, node.with_body(r))?)
            }
            ast::Type::Infer(r) => Type::Infer(visitor.visit_infer_type(ctx, node.with_body(r))?),
        })
    }

    pub fn walk_type_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Type<'c>>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<
            'c,
            NamedTypeRet = Ret,
            RefTypeRet = Ret,
            RawRefTypeRet = Ret,
            TypeVarRet = Ret,
            ExistentialTypeRet = Ret,
            InferTypeRet = Ret,
        >,
    {
        Ok(match walk_type(visitor, ctx, node)? {
            Type::Named(r) => r,
            Type::Ref(r) => r,
            Type::RawRef(r) => r,
            Type::TypeVar(r) => r,
            Type::Existential(r) => r,
            Type::Infer(r) => r,
        })
    }

    pub enum Pattern<'c, V: AstVisitor<'c>> {
        Enum(V::EnumPatternRet),
        Struct(V::StructPatternRet),
        Namespace(V::NamespacePatternRet),
        Tuple(V::TuplePatternRet),
        Literal(V::LiteralPatternRet),
        Or(V::OrPatternRet),
        If(V::IfPatternRet),
        Binding(V::BindingPatternRet),
        Ignore(V::IgnorePatternRet),
    }

    pub fn walk_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Pattern<'c>>,
    ) -> Result<Pattern<'c, V>, V::Error> {
        Ok(match &*node {
            ast::Pattern::Enum(r) => {
                Pattern::Enum(visitor.visit_enum_pattern(ctx, node.with_body(r))?)
            }
            ast::Pattern::Struct(r) => {
                Pattern::Struct(visitor.visit_struct_pattern(ctx, node.with_body(r))?)
            }
            ast::Pattern::Namespace(r) => {
                Pattern::Namespace(visitor.visit_namespace_pattern(ctx, node.with_body(r))?)
            }
            ast::Pattern::Tuple(r) => {
                Pattern::Tuple(visitor.visit_tuple_pattern(ctx, node.with_body(r))?)
            }
            ast::Pattern::Literal(r) => {
                Pattern::Literal(visitor.visit_literal_pattern(ctx, node.with_body(r))?)
            }
            ast::Pattern::Or(r) => Pattern::Or(visitor.visit_or_pattern(ctx, node.with_body(r))?),
            ast::Pattern::If(r) => Pattern::If(visitor.visit_if_pattern(ctx, node.with_body(r))?),
            ast::Pattern::Binding(r) => {
                Pattern::Binding(visitor.visit_binding_pattern(ctx, node.with_body(r))?)
            }
            ast::Pattern::Ignore(r) => {
                Pattern::Ignore(visitor.visit_ignore_pattern(ctx, node.with_body(r))?)
            }
        })
    }

    pub fn walk_pattern_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Pattern<'c>>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<
            'c,
            EnumPatternRet = Ret,
            StructPatternRet = Ret,
            NamespacePatternRet = Ret,
            TuplePatternRet = Ret,
            LiteralPatternRet = Ret,
            OrPatternRet = Ret,
            IfPatternRet = Ret,
            BindingPatternRet = Ret,
            IgnorePatternRet = Ret,
        >,
    {
        Ok(match walk_pattern(visitor, ctx, node)? {
            Pattern::Enum(r) => r,
            Pattern::Struct(r) => r,
            Pattern::Namespace(r) => r,
            Pattern::Tuple(r) => r,
            Pattern::Literal(r) => r,
            Pattern::Or(r) => r,
            Pattern::If(r) => r,
            Pattern::Binding(r) => r,
            Pattern::Ignore(r) => r,
        })
    }

    pub struct OrPattern<'c, V: AstVisitor<'c>> {
        pub variants: V::CollectionContainer<V::PatternRet>,
    }
    pub fn walk_or_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::OrPattern<'c>>,
    ) -> Result<OrPattern<'c, V>, V::Error> {
        Ok(OrPattern {
            variants: V::try_collect_items(
                ctx,
                node.variants
                    .iter()
                    .map(|v| visitor.visit_pattern(ctx, v.ast_ref())),
            )?,
        })
    }

    pub struct EnumPattern<'c, V: AstVisitor<'c>> {
        pub name: V::AccessNameRet,
        pub args: V::CollectionContainer<V::PatternRet>,
    }
    pub fn walk_enum_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::EnumPattern<'c>>,
    ) -> Result<EnumPattern<'c, V>, V::Error> {
        Ok(EnumPattern {
            name: visitor.visit_access_name(ctx, node.name.ast_ref())?,
            args: V::try_collect_items(
                ctx,
                node.args
                    .iter()
                    .map(|a| visitor.visit_pattern(ctx, a.ast_ref())),
            )?,
        })
    }

    pub struct StructPattern<'c, V: AstVisitor<'c>> {
        pub name: V::AccessNameRet,
        pub entries: V::CollectionContainer<V::DestructuringPatternRet>,
    }
    pub fn walk_struct_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::StructPattern<'c>>,
    ) -> Result<StructPattern<'c, V>, V::Error> {
        Ok(StructPattern {
            name: visitor.visit_access_name(ctx, node.name.ast_ref())?,
            entries: V::try_collect_items(
                ctx,
                node.entries
                    .iter()
                    .map(|a| visitor.visit_destructuring_pattern(ctx, a.ast_ref())),
            )?,
        })
    }

    pub struct NamespacePattern<'c, V: AstVisitor<'c>> {
        pub patterns: V::CollectionContainer<V::DestructuringPatternRet>,
    }
    pub fn walk_namespace_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::NamespacePattern<'c>>,
    ) -> Result<NamespacePattern<'c, V>, V::Error> {
        Ok(NamespacePattern {
            patterns: V::try_collect_items(
                ctx,
                node.patterns
                    .iter()
                    .map(|a| visitor.visit_destructuring_pattern(ctx, a.ast_ref())),
            )?,
        })
    }

    pub struct TuplePattern<'c, V: AstVisitor<'c>> {
        pub elements: V::CollectionContainer<V::PatternRet>,
    }
    pub fn walk_tuple_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::TuplePattern<'c>>,
    ) -> Result<TuplePattern<'c, V>, V::Error> {
        Ok(TuplePattern {
            elements: V::try_collect_items(
                ctx,
                node.elements
                    .iter()
                    .map(|a| visitor.visit_pattern(ctx, a.ast_ref())),
            )?,
        })
    }

    pub struct IfPattern<'c, V: AstVisitor<'c>> {
        pub pattern: V::PatternRet,
        pub condition: V::ExpressionRet,
    }
    pub fn walk_if_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::IfPattern<'c>>,
    ) -> Result<IfPattern<'c, V>, V::Error> {
        Ok(IfPattern {
            pattern: visitor.visit_pattern(ctx, node.pattern.ast_ref())?,
            condition: visitor.visit_expression(ctx, node.condition.ast_ref())?,
        })
    }

    pub struct BindingPattern<'c, V: AstVisitor<'c>>(pub V::NameRet);
    pub fn walk_binding_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::BindingPattern<'c>>,
    ) -> Result<BindingPattern<'c, V>, V::Error> {
        Ok(BindingPattern(visitor.visit_name(ctx, node.0.ast_ref())?))
    }

    pub enum LiteralPattern<'c, V: AstVisitor<'c>> {
        Str(V::StrLiteralPatternRet),
        Char(V::CharLiteralPatternRet),
        Int(V::IntLiteralPatternRet),
        Float(V::FloatLiteralPatternRet),
    }

    pub fn walk_literal_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::LiteralPattern>,
    ) -> Result<LiteralPattern<'c, V>, V::Error> {
        Ok(match &*node {
            ast::LiteralPattern::Str(r) => {
                LiteralPattern::Str(visitor.visit_str_literal_pattern(ctx, node.with_body(r))?)
            }
            ast::LiteralPattern::Char(r) => {
                LiteralPattern::Char(visitor.visit_char_literal_pattern(ctx, node.with_body(r))?)
            }
            ast::LiteralPattern::Int(r) => {
                LiteralPattern::Int(visitor.visit_int_literal_pattern(ctx, node.with_body(r))?)
            }
            ast::LiteralPattern::Float(r) => {
                LiteralPattern::Float(visitor.visit_float_literal_pattern(ctx, node.with_body(r))?)
            }
        })
    }

    pub fn walk_literal_pattern_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::LiteralPattern>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<
            'c,
            StrLiteralPatternRet = Ret,
            CharLiteralPatternRet = Ret,
            IntLiteralPatternRet = Ret,
            FloatLiteralPatternRet = Ret,
        >,
    {
        Ok(match walk_literal_pattern(visitor, ctx, node)? {
            LiteralPattern::Str(r) => r,
            LiteralPattern::Char(r) => r,
            LiteralPattern::Int(r) => r,
            LiteralPattern::Float(r) => r,
        })
    }

    pub struct DestructuringPattern<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub pattern: V::PatternRet,
    }
    pub fn walk_destructuring_pattern<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::DestructuringPattern<'c>>,
    ) -> Result<DestructuringPattern<'c, V>, V::Error> {
        Ok(DestructuringPattern {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            pattern: visitor.visit_pattern(ctx, node.pattern.ast_ref())?,
        })
    }

    pub struct ExprStatement<'c, V: AstVisitor<'c>>(pub V::ExpressionRet);
    pub fn walk_expr_statement<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::ExprStatement<'c>>,
    ) -> Result<ExprStatement<'c, V>, V::Error> {
        Ok(ExprStatement(
            visitor.visit_expression(ctx, node.0.ast_ref())?,
        ))
    }

    pub struct ReturnStatement<'c, V: AstVisitor<'c>>(pub Option<V::ExpressionRet>);
    pub fn walk_return_statement<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::ReturnStatement<'c>>,
    ) -> Result<ReturnStatement<'c, V>, V::Error> {
        Ok(ReturnStatement(
            node.0
                .as_ref()
                .map(|n| visitor.visit_expression(ctx, n.ast_ref()))
                .transpose()?,
        ))
    }

    pub struct BlockStatement<'c, V: AstVisitor<'c>>(pub V::BlockRet);
    pub fn walk_block_statement<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::BlockStatement<'c>>,
    ) -> Result<BlockStatement<'c, V>, V::Error> {
        Ok(BlockStatement(visitor.visit_block(ctx, node.0.ast_ref())?))
    }

    pub struct LetStatement<'c, V: AstVisitor<'c>> {
        pub pattern: V::PatternRet,
        pub ty: Option<V::TypeRet>,
        pub bound: Option<V::BoundRet>,
        pub value: Option<V::ExpressionRet>,
    }
    pub fn walk_let_statement<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::LetStatement<'c>>,
    ) -> Result<LetStatement<'c, V>, V::Error> {
        Ok(LetStatement {
            pattern: visitor.visit_pattern(ctx, node.pattern.ast_ref())?,
            ty: node
                .ty
                .as_ref()
                .map(|t| visitor.visit_type(ctx, t.ast_ref()))
                .transpose()?,
            bound: node
                .bound
                .as_ref()
                .map(|t| visitor.visit_bound(ctx, t.ast_ref()))
                .transpose()?,
            value: node
                .value
                .as_ref()
                .map(|t| visitor.visit_expression(ctx, t.ast_ref()))
                .transpose()?,
        })
    }

    pub struct AssignStatement<'c, V: AstVisitor<'c>> {
        pub lhs: V::ExpressionRet,
        pub rhs: V::ExpressionRet,
    }
    pub fn walk_assign_statement<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::AssignStatement<'c>>,
    ) -> Result<AssignStatement<'c, V>, V::Error> {
        Ok(AssignStatement {
            lhs: visitor.visit_expression(ctx, node.lhs.ast_ref())?,
            rhs: visitor.visit_expression(ctx, node.rhs.ast_ref())?,
        })
    }

    pub struct StructDefEntry<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub ty: Option<V::TypeRet>,
        pub default: Option<V::ExpressionRet>,
    }
    pub fn walk_struct_def_entry<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::StructDefEntry<'c>>,
    ) -> Result<StructDefEntry<'c, V>, V::Error> {
        Ok(StructDefEntry {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            ty: node
                .ty
                .as_ref()
                .map(|t| visitor.visit_type(ctx, t.ast_ref()))
                .transpose()?,
            default: node
                .default
                .as_ref()
                .map(|d| visitor.visit_expression(ctx, d.ast_ref()))
                .transpose()?,
        })
    }

    pub struct StructDef<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub bound: Option<V::BoundRet>,
        pub entries: V::CollectionContainer<V::StructDefEntryRet>,
    }
    pub fn walk_struct_def<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::StructDef<'c>>,
    ) -> Result<StructDef<'c, V>, V::Error> {
        Ok(StructDef {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            bound: node
                .bound
                .as_ref()
                .map(|b| visitor.visit_bound(ctx, b.ast_ref()))
                .transpose()?,
            entries: V::try_collect_items(
                ctx,
                node.entries
                    .iter()
                    .map(|b| visitor.visit_struct_def_entry(ctx, b.ast_ref())),
            )?,
        })
    }

    pub struct EnumDefEntry<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub args: V::CollectionContainer<V::TypeRet>,
    }
    pub fn walk_enum_def_entry<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::EnumDefEntry<'c>>,
    ) -> Result<EnumDefEntry<'c, V>, V::Error> {
        Ok(EnumDefEntry {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            args: V::try_collect_items(
                ctx,
                node.args
                    .iter()
                    .map(|b| visitor.visit_type(ctx, b.ast_ref())),
            )?,
        })
    }

    pub struct EnumDef<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub bound: Option<V::BoundRet>,
        pub entries: V::CollectionContainer<V::EnumDefEntryRet>,
    }
    pub fn walk_enum_def<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::EnumDef<'c>>,
    ) -> Result<EnumDef<'c, V>, V::Error> {
        Ok(EnumDef {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            bound: node
                .bound
                .as_ref()
                .map(|b| visitor.visit_bound(ctx, b.ast_ref()))
                .transpose()?,
            entries: V::try_collect_items(
                ctx,
                node.entries
                    .iter()
                    .map(|b| visitor.visit_enum_def_entry(ctx, b.ast_ref())),
            )?,
        })
    }

    pub struct TraitBound<'c, V: AstVisitor<'c>> {
        pub name: V::AccessNameRet,
        pub type_args: V::CollectionContainer<V::TypeRet>,
    }
    pub fn walk_trait_bound<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::TraitBound<'c>>,
    ) -> Result<TraitBound<'c, V>, V::Error> {
        Ok(TraitBound {
            name: visitor.visit_access_name(ctx, node.name.ast_ref())?,
            type_args: V::try_collect_items(
                ctx,
                node.type_args
                    .iter()
                    .map(|t| visitor.visit_type(ctx, t.ast_ref())),
            )?,
        })
    }

    pub struct Bound<'c, V: AstVisitor<'c>> {
        pub type_args: V::CollectionContainer<V::TypeRet>,
        pub trait_bounds: V::CollectionContainer<V::TraitBoundRet>,
    }
    pub fn walk_bound<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Bound<'c>>,
    ) -> Result<Bound<'c, V>, V::Error> {
        Ok(Bound {
            type_args: V::try_collect_items(
                ctx,
                node.type_args
                    .iter()
                    .map(|t| visitor.visit_type(ctx, t.ast_ref())),
            )?,
            trait_bounds: V::try_collect_items(
                ctx,
                node.trait_bounds
                    .iter()
                    .map(|t| visitor.visit_trait_bound(ctx, t.ast_ref())),
            )?,
        })
    }

    pub struct TraitDef<'c, V: AstVisitor<'c>> {
        pub name: V::NameRet,
        pub bound: V::BoundRet,
        pub trait_type: V::TypeRet,
    }
    pub fn walk_trait_def<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::TraitDef<'c>>,
    ) -> Result<TraitDef<'c, V>, V::Error> {
        Ok(TraitDef {
            name: visitor.visit_name(ctx, node.name.ast_ref())?,
            bound: visitor.visit_bound(ctx, node.bound.ast_ref())?,
            trait_type: visitor.visit_type(ctx, node.trait_type.ast_ref())?,
        })
    }

    pub enum Statement<'c, V: AstVisitor<'c>> {
        Expr(V::ExprStatementRet),
        Return(V::ReturnStatementRet),
        Block(V::BlockStatementRet),
        Break(V::BreakStatementRet),
        Continue(V::ContinueStatementRet),
        Let(V::LetStatementRet),
        Assign(V::AssignStatementRet),
        StructDef(V::StructDefRet),
        EnumDef(V::EnumDefRet),
        TraitDef(V::TraitDefRet),
    }

    pub fn walk_statement<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Statement<'c>>,
    ) -> Result<Statement<'c, V>, V::Error> {
        Ok(match &*node {
            ast::Statement::Expr(r) => {
                Statement::Expr(visitor.visit_expr_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::Return(r) => {
                Statement::Return(visitor.visit_return_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::Block(r) => {
                Statement::Block(visitor.visit_block_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::Break(r) => {
                Statement::Break(visitor.visit_break_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::Continue(r) => {
                Statement::Continue(visitor.visit_continue_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::Let(r) => {
                Statement::Let(visitor.visit_let_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::Assign(r) => {
                Statement::Assign(visitor.visit_assign_statement(ctx, node.with_body(r))?)
            }
            ast::Statement::StructDef(r) => {
                Statement::StructDef(visitor.visit_struct_def(ctx, node.with_body(r))?)
            }
            ast::Statement::EnumDef(r) => {
                Statement::EnumDef(visitor.visit_enum_def(ctx, node.with_body(r))?)
            }
            ast::Statement::TraitDef(r) => {
                Statement::TraitDef(visitor.visit_trait_def(ctx, node.with_body(r))?)
            }
        })
    }

    pub fn walk_statement_same_children<'c, V, Ret>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Statement<'c>>,
    ) -> Result<Ret, V::Error>
    where
        V: AstVisitor<
            'c,
            ExprStatementRet = Ret,
            ReturnStatementRet = Ret,
            BlockStatementRet = Ret,
            BreakStatementRet = Ret,
            ContinueStatementRet = Ret,
            LetStatementRet = Ret,
            AssignStatementRet = Ret,
            StructDefRet = Ret,
            EnumDefRet = Ret,
            TraitDefRet = Ret,
        >,
    {
        Ok(match walk_statement(visitor, ctx, node)? {
            Statement::Expr(r) => r,
            Statement::Return(r) => r,
            Statement::Block(r) => r,
            Statement::Break(r) => r,
            Statement::Continue(r) => r,
            Statement::Let(r) => r,
            Statement::Assign(r) => r,
            Statement::StructDef(r) => r,
            Statement::EnumDef(r) => r,
            Statement::TraitDef(r) => r,
        })
    }

    pub struct Module<'c, V: AstVisitor<'c>> {
        pub contents: V::CollectionContainer<V::StatementRet>,
    }

    pub fn walk_module<'c, V: AstVisitor<'c>>(
        visitor: &mut V,
        ctx: &V::Ctx,
        node: ast::AstNodeRef<ast::Module<'c>>,
    ) -> Result<Module<'c, V>, V::Error> {
        Ok(Module {
            contents: V::try_collect_items(
                ctx,
                node.contents
                    .iter()
                    .map(|s| visitor.visit_statement(ctx, s.ast_ref())),
            )?,
        })
    }
}
