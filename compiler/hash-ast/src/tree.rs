//! AST visualisation utilities.

use std::{convert::Infallible, iter};

use hash_utils::tree_writing::TreeNode;

use crate::{
    ast,
    visitor::{walk, AstVisitor},
};

/// Struct implementing [crate::visitor::AstVisitor], for the purpose of
/// transforming the AST tree into a [TreeNode] tree, for visualisation
/// purposes.
pub struct AstTreeGenerator;

/// Easy way to format a [TreeNode] label with a main label as well as short
/// contents, and a quoting string.
fn labelled(label: impl ToString, contents: impl ToString, quote_str: &str) -> String {
    format!("{} {}{}{}", label.to_string(), quote_str, contents.to_string(), quote_str)
}

impl AstVisitor for AstTreeGenerator {
    type Ctx = ();

    type CollectionContainer<T> = Vec<T>;

    fn try_collect_items<T, E, I: Iterator<Item = Result<T, E>>>(
        _: &Self::Ctx,
        items: I,
    ) -> Result<Self::CollectionContainer<T>, E> {
        items.collect()
    }

    type Error = Infallible;
    type NameRet = TreeNode;

    fn visit_name(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::Name>,
    ) -> Result<Self::NameRet, Self::Error> {
        Ok(TreeNode::leaf(node.ident))
    }

    type LitRet = TreeNode;
    fn visit_lit(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Lit>,
    ) -> Result<Self::LitRet, Self::Error> {
        walk::walk_lit_same_children(self, ctx, node)
    }

    type MapLitRet = TreeNode;
    fn visit_map_lit(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MapLit>,
    ) -> Result<Self::MapLitRet, Self::Error> {
        Ok(TreeNode::branch("map", walk::walk_map_lit(self, ctx, node)?.entries))
    }

    type MapLitEntryRet = TreeNode;
    fn visit_map_lit_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MapLitEntry>,
    ) -> Result<Self::MapLitEntryRet, Self::Error> {
        let walk::MapLitEntry { key, value } = walk::walk_map_lit_entry(self, ctx, node)?;
        Ok(TreeNode::branch(
            "entry",
            vec![TreeNode::branch("key", vec![key]), TreeNode::branch("value", vec![value])],
        ))
    }

    type ListLitRet = TreeNode;
    fn visit_list_lit(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ListLit>,
    ) -> Result<Self::ListLitRet, Self::Error> {
        let children = walk::walk_list_lit(self, ctx, node)?;
        Ok(TreeNode::branch("list", children.elements))
    }

    type SetLitRet = TreeNode;
    fn visit_set_lit(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::SetLit>,
    ) -> Result<Self::SetLitRet, Self::Error> {
        let children = walk::walk_set_lit(self, ctx, node)?;
        Ok(TreeNode::branch("set", children.elements))
    }

    type TupleLitEntryRet = TreeNode;
    fn visit_tuple_lit_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TupleLitEntry>,
    ) -> Result<Self::TupleLitRet, Self::Error> {
        let walk::TupleLitEntry { name, ty, value } = walk::walk_tuple_lit_entry(self, ctx, node)?;

        Ok(TreeNode::branch(
            "entry",
            name.map(|t| TreeNode::branch("name", vec![t]))
                .into_iter()
                .chain(ty.map(|t| TreeNode::branch("type", vec![t])).into_iter())
                .chain(iter::once(TreeNode::branch("value", vec![value])))
                .collect(),
        ))
    }

    type TupleLitRet = TreeNode;

    fn visit_tuple_lit(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TupleLit>,
    ) -> Result<Self::TupleLitRet, Self::Error> {
        let children = walk::walk_tuple_lit(self, ctx, node)?;
        Ok(TreeNode::branch("tuple", children.elements))
    }

    type StrLitRet = TreeNode;
    fn visit_str_lit(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::StrLit>,
    ) -> Result<Self::StrLitRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("str", node.0, "\"")))
    }

    type CharLitRet = TreeNode;
    fn visit_char_lit(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::CharLit>,
    ) -> Result<Self::CharLitRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("char", node.0, "'")))
    }

    type FloatLitRet = TreeNode;
    fn visit_float_lit(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::FloatLit>,
    ) -> Result<Self::FloatLitRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("float", node.0, "")))
    }

    type BoolLitRet = TreeNode;
    fn visit_bool_lit(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::BoolLit>,
    ) -> Result<Self::BoolLitRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("bool", node.0, "")))
    }

    type IntLitRet = TreeNode;
    fn visit_int_lit(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::IntLit>,
    ) -> Result<Self::IntLitRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("int", node.0, "")))
    }

    type BinaryOperatorRet = TreeNode;
    fn visit_binary_operator(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::BinOp>,
    ) -> Result<Self::BinaryOperatorRet, Self::Error> {
        Ok(TreeNode::leaf(format!("operator `{}`", node.body())))
    }

    type UnaryOperatorRet = TreeNode;
    fn visit_unary_operator(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::UnOp>,
    ) -> Result<Self::UnaryOperatorRet, Self::Error> {
        Ok(TreeNode::leaf(format!("operator `{}`", node.body())))
    }

    type ExprRet = TreeNode;
    fn visit_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Expr>,
    ) -> Result<Self::ExprRet, Self::Error> {
        walk::walk_expr_same_children(self, ctx, node)
    }

    type VariableExprRet = TreeNode;
    fn visit_variable_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::VariableExpr>,
    ) -> Result<Self::VariableExprRet, Self::Error> {
        let walk::VariableExpr { name } = walk::walk_variable_expr(self, ctx, node)?;

        Ok(TreeNode::branch("variable", vec![TreeNode::leaf(labelled("named", name.label, "\""))]))
    }

    type DirectiveExprRet = TreeNode;
    fn visit_directive_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::DirectiveExpr>,
    ) -> Result<Self::DirectiveExprRet, Self::Error> {
        let walk::DirectiveExpr { subject, .. } = walk::walk_directive_expr(self, ctx, node)?;

        Ok(TreeNode::branch(labelled("directive", node.name.ident, "\""), vec![subject]))
    }

    type ConstructorCallArgRet = TreeNode;
    fn visit_constructor_call_arg(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ConstructorCallArg>,
    ) -> Result<Self::ConstructorCallArgRet, Self::Error> {
        if let Some(name) = &node.name {
            Ok(TreeNode::branch(
                "arg",
                vec![
                    TreeNode::leaf(labelled("named", name.ident, "\"")),
                    TreeNode::branch("value", vec![self.visit_expr(ctx, node.value.ast_ref())?]),
                ],
            ))
        } else {
            self.visit_expr(ctx, node.value.ast_ref())
        }
    }

    type ConstructorCallArgsRet = TreeNode;
    fn visit_constructor_call_args(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ConstructorCallArgs>,
    ) -> Result<Self::ConstructorCallArgsRet, Self::Error> {
        Ok(TreeNode::branch(
            "args",
            walk::walk_constructor_call_args(self, ctx, node)?.entries.into_iter().collect(),
        ))
    }

    type ConstructorCallExprRet = TreeNode;
    fn visit_constructor_call_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ConstructorCallExpr>,
    ) -> Result<Self::ConstructorCallExprRet, Self::Error> {
        let walk::ConstructorCallExpr { subject, args } =
            walk::walk_constructor_call_expr(self, ctx, node)?;

        let children = if !node.args.entries.is_empty() {
            vec![TreeNode::branch("subject", vec![subject]), args]
        } else {
            vec![TreeNode::branch("subject", vec![subject])]
        };

        Ok(TreeNode::branch("constructor", children))
    }

    type AccessExprRet = TreeNode;
    fn visit_access_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AccessExpr>,
    ) -> Result<Self::AccessExprRet, Self::Error> {
        let walk::AccessExpr { subject, .. } = walk::walk_access_expr(self, ctx, node)?;
        Ok(TreeNode::branch(
            "access",
            vec![
                TreeNode::branch("subject", vec![subject]),
                TreeNode::leaf(labelled("property", node.property.ident, "\"")),
                TreeNode::leaf(labelled("kind", node.kind, "\"")),
            ],
        ))
    }

    type AccessKindRet = TreeNode;
    fn visit_access_kind(
        &mut self,
        _: &Self::Ctx,
        node: ast::AccessKind,
    ) -> Result<Self::AccessKindRet, Self::Error> {
        match node {
            ast::AccessKind::Property => Ok(TreeNode::leaf("property")),
            ast::AccessKind::Namespace => Ok(TreeNode::leaf("namespace")),
        }
    }

    type RefExprRet = TreeNode;
    fn visit_ref_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::RefExpr>,
    ) -> Result<Self::RefExprRet, Self::Error> {
        let walk::RefExpr { inner_expr, mutability } = walk::walk_ref_expr(self, ctx, node)?;
        Ok(TreeNode::branch(
            "ref",
            iter::once(inner_expr)
                .chain(mutability.map(|inner| TreeNode::branch("mutability", vec![inner])))
                .collect(),
        ))
    }

    type DerefExprRet = TreeNode;
    fn visit_deref_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::DerefExpr>,
    ) -> Result<Self::DerefExprRet, Self::Error> {
        let walk::DerefExpr(inner_expr) = walk::walk_deref_expr(self, ctx, node)?;
        Ok(TreeNode::branch("deref", vec![inner_expr]))
    }

    type UnsafeExprRet = TreeNode;
    fn visit_unsafe_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::UnsafeExpr>,
    ) -> Result<Self::DerefExprRet, Self::Error> {
        let walk::UnsafeExpr(inner_expr) = walk::walk_unsafe_expr(self, ctx, node)?;
        Ok(TreeNode::branch("unsafe", vec![inner_expr]))
    }

    type LitExprRet = TreeNode;
    fn visit_lit_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LitExpr>,
    ) -> Result<Self::LitExprRet, Self::Error> {
        let walk::LitExpr(lit) = walk::walk_lit_expr(self, ctx, node)?;
        Ok(lit)
    }

    type CastExprRet = TreeNode;
    fn visit_cast_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::CastExpr>,
    ) -> Result<Self::CastExprRet, Self::Error> {
        let walk::CastExpr { ty, expr } = walk::walk_cast_expr(self, ctx, node)?;
        Ok(TreeNode::branch(
            "cast",
            vec![TreeNode::branch("subject", vec![expr]), TreeNode::branch("type", vec![ty])],
        ))
    }

    type TyExprRet = TreeNode;
    fn visit_ty_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TyExpr>,
    ) -> Result<Self::TyExprRet, Self::Error> {
        let walk::TyExpr(ty) = walk::walk_ty_expr(self, ctx, node)?;

        Ok(TreeNode::branch("type_expr", vec![ty]))
    }

    type BlockExprRet = TreeNode;
    fn visit_block_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BlockExpr>,
    ) -> Result<Self::BlockExprRet, Self::Error> {
        Ok(walk::walk_block_expr(self, ctx, node)?.0)
    }

    type ImportRet = TreeNode;
    fn visit_import(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::Import>,
    ) -> Result<Self::ImportRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("import", node.path, "\"")))
    }

    type ImportExprRet = TreeNode;
    fn visit_import_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ImportExpr>,
    ) -> Result<Self::ImportExprRet, Self::Error> {
        Ok(walk::walk_import_expr(self, ctx, node)?.0)
    }

    type TyRet = TreeNode;
    fn visit_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Ty>,
    ) -> Result<Self::TyRet, Self::Error> {
        walk::walk_ty_same_children(self, ctx, node)
    }

    type TupleTyRet = TreeNode;
    fn visit_tuple_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TupleTy>,
    ) -> Result<Self::TupleTyRet, Self::Error> {
        let walk::TupleTy { entries } = walk::walk_tuple_ty(self, ctx, node)?;

        Ok(TreeNode::branch("tuple", entries))
    }

    type ListTyRet = TreeNode;
    fn visit_list_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ListTy>,
    ) -> Result<Self::TupleTyRet, Self::Error> {
        let walk::ListTy { inner } = walk::walk_list_ty(self, ctx, node)?;

        Ok(TreeNode::branch("list", vec![inner]))
    }

    type SetTyRet = TreeNode;
    fn visit_set_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::SetTy>,
    ) -> Result<Self::TupleTyRet, Self::Error> {
        let walk::SetTy { inner: key } = walk::walk_set_ty(self, ctx, node)?;

        Ok(TreeNode::branch("set", vec![key]))
    }

    type MapTyRet = TreeNode;
    fn visit_map_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MapTy>,
    ) -> Result<Self::TupleTyRet, Self::Error> {
        let walk::MapTy { key, value } = walk::walk_map_ty(self, ctx, node)?;

        Ok(TreeNode::branch(
            "map",
            vec![TreeNode::branch("key", vec![key]), TreeNode::branch("key", vec![value])],
        ))
    }

    type TyArgRet = TreeNode;
    fn visit_ty_arg(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TyArg>,
    ) -> Result<Self::TyArgRet, Self::Error> {
        let walk::TyArg { name, ty } = walk::walk_ty_arg(self, ctx, node)?;

        if let Some(name) = name {
            Ok(TreeNode::branch(
                "field",
                vec![TreeNode::branch("name", vec![name]), TreeNode::branch("type", vec![ty])],
            ))
        } else {
            Ok(ty)
        }
    }

    type FnTyRet = TreeNode;
    fn visit_fn_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FnTy>,
    ) -> Result<Self::FnTyRet, Self::Error> {
        let walk::FnTy { params, return_ty } = walk::walk_fn_ty(self, ctx, node)?;

        let return_child = TreeNode::branch("return", vec![return_ty]);

        let children = {
            if params.is_empty() {
                vec![return_child]
            } else {
                vec![TreeNode::branch("parameters", params), return_child]
            }
        };

        Ok(TreeNode::branch("function", children))
    }

    type TyFnRet = TreeNode;
    fn visit_ty_fn_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TyFn>,
    ) -> Result<Self::TyFnRet, Self::Error> {
        let walk::TyFn { params, return_ty } = walk::walk_ty_fn(self, ctx, node)?;

        let mut children = vec![TreeNode::branch("return", vec![return_ty])];

        // Add the parameters branch to the start
        if !params.is_empty() {
            children.insert(0, TreeNode::branch("parameters", params));
        }

        Ok(TreeNode::branch("type_function", children))
    }

    type TyFnCallRet = TreeNode;
    fn visit_ty_fn_call(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TyFnCall>,
    ) -> Result<Self::TyFnCallRet, Self::Error> {
        let walk::TyFnCall { subject, args } = walk::walk_ty_fn_call(self, ctx, node)?;

        Ok(TreeNode::branch(
            "type_function_call",
            vec![TreeNode::branch("subject", vec![subject]), TreeNode::branch("arguments", args)],
        ))
    }

    type NamedTyRet = TreeNode;
    fn visit_named_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::NamedTy>,
    ) -> Result<Self::NamedTyRet, Self::Error> {
        let walk::NamedTy { name } = walk::walk_named_ty(self, ctx, node)?;
        Ok(TreeNode::leaf(labelled("named", name.label, "\"")))
    }

    type AccessTyRet = TreeNode;
    fn visit_access_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AccessTy>,
    ) -> Result<Self::AccessTyRet, Self::Error> {
        let walk::AccessTy { subject, .. } = walk::walk_access_ty(self, ctx, node)?;
        Ok(TreeNode::branch(
            "access",
            vec![
                TreeNode::branch("subject", vec![subject]),
                TreeNode::leaf(labelled("property", node.property.ident, "\"")),
            ],
        ))
    }

    type RefTyRet = TreeNode;
    fn visit_ref_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::RefTy>,
    ) -> Result<Self::RefTyRet, Self::Error> {
        let walk::RefTy { inner, mutability, .. } = walk::walk_ref_ty(self, ctx, node)?;

        let label = if node.kind.as_ref().map_or(false, |t| *t.body() == ast::RefKind::Raw) {
            "raw_ref"
        } else {
            "ref"
        };

        Ok(TreeNode::branch(
            label,
            iter::once(inner)
                .chain(mutability.map(|t| TreeNode::branch("mutability", vec![t])))
                .collect(),
        ))
    }

    type MergeTyRet = TreeNode;
    fn visit_merge_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MergeTy>,
    ) -> Result<Self::MergeTyRet, Self::Error> {
        let walk::MergeTy { lhs, rhs } = walk::walk_merge_ty(self, ctx, node)?;

        Ok(TreeNode::branch(
            "merge_ty",
            vec![TreeNode::branch("lhs", vec![lhs]), TreeNode::branch("rhs", vec![rhs])],
        ))
    }

    type UnionTyRet = TreeNode;
    fn visit_union_ty(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::UnionTy>,
    ) -> Result<Self::UnionTyRet, Self::Error> {
        let walk::UnionTy { lhs, rhs } = walk::walk_union_ty(self, ctx, node)?;

        Ok(TreeNode::branch(
            "union",
            vec![TreeNode::branch("lhs", vec![lhs]), TreeNode::branch("rhs", vec![rhs])],
        ))
    }

    type TyFnDefRet = TreeNode;
    fn visit_ty_fn_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TyFnDef>,
    ) -> Result<Self::TyFnDefRet, Self::Error> {
        let walk::TyFnDef { params: args, return_ty, body } =
            walk::walk_ty_fn_def(self, ctx, node)?;

        Ok(TreeNode::branch(
            "type_function",
            iter::once(TreeNode::branch("args", args))
                .chain(return_ty.map(|r| TreeNode::branch("return_type", vec![r])))
                .chain(iter::once(TreeNode::branch("body", vec![body])))
                .collect(),
        ))
    }

    type FnDefRet = TreeNode;
    fn visit_fn_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::FnDef>,
    ) -> Result<Self::FnDefRet, Self::Error> {
        let walk::FnDef { args, fn_body, return_ty } = walk::walk_fn_def(self, ctx, node)?;

        Ok(TreeNode::branch(
            "function_def",
            iter::once(TreeNode::branch("args", args))
                .chain(return_ty.map(|r| TreeNode::branch("return_type", vec![r])))
                .chain(iter::once(TreeNode::branch("body", vec![fn_body])))
                .collect(),
        ))
    }

    type ParamRet = TreeNode;
    fn visit_param(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Param>,
    ) -> Result<Self::ParamRet, Self::Error> {
        let walk::Param { name, ty, default } = walk::walk_param(self, ctx, node)?;
        Ok(TreeNode::branch(
            "param",
            iter::once(TreeNode::branch("name", vec![name]))
                .chain(ty.map(|t| TreeNode::branch("type", vec![t])))
                .chain(default.map(|d| TreeNode::branch("default", vec![d])))
                .collect(),
        ))
    }

    type BlockRet = TreeNode;
    fn visit_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Block>,
    ) -> Result<Self::BlockRet, Self::Error> {
        walk::walk_block_same_children(self, ctx, node)
    }

    type MatchCaseRet = TreeNode;
    fn visit_match_case(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MatchCase>,
    ) -> Result<Self::MatchCaseRet, Self::Error> {
        let walk::MatchCase { expr, pat: pattern } = walk::walk_match_case(self, ctx, node)?;
        Ok(TreeNode::branch("case", vec![pattern, TreeNode::branch("branch", vec![expr])]))
    }

    type MatchBlockRet = TreeNode;

    fn visit_match_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MatchBlock>,
    ) -> Result<Self::MatchBlockRet, Self::Error> {
        let walk::MatchBlock { cases, subject } = walk::walk_match_block(self, ctx, node)?;

        Ok(TreeNode::branch(
            "match",
            vec![TreeNode::branch("subject", vec![subject]), TreeNode::branch("cases", cases)],
        ))
    }

    type LoopBlockRet = TreeNode;

    fn visit_loop_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LoopBlock>,
    ) -> Result<Self::LoopBlockRet, Self::Error> {
        let walk::LoopBlock(inner) = walk::walk_loop_block(self, ctx, node)?;
        Ok(TreeNode::branch("loop", vec![inner]))
    }

    type ForLoopBlockRet = TreeNode;
    fn visit_for_loop_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ForLoopBlock>,
    ) -> Result<Self::LoopBlockRet, Self::Error> {
        let walk::ForLoopBlock { pat: pattern, iterator, body } =
            walk::walk_for_loop_block(self, ctx, node)?;

        Ok(TreeNode::branch("for_loop", vec![pattern, iterator, body]))
    }

    type WhileLoopBlockRet = TreeNode;
    fn visit_while_loop_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::WhileLoopBlock>,
    ) -> Result<Self::WhileLoopBlockRet, Self::Error> {
        let walk::WhileLoopBlock { condition, body } =
            walk::walk_while_loop_block(self, ctx, node)?;

        Ok(TreeNode::branch("while_loop", vec![condition, body]))
    }

    type ModBlockRet = TreeNode;
    fn visit_mod_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ModBlock>,
    ) -> Result<Self::ModBlockRet, Self::Error> {
        let walk::ModBlock(inner) = walk::walk_mod_block(self, ctx, node)?;
        Ok(TreeNode::branch("module", inner.children))
    }

    type ImplBlockRet = TreeNode;
    fn visit_impl_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ImplBlock>,
    ) -> Result<Self::ImplBlockRet, Self::Error> {
        let walk::ImplBlock(inner) = walk::walk_impl_block(self, ctx, node)?;
        Ok(TreeNode::branch("impl", inner.children))
    }

    type IfClauseRet = TreeNode;
    fn visit_if_clause(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IfClause>,
    ) -> Result<Self::IfClauseRet, Self::Error> {
        let walk::IfClause { condition, body } = walk::walk_if_clause(self, ctx, node)?;

        Ok(TreeNode::branch(
            "clause",
            vec![
                TreeNode::branch("condition", vec![condition]),
                TreeNode::branch("body", vec![body]),
            ],
        ))
    }

    type IfBlockRet = TreeNode;
    fn visit_if_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IfBlock>,
    ) -> Result<Self::IfBlockRet, Self::Error> {
        let walk::IfBlock { clauses, otherwise } = walk::walk_if_block(self, ctx, node)?;

        let mut children = vec![TreeNode::branch("clauses", clauses)];

        if let Some(else_clause) = otherwise {
            children.push(TreeNode::branch("otherwise", vec![else_clause]))
        }

        Ok(TreeNode::branch("if", children))
    }

    type BodyBlockRet = TreeNode;
    fn visit_body_block(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BodyBlock>,
    ) -> Result<Self::BodyBlockRet, Self::Error> {
        let walk::BodyBlock { statements, expr } = walk::walk_body_block(self, ctx, node)?;

        let mut children = Vec::new();
        if !statements.is_empty() {
            children.push(TreeNode::branch("statements", statements));
        }
        if let Some(expr) = expr {
            children.push(TreeNode::branch("expr", vec![expr]));
        }

        Ok(TreeNode::branch("body", children))
    }

    type ReturnStatementRet = TreeNode;
    fn visit_return_statement(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ReturnStatement>,
    ) -> Result<Self::ReturnStatementRet, Self::Error> {
        let walk::ReturnStatement(inner) = walk::walk_return_statement(self, ctx, node)?;
        Ok(TreeNode::branch("return", inner.into_iter().collect()))
    }

    type BreakStatementRet = TreeNode;
    fn visit_break_statement(
        &mut self,
        _: &Self::Ctx,
        _: ast::AstNodeRef<ast::BreakStatement>,
    ) -> Result<Self::BreakStatementRet, Self::Error> {
        Ok(TreeNode::leaf("break"))
    }

    type ContinueStatementRet = TreeNode;
    fn visit_continue_statement(
        &mut self,
        _: &Self::Ctx,
        _: ast::AstNodeRef<ast::ContinueStatement>,
    ) -> Result<Self::ContinueStatementRet, Self::Error> {
        Ok(TreeNode::leaf("continue"))
    }

    type VisibilityRet = TreeNode;
    fn visit_visibility_modifier(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::Visibility>,
    ) -> Result<Self::VisibilityRet, Self::Error> {
        match node.body() {
            ast::Visibility::Private => Ok(TreeNode::leaf("private")),
            ast::Visibility::Public => Ok(TreeNode::leaf("public")),
        }
    }

    type MutabilityRet = TreeNode;
    fn visit_mutability_modifier(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::Mutability>,
    ) -> Result<Self::MutabilityRet, Self::Error> {
        match node.body() {
            ast::Mutability::Mutable => Ok(TreeNode::leaf("mutable")),
            ast::Mutability::Immutable => Ok(TreeNode::leaf("immutable")),
        }
    }

    type RefKindRet = ();

    fn visit_ref_kind(
        &mut self,
        _: &Self::Ctx,
        _: ast::AstNodeRef<ast::RefKind>,
    ) -> Result<Self::RefKindRet, Self::Error> {
        Ok(())
    }

    type DeclarationRet = TreeNode;

    fn visit_declaration(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Declaration>,
    ) -> Result<Self::DeclarationRet, Self::Error> {
        let walk::Declaration { pat: pattern, ty, value } =
            walk::walk_declaration(self, ctx, node)?;

        Ok(TreeNode::branch(
            "declaration",
            iter::once(TreeNode::branch("pattern", vec![pattern]))
                .chain(ty.map(|t| TreeNode::branch("type", vec![t])))
                .chain(value.map(|t| TreeNode::branch("value", vec![t])))
                .collect(),
        ))
    }

    type MergeDeclarationRet = TreeNode;
    fn visit_merge_declaration(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::MergeDeclaration>,
    ) -> Result<Self::MergeDeclarationRet, Self::Error> {
        let walk::MergeDeclaration { decl: pattern, value } =
            walk::walk_merge_declaration(self, ctx, node)?;

        Ok(TreeNode::branch("merge_declaration", vec![pattern, value]))
    }

    type AssignExprRet = TreeNode;
    fn visit_assign_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AssignExpr>,
    ) -> Result<Self::AssignExprRet, Self::Error> {
        let walk::AssignExpr { lhs, rhs } = walk::walk_assign_expr(self, ctx, node)?;
        Ok(TreeNode::branch(
            "assign",
            vec![TreeNode::branch("lhs", vec![lhs]), TreeNode::branch("rhs", vec![rhs])],
        ))
    }

    type AssignOpExprRet = TreeNode;
    fn visit_assign_op_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AssignOpExpr>,
    ) -> Result<Self::AssignOpExprRet, Self::Error> {
        let walk::AssignOpStatement { lhs, rhs, operator } =
            walk::walk_assign_op_statement(self, ctx, node)?;
        Ok(TreeNode::branch(
            "assign",
            vec![operator, TreeNode::branch("lhs", vec![lhs]), TreeNode::branch("rhs", vec![rhs])],
        ))
    }

    type BinaryExprRet = TreeNode;
    fn visit_binary_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BinaryExpr>,
    ) -> Result<Self::BinaryExprRet, Self::Error> {
        let walk::BinaryExpr { operator, lhs, rhs } = walk::walk_binary_expr(self, ctx, node)?;

        Ok(TreeNode::branch(
            "binary_expr",
            vec![operator, TreeNode::branch("lhs", vec![lhs]), TreeNode::branch("rhs", vec![rhs])],
        ))
    }

    type UnaryExprRet = TreeNode;
    fn visit_unary_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::UnaryExpr>,
    ) -> Result<Self::UnaryExprRet, Self::Error> {
        let walk::UnaryExpr { operator, expr } = walk::walk_unary_expr(self, ctx, node)?;

        Ok(TreeNode::branch("unary_expr", vec![operator, TreeNode::branch("expr", vec![expr])]))
    }

    type IndexExprRet = TreeNode;

    fn visit_index_expr(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IndexExpr>,
    ) -> Result<Self::IndexExprRet, Self::Error> {
        let walk::IndexExpr { subject, index_expr } = walk::walk_index_expr(self, ctx, node)?;

        Ok(TreeNode::branch(
            "index",
            vec![
                TreeNode::branch("subject", vec![subject]),
                TreeNode::branch("index_expr", vec![index_expr]),
            ],
        ))
    }

    type StructDefRet = TreeNode;
    fn visit_struct_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::StructDef>,
    ) -> Result<Self::StructDefRet, Self::Error> {
        let walk::StructDef { entries } = walk::walk_struct_def(self, ctx, node)?;
        Ok(TreeNode::branch(
            "struct_def",
            iter::once(TreeNode::branch("fields", entries)).collect(),
        ))
    }

    type EnumDefEntryRet = TreeNode;
    fn visit_enum_def_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::EnumDefEntry>,
    ) -> Result<Self::EnumDefEntryRet, Self::Error> {
        let walk::EnumDefEntry { name, args } = walk::walk_enum_def_entry(self, ctx, node)?;
        Ok(TreeNode::branch(
            labelled("variant", name.label, "\""),
            if args.is_empty() { vec![] } else { vec![TreeNode::branch("args", args)] },
        ))
    }

    type EnumDefRet = TreeNode;
    fn visit_enum_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::EnumDef>,
    ) -> Result<Self::EnumDefRet, Self::Error> {
        let walk::EnumDef { entries } = walk::walk_enum_def(self, ctx, node)?;
        Ok(TreeNode::branch(
            "enum_def",
            iter::once(TreeNode::branch("variants", entries)).collect(),
        ))
    }

    type TraitDefRet = TreeNode;
    fn visit_trait_def(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TraitDef>,
    ) -> Result<Self::TraitDefRet, Self::Error> {
        let walk::TraitDef { members } = walk::walk_trait_def(self, ctx, node)?;

        Ok(TreeNode::branch("trait_def", vec![TreeNode::branch("members", members)]))
    }

    type TraitImplRet = TreeNode;
    fn visit_trait_impl(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TraitImpl>,
    ) -> Result<Self::TraitImplRet, Self::Error> {
        let walk::TraitImpl { implementation, ty: name } = walk::walk_trait_impl(self, ctx, node)?;

        Ok(TreeNode::branch(
            "trait_impl",
            vec![name, TreeNode::branch("implementation", implementation)],
        ))
    }

    type PatRet = TreeNode;

    fn visit_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Pat>,
    ) -> Result<Self::PatRet, Self::Error> {
        walk::walk_pat_same_children(self, ctx, node)
    }

    type AccessPatRet = TreeNode;
    fn visit_access_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::AccessPat>,
    ) -> Result<Self::AccessPatRet, Self::Error> {
        let walk::AccessPat { subject, .. } = walk::walk_access_pat(self, ctx, node)?;
        Ok(TreeNode::branch(
            "access",
            vec![
                TreeNode::branch("subject", vec![subject]),
                TreeNode::leaf(labelled("property", node.property.ident, "\"")),
            ],
        ))
    }

    type ConstructorPatRet = TreeNode;
    fn visit_constructor_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ConstructorPat>,
    ) -> Result<Self::ConstructorPatRet, Self::Error> {
        let walk::ConstructorPat { subject, args } = walk::walk_constructor_pat(self, ctx, node)?;

        let children = if !node.fields.is_empty() {
            vec![TreeNode::branch("subject", vec![subject]), TreeNode::branch("args", args)]
        } else {
            vec![TreeNode::branch("subject", vec![subject])]
        };

        Ok(TreeNode::branch("constructor", children))
    }

    type TuplePatEntryRet = TreeNode;
    fn visit_tuple_pat_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TuplePatEntry>,
    ) -> Result<Self::TuplePatEntryRet, Self::Error> {
        let walk::TuplePatEntry { name, pat: pattern } =
            walk::walk_tuple_pat_entry(self, ctx, node)?;

        Ok(TreeNode::branch(
            "entry",
            name.map(|t| TreeNode::branch("name", vec![t]))
                .into_iter()
                .chain(iter::once(TreeNode::branch("pattern", vec![pattern])))
                .collect(),
        ))
    }

    type TuplePatRet = TreeNode;
    fn visit_tuple_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::TuplePat>,
    ) -> Result<Self::TuplePatRet, Self::Error> {
        let walk::TuplePat { elements } = walk::walk_tuple_pat(self, ctx, node)?;
        Ok(TreeNode::branch("tuple", elements))
    }

    type ListPatRet = TreeNode;
    fn visit_list_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ListPat>,
    ) -> Result<Self::TuplePatRet, Self::Error> {
        let walk::ListPat { elements } = walk::walk_list_pat(self, ctx, node)?;
        Ok(TreeNode::branch("list", elements))
    }

    type SpreadPatRet = TreeNode;
    fn visit_spread_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::SpreadPat>,
    ) -> Result<Self::SpreadPatRet, Self::Error> {
        let walk::SpreadPat { name } = walk::walk_spread_pat(self, ctx, node)?;

        if let Some(name) = name {
            Ok(TreeNode::leaf(labelled("spread", name.label, "\"")))
        } else {
            Ok(TreeNode::leaf("spread"))
        }
    }

    type StrLitPatRet = TreeNode;
    fn visit_str_lit_pat(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::StrLitPat>,
    ) -> Result<Self::StrLitPatRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("str", node.0, "\"")))
    }

    type CharLitPatRet = TreeNode;
    fn visit_char_lit_pat(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::CharLitPat>,
    ) -> Result<Self::CharLitPatRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("char", node.0, "\'")))
    }

    type IntLitPatRet = TreeNode;
    fn visit_int_lit_pat(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::IntLitPat>,
    ) -> Result<Self::IntLitPatRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("int", node.0, "")))
    }

    type FloatLitPatRet = TreeNode;
    fn visit_float_lit_pat(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::FloatLitPat>,
    ) -> Result<Self::FloatLitPatRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("float", node.0, "")))
    }

    type BoolLitPatRet = TreeNode;
    fn visit_bool_lit_pat(
        &mut self,
        _: &Self::Ctx,
        node: ast::AstNodeRef<ast::BoolLitPat>,
    ) -> Result<Self::BoolLitPatRet, Self::Error> {
        Ok(TreeNode::leaf(labelled("bool", node.0, "")))
    }

    type LitPatRet = TreeNode;
    fn visit_lit_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::LitPat>,
    ) -> Result<Self::LitPatRet, Self::Error> {
        walk::walk_lit_pat_same_children(self, ctx, node)
    }

    type OrPatRet = TreeNode;
    fn visit_or_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::OrPat>,
    ) -> Result<Self::OrPatRet, Self::Error> {
        let walk::OrPat { variants } = walk::walk_or_pat(self, ctx, node)?;
        Ok(TreeNode::branch("or", variants))
    }

    type IfPatRet = TreeNode;
    fn visit_if_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::IfPat>,
    ) -> Result<Self::IfPatRet, Self::Error> {
        let walk::IfPat { condition, pat: pattern } = walk::walk_if_pat(self, ctx, node)?;
        Ok(TreeNode::branch(
            "if",
            vec![
                TreeNode::branch("condition", vec![condition]),
                TreeNode::branch("pattern", vec![pattern]),
            ],
        ))
    }

    type BindingPatRet = TreeNode;
    fn visit_binding_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::BindingPat>,
    ) -> Result<Self::BindingPatRet, Self::Error> {
        let walk::BindingPat { name, .. } = walk::walk_binding_pat(self, ctx, node)?;

        Ok(TreeNode::branch(
            "binding",
            iter::once(TreeNode::leaf(labelled("name", name.label, "\"")))
                .chain(
                    node.visibility
                        .as_ref()
                        .map(|t| TreeNode::leaf(labelled("visibility", t.body(), "\""))),
                )
                .chain(
                    node.mutability
                        .as_ref()
                        .map(|t| TreeNode::leaf(labelled("mutability", t.body(), "\""))),
                )
                .collect(),
        ))
    }

    type IgnorePatRet = TreeNode;

    fn visit_ignore_pat(
        &mut self,
        _: &Self::Ctx,
        _: ast::AstNodeRef<ast::IgnorePat>,
    ) -> Result<Self::IgnorePatRet, Self::Error> {
        Ok(TreeNode::leaf("ignore"))
    }

    type ModulePatEntryRet = TreeNode;

    fn visit_module_pat_entry(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ModulePatEntry>,
    ) -> Result<Self::ModulePatEntryRet, Self::Error> {
        let walk::ModulePatEntry { name, pat: pattern } =
            walk::walk_module_pat_entry(self, ctx, node)?;
        Ok(TreeNode::branch(labelled("assign", name.label, "\""), vec![pattern]))
    }

    type ModulePatRet = TreeNode;

    fn visit_module_pat(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::ModulePat>,
    ) -> Result<Self::ModulePatRet, Self::Error> {
        let walk::ModulePat { fields: patterns } = walk::walk_module_pat(self, ctx, node)?;
        Ok(TreeNode::branch("module", vec![TreeNode::branch("members", patterns)]))
    }

    type ModuleRet = TreeNode;

    fn visit_module(
        &mut self,
        ctx: &Self::Ctx,
        node: ast::AstNodeRef<ast::Module>,
    ) -> Result<Self::ModuleRet, Self::Error> {
        let walk::Module { contents } = walk::walk_module(self, ctx, node)?;
        Ok(TreeNode::branch("module", contents))
    }
}
