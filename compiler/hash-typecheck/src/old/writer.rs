//! All rights reserved 2022 (c) The Hash Language authors
use crate::{
    storage::GlobalStorage,
    types::{
        EnumDef, FnType, RawRefType, RefType, StructDef, TupleType, TypeId, TypeVar, UserType,
    },
};
use core::fmt;
use hash_ast::ident::IDENTIFIER_MAP;
use hash_utils::tree_writing::TreeNode;

pub fn print_type_list(types: &[TypeId], storage: &GlobalStorage) {
    print!("[");
    for (i, ty) in types.iter().enumerate() {
        print!("{}", TypeWithStorage::new(*ty, storage));
        if i != types.len() - 1 {
            print!(", ");
        }
    }
    println!("]");
}

pub fn print_type(ty: TypeId, storage: &GlobalStorage) {
    println!("{}", TypeWithStorage::new(ty, storage));
}

pub struct TypeWithStorage<'g, 'c, 'w> {
    ty: TypeId,
    storage: &'g GlobalStorage<'c, 'w>,
}

impl<'g, 'c, 'w> TypeWithStorage<'g, 'c, 'w> {
    pub fn new(ty: TypeId, storage: &'g GlobalStorage<'c, 'w>) -> Self {
        Self { ty, storage }
    }

    #[must_use]
    pub fn for_type(&self, ty: TypeId) -> Self {
        Self { ty, ..*self }
    }

    pub fn to_tree_node(&self) -> TreeNode {
        match self.storage.types.get(self.ty) {
            crate::types::TypeValue::Ref(RefType { inner }) => {
                TreeNode::branch("ref", vec![self.for_type(*inner).to_tree_node()])
            }
            crate::types::TypeValue::RawRef(RawRefType { inner }) => {
                TreeNode::branch("raw_ref", vec![self.for_type(*inner).to_tree_node()])
            }
            crate::types::TypeValue::Fn(FnType { args, return_ty }) => TreeNode::branch(
                "function",
                vec![
                    TreeNode::branch(
                        "arguments",
                        args.iter()
                            .map(|(name, arg)| {
                                if let Some(name) = name {
                                    TreeNode::branch(
                                        "field",
                                        vec![
                                            TreeNode::leaf(format!(
                                                "name `{}`",
                                                IDENTIFIER_MAP.get_ident(*name)
                                            )),
                                            self.for_type(*arg).to_tree_node(),
                                        ],
                                    )
                                } else {
                                    self.for_type(*arg).to_tree_node()
                                }
                            })
                            .collect(),
                    ),
                    TreeNode::branch("return", vec![self.for_type(*return_ty).to_tree_node()]),
                ],
            ),
            crate::types::TypeValue::Var(TypeVar { name }) => {
                TreeNode::leaf(format!("var \"{}\"", IDENTIFIER_MAP.get_ident(*name)))
            }
            crate::types::TypeValue::Prim(prim) => TreeNode::leaf(format!(
                "primitive \"{}\"",
                match prim {
                    crate::types::PrimType::USize => "usize",
                    crate::types::PrimType::U8 => "u8",
                    crate::types::PrimType::U16 => "u16",
                    crate::types::PrimType::U32 => "u32",
                    crate::types::PrimType::U64 => "u64",
                    crate::types::PrimType::ISize => "isize",
                    crate::types::PrimType::I8 => "i8",
                    crate::types::PrimType::I16 => "i16",
                    crate::types::PrimType::I32 => "i32",
                    crate::types::PrimType::I64 => "i64",
                    crate::types::PrimType::F32 => "f32",
                    crate::types::PrimType::F64 => "f64",
                    crate::types::PrimType::Char => "char",
                    crate::types::PrimType::Void => "void",
                    crate::types::PrimType::Bool => "bool",
                }
            )),
            crate::types::TypeValue::User(UserType { def_id, args }) => {
                let label = match self.storage.type_defs.get(*def_id).kind {
                    crate::types::TypeDefValueKind::Enum(EnumDef { name, .. }) => {
                        format!("enum \"{}\"", IDENTIFIER_MAP.get_ident(name))
                    }
                    crate::types::TypeDefValueKind::Struct(StructDef { name, .. }) => {
                        format!("struct \"{}\"", IDENTIFIER_MAP.get_ident(name))
                    }
                };

                TreeNode::branch(
                    label,
                    vec![TreeNode::branch(
                        "parameters",
                        args.iter()
                            .map(|&a| self.for_type(a).to_tree_node())
                            .collect(),
                    )],
                )
            }
            // @@Todo: print trait bounds
            crate::types::TypeValue::Unknown(_) => TreeNode::leaf("unknown".to_owned()),
            crate::types::TypeValue::Namespace(_) => {
                todo!()
                // TreeNode::leaf(format!("namespace ({:?})", module_idx))
            }
            crate::types::TypeValue::Tuple(TupleType { types }) => TreeNode::branch(
                "tuple",
                types
                    .iter()
                    .map(|(name, arg)| {
                        if let Some(name) = name {
                            TreeNode::branch(
                                "field",
                                vec![
                                    TreeNode::leaf(format!(
                                        "name `{}`",
                                        IDENTIFIER_MAP.get_ident(*name)
                                    )),
                                    self.for_type(*arg).to_tree_node(),
                                ],
                            )
                        } else {
                            self.for_type(*arg).to_tree_node()
                        }
                    })
                    .collect(),
            ),
        }
    }
}

impl<'g, 'c, 'w> fmt::Display for TypeWithStorage<'g, 'c, 'w> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.storage.types.get(self.ty) {
            crate::types::TypeValue::Ref(RefType { inner }) => {
                write!(f, "&{}", self.for_type(*inner))?;
            }
            crate::types::TypeValue::RawRef(RawRefType { inner }) => {
                write!(f, "&raw {}", self.for_type(*inner))?;
            }
            crate::types::TypeValue::Fn(FnType { args, return_ty }) => {
                write!(f, "(")?;
                for (i, (name, arg)) in args.iter().enumerate() {
                    if let Some(name) = name {
                        write!(f, "{}: ", IDENTIFIER_MAP.get_ident(*name))?;
                    };

                    write!(f, "{}", self.for_type(*arg))?;
                    if i != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") => {}", self.for_type(*return_ty))?;
            }
            crate::types::TypeValue::Var(TypeVar { name }) => {
                write!(f, "{}", IDENTIFIER_MAP.get_ident(*name))?;
            }
            crate::types::TypeValue::User(UserType { def_id, args }) => {
                match self.storage.type_defs.get(*def_id).kind {
                    crate::types::TypeDefValueKind::Enum(EnumDef { name, .. }) => {
                        write!(f, "{}", IDENTIFIER_MAP.get_ident(name))?;
                    }
                    crate::types::TypeDefValueKind::Struct(StructDef { name, .. }) => {
                        write!(f, "{}", IDENTIFIER_MAP.get_ident(name))?;
                    }
                };

                if !args.is_empty() {
                    write!(f, "<")?;
                }
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", self.for_type(*arg))?;
                    if i != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                if !args.is_empty() {
                    write!(f, ">")?;
                }
            }
            crate::types::TypeValue::Prim(prim) => {
                write!(
                    f,
                    "{}",
                    match prim {
                        crate::types::PrimType::USize => "usize",
                        crate::types::PrimType::U8 => "u8",
                        crate::types::PrimType::U16 => "u16",
                        crate::types::PrimType::U32 => "u32",
                        crate::types::PrimType::U64 => "u64",
                        crate::types::PrimType::ISize => "isize",
                        crate::types::PrimType::I8 => "i8",
                        crate::types::PrimType::I16 => "i16",
                        crate::types::PrimType::I32 => "i32",
                        crate::types::PrimType::I64 => "i64",
                        crate::types::PrimType::F32 => "f32",
                        crate::types::PrimType::F64 => "f64",
                        crate::types::PrimType::Char => "char",
                        crate::types::PrimType::Void => "void",
                        crate::types::PrimType::Bool => "bool",
                    }
                )?;
            }
            crate::types::TypeValue::Tuple(TupleType { types }) => {
                // @@Todo: this is not exactly the right syntax, we need trailing commas in some
                // cases.
                write!(f, "(")?;
                for (i, (name, ty)) in types.iter().enumerate() {
                    if let Some(name) = name {
                        write!(f, "{}: ", IDENTIFIER_MAP.get_ident(*name))?;
                    };

                    write!(f, "{}", self.for_type(*ty))?;
                    if i != types.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
            }
            crate::types::TypeValue::Unknown(_) => {
                write!(f, "unknown")?;
            }
            crate::types::TypeValue::Namespace(_) => {
                write!(f, "{{module}}")?;
            }
        }

        Ok(())
    }
}