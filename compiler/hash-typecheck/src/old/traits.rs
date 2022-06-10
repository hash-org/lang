//! All rights reserved 2022 (c) The Hash Language authors
use crate::{
    error::{ArgumentLengthMismatch, Symbol, TypecheckError, TypecheckResult},
    storage::{GlobalStorage, SourceStorage},
    types::{TypeId, TypeList, TypeStorage},
    unify::{Substitution, Unifier, UnifyStrategy},
    writer::TypeWithStorage,
};
use hash_alloc::Wall;
use hash_source::location::SourceLocation;
use hash_utils::counter;
use std::cell::Cell;
use std::collections::{BTreeMap, HashMap};

counter! {
    name: TraitId,
    counter_name: TRAIT_COUNTER,
    visibility: pub,
    method_visibility:,
}

#[derive(Debug)]
pub struct TraitBounds {
    pub bounds: Vec<TraitBound>,
}

impl TraitBounds {
    pub fn empty() -> Self {
        Self { bounds: vec![] }
    }
}

#[derive(Debug)]
pub struct TraitBound {
    pub trt: TraitId,
    pub params: TypeList,
}

impl TraitBound {}

counter! {
    name: TraitImplId,
    counter_name: TRAIT_IMPL_COUNTER,
    visibility: pub,
    method_visibility:,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub trait_id: TraitId,
    pub args: TypeList,
    pub bounds: TraitBounds,
}

#[derive(Debug)]
pub struct Trait {
    pub id: TraitId,
    pub args: TypeList,
    pub bounds: TraitBounds,
    pub fn_type: TypeId,
}

#[derive(Debug)]
pub struct ImplsForTrait {
    impls: BTreeMap<TraitImplId, Box<TraitImpl>>,
}

impl ImplsForTrait {
    pub fn empty() -> Self {
        Self {
            impls: BTreeMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (TraitImplId, &TraitImpl)> + '_ {
        self.impls.iter().map(|(&a, b)| (a, b.as_ref()))
    }
}

#[derive(Debug)]
pub struct TraitImplStorage {
    data: HashMap<TraitId, ImplsForTrait>,
}

impl TraitImplStorage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_impl(&mut self, trait_id: TraitId, trait_impl: TraitImpl) -> TraitImplId {
        let impls_for_trait = self
            .data
            .entry(trait_id)
            .or_insert_with(ImplsForTrait::empty);
        let id = TraitImplId::new();
        impls_for_trait.impls.insert(id, Box::new(trait_impl));
        id
    }

    pub fn get_impls(&mut self, trait_id: TraitId) -> &ImplsForTrait {
        self.data
            .entry(trait_id)
            .or_insert_with(ImplsForTrait::empty)
    }

    pub fn get_impls_mut(&mut self, trait_id: TraitId) -> &mut ImplsForTrait {
        self.data
            .entry(trait_id)
            .or_insert_with(ImplsForTrait::empty)
    }
}

impl Default for TraitImplStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct TraitStorage<'c, 'w> {
    data: HashMap<TraitId, Cell<&'c Trait>>,
    wall: &'w Wall<'c>,
}

impl<'c, 'w> TraitStorage<'c, 'w> {
    pub fn new(wall: &'w Wall<'c>) -> Self {
        Self {
            data: HashMap::new(),
            wall,
        }
    }

    pub fn get(&self, trait_id: TraitId) -> &'c Trait {
        self.data.get(&trait_id).unwrap().get()
    }

    pub fn create(&mut self, args: TypeList, bounds: TraitBounds, fn_type: TypeId) -> TraitId {
        let id = TraitId::new();
        self.data.insert(
            id,
            Cell::new(self.wall.alloc_value(Trait {
                id,
                args,
                bounds,
                fn_type,
            })),
        );
        id
    }
}

#[derive(Debug)]
pub struct CoreTraits {
    pub hash: TraitId,
    pub eq: TraitId,
}

impl<'c, 'w> CoreTraits {
    pub fn create(_types: &mut TypeStorage<'c, 'w>, _wall: &'w Wall) -> Self {
        CoreTraits {
            hash: TraitId::new(),
            eq: TraitId::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchTraitImplResult {
    pub sub_from_trait_def: Substitution,
    pub sub_from_trait_impl: Substitution,
}

pub struct TraitHelper<'c, 'w, 'ms, 'gs> {
    module_storage: &'ms mut SourceStorage,
    global_storage: &'gs mut GlobalStorage<'c, 'w>,
}

impl<'c, 'w, 'ms, 'gs> TraitHelper<'c, 'w, 'ms, 'gs> {
    pub fn new(
        module_storage: &'ms mut SourceStorage,
        global_storage: &'gs mut GlobalStorage<'c, 'w>,
    ) -> Self {
        Self {
            module_storage,
            global_storage,
        }
    }

    fn unifier(&mut self) -> Unifier<'c, 'w, '_, '_> {
        Unifier::new(self.module_storage, self.global_storage)
    }

    pub fn find_trait_impl(
        &mut self,
        trt: &Trait,
        trait_args: &[TypeId],
        fn_type: Option<TypeId>,
        trt_symbol: impl FnOnce() -> Symbol,
        args_location: Option<SourceLocation>,
    ) -> TypecheckResult<MatchTraitImplResult> {
        let mut trait_args = trait_args.to_owned();
        if trait_args.is_empty() {
            trait_args.extend(
                trt.args
                    .iter()
                    .map(|_| self.global_storage.types.create_unknown_type()),
            )
        }

        if trait_args.len() != trt.args.len() {
            return Err(TypecheckError::TypeArgumentLengthMismatch {
                location: args_location.or_else(|| trt_symbol().span()),
                mismatch: ArgumentLengthMismatch::new(trt.args.len(), trait_args.len()),
            });
        }

        // Resolve any remaining fn args
        let mut unifier = self.unifier();
        if let Some(fn_type) = fn_type {
            let trait_vars_sub = unifier.instantiate_vars_list(&trt.args)?;
            let instantiated_fn = unifier.apply_sub(&trait_vars_sub, trt.fn_type)?;
            unifier.unify(fn_type, instantiated_fn, UnifyStrategy::ModifyBoth)?;

            let instantiated_args =
                unifier.apply_sub_to_list_make_vec(&trait_vars_sub, &trt.args)?;
            unifier.unify_pairs(
                trait_args.iter().zip(instantiated_args.iter()),
                UnifyStrategy::ModifyTarget,
            )?;
        }

        // @@Performance: we have to collect due to lifetime issues, this is not ideal.
        // let impls: Vec<_> = self
        //     .global_storage
        //     .trait_impls
        //     .get_impls(trt.id)
        //     .iter()
        //     .collect();

        // for (_, trait_impl) in impls.iter() {
        //     match self.match_trait_impl(trait_impl, &trait_args) {
        //         Ok(matched) => {
        //             return Ok(matched);
        //         }
        //         Err(_e) => {
        //             continue;
        //             // last_err.replace(e);
        //         }
        //     }
        // }

        // @@Todo: better errors
        Err(TypecheckError::NoMatchingTraitImplementations(trt_symbol()))
    }

    pub fn print_types(&self, types: &[TypeId]) {
        print!("[");
        for &a in types {
            print!("{}, ", TypeWithStorage::new(a, self.global_storage));
        }
        println!("]");
    }

    pub fn match_trait_impl(
        &mut self,
        trait_impl: &TraitImpl,
        trait_args: &[TypeId],
    ) -> TypecheckResult<MatchTraitImplResult> {
        let trt = self.global_storage.traits.get(trait_impl.trait_id);

        // @@Ambiguity: for now let's assume all type variables in here are new
        let impl_vars: Vec<_> = trait_impl
            .args
            .iter()
            .flat_map(|&arg| {
                let arg_ty = self.global_storage.types.get(arg);

                arg_ty.fold_type_ids(vec![], |mut vars, ty_id| {
                    match self.global_storage.types.get(ty_id) {
                        crate::types::TypeValue::Var(_) => {
                            vars.push(ty_id);
                            vars
                        }
                        _ => vars,
                    }
                })
            })
            .collect();

        let mut unifier = Unifier::new(self.module_storage, self.global_storage);
        let trait_args_sub = unifier.instantiate_vars_list(&trt.args)?;
        let trait_impl_args_sub =
            unifier.instantiate_vars_for_list(&trait_impl.args, &impl_vars)?;
        let mut unifier = Unifier::new(self.module_storage, self.global_storage);
        let trait_args_instantiated =
            unifier.apply_sub_to_list_make_vec(&trait_args_sub, &trt.args)?;
        let trait_impl_args_instantiated =
            unifier.apply_sub_to_list_make_vec(&trait_impl_args_sub, &trait_impl.args)?;

        let mut unifier = Unifier::new(self.module_storage, self.global_storage);
        unifier.unify_pairs(
            trait_impl_args_instantiated
                .iter()
                .zip(trait_args_instantiated.iter()),
            UnifyStrategy::ModifyBoth,
        )?;
        unifier.unify_pairs(
            trait_args_instantiated.iter().zip(trait_args.iter()),
            UnifyStrategy::ModifyTarget,
        )?;

        let sub_from_trait_def =
            Substitution::from_pairs(trt.args.iter().zip(trait_args_instantiated.iter()));
        let sub_from_trait_impl =
            Substitution::from_pairs(trait_impl.args.iter().zip(trait_args_instantiated.iter()));

        Ok(MatchTraitImplResult {
            sub_from_trait_def,
            sub_from_trait_impl,
        })
    }
}