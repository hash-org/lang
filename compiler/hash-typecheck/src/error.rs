//! Error-related data structures for errors that occur during typechecking.
use std::fmt::Display;

use crate::storage::primitives::{AccessTerm, ArgsId, ParamsId, TermId};
use hash_source::identifier::Identifier;

/// Convenient type alias for a result with a [TcError] as the error type.
pub(crate) type TcResult<T> = Result<T, TcError>;

/// Particular reason why parameters couldn't be unified, either argument
/// length mis-match or that a name mismatched between the two given parameters.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ParamUnificationErrorReason {
    /// The provided and expected parameter lengths mismatched.
    LengthMismatch,
    /// A name mismatch of the parameters occurred at the particular
    /// index.
    NameMismatch(usize),
}

// / This enum describes the origin kind of the subject that a parameter
/// unification occurred on.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ParamUnificationOrigin {
    Tuple,
    Function,
    TypeFunction,
}

impl Display for ParamUnificationOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamUnificationOrigin::Tuple => write!(f, "tuple"),
            ParamUnificationOrigin::Function => write!(f, "function"),
            ParamUnificationOrigin::TypeFunction => write!(f, "type function"),
        }
    }
}

/// An error that occurs during typechecking.
#[derive(Debug, Clone)]
pub(crate) enum TcError {
    /// Cannot unify the two terms.
    CannotUnify { src: TermId, target: TermId },
    /// Cannot unify the two parameter lists. This can occur if the names
    /// don't match of the parameters or if the number of parameters isn't the
    /// same.
    CannotUnifyParams {
        src_params: ParamsId,
        target_params: ParamsId,
        src: TermId,
        target: TermId,
        origin: ParamUnificationOrigin,
        reason: ParamUnificationErrorReason,
    },
    /// The given term should be a type function but it isn't.
    NotATypeFunction { term: TermId },
    /// The given value cannot be used as a type.
    CannotUseValueAsTy { value: TermId },
    /// The given arguments do not match the length of the target parameters.
    MismatchingArgParamLength { args: ArgsId, params: ParamsId, target: TermId },
    /// The parameter with the given name is not found in the given parameter
    /// list.
    ParamNotFound { params: ParamsId, name: Identifier },
    /// There is a parameter (at the index `param_index_given_twice`) which is
    /// specified twice in the given argument list.
    ParamGivenTwice { args: ArgsId, params: ParamsId, param_index_given_twice: usize },
    /// It is invalid to use a positional argument after a named argument.
    CannotUsePositionalArgAfterNamedArg { args: ArgsId, problematic_arg_index: usize },
    /// The given name cannot be resolved in the given value.
    UnresolvedNameInValue { name: Identifier, value: TermId },
    /// The given variable cannot be resolved in the current context.
    UnresolvedVariable { name: Identifier },
    /// The given value does not support accessing (of the given name).
    UnsupportedAccess { name: Identifier, value: TermId },
    /// The given value does not support namespace accessing (of the given
    /// name).
    UnsupportedNamespaceAccess { name: Identifier, value: TermId },
    /// The given value does not support property accessing (of the given name).
    UnsupportedPropertyAccess { name: Identifier, value: TermId },
    /// The given type function cannot be applied to the given arguments, due to
    /// the given errors.
    InvalidTypeFunctionApplication {
        type_fn: TermId,
        args: ArgsId,
        unification_errors: Vec<TcError>,
    },
    /// The given term cannot be used in a merge operation.
    InvalidElementOfMerge { term: TermId },
    /// The given term cannot be used as a type function parameter type.
    InvalidTypeFunctionParameterType { param_ty: TermId },
    /// The given term cannot be used as a type function return type.
    InvalidTypeFunctionReturnType { return_ty: TermId },
    /// The given term cannot be used as a type function return value.
    InvalidTypeFunctionReturnValue { return_value: TermId },
    /// The given merge term should only contain zero or one nominal elements,
    /// but it contains more.
    MergeShouldOnlyContainOneNominal {
        merge_term: TermId,
        nominal_term: TermId,
        second_nominal_term: TermId,
    },
    /// The given merge term should contain only level 1 terms.
    MergeShouldBeLevel1 { merge_term: TermId, offending_term: TermId },
    /// The given merge term should contain only level 2 terms.
    MergeShouldBeLevel2 { merge_term: TermId, offending_term: TermId },
    /// More type annotations are needed to resolve the given term.
    NeedMoreTypeAnnotationsToResolve { term_to_resolve: TermId },
    /// The given term cannot be instantiated at runtime.
    TermIsNotRuntimeInstantiable { term: TermId },
    /// The given term cannot be used as the subject of a type function
    /// application.
    UnsupportedTypeFunctionApplication { subject_id: TermId },
    /// The given access operation results in more than one result.
    AmbiguousAccess { access: AccessTerm },
    /// The given access operation does not resolve to a method.
    InvalidPropertyAccessOfNonMethod { subject: TermId, property: Identifier },
    /// The given member requires an initialisation in the current scope.
    /// @@ErrorReporting: add span of member.
    UninitialisedMemberNotAllowed { member_ty: TermId },
    /// Cannot implement something that isn't a trait.
    CannotImplementNonTrait { supposed_trait_term: TermId },
    /// The trait implementation `trt_impl_term_id` is missing the member
    /// `trt_def_missing_member_id` from the trait `trt_def_term_id`.
    TraitImplementationMissingMember {
        trt_impl_term_id: TermId,
        trt_def_term_id: TermId,
        // @@ErrorReporting: Ideally we want to be able to identify whole members rather than just
        // "terms".
        trt_def_missing_member_term_id: TermId,
    },
}
