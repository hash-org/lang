//! Hash Error code definitions.

error_codes! {
    // Semantic errors
    ExpectingBooleanInCondition = 1,
    UsingBreakOutsideLoop = 2,
    UsingContinueOutsideLoop = 3,
    UsingReturnOutsideFn = 4,
    // 5: un-used

    // Name spacing and symbol errors
    UnresolvedSymbol = 10,
    UnsupportedAccess = 11,
    UnsupportedNamespaceAccess = 12,
    UnsupportedPropertyAccess = 13,
    AmbiguousAccess = 14,
    UnresolvedNameInValue = 15,
    InvalidPropertyAccess = 16,
    MissingStructField = 17,
    UninitialisedMember = 18,
    InvalidAssignSubject = 19,

    // Type errors
    TypeMismatch = 20,
    DisallowedType = 21,
    UnresolvedType = 22,
    TyIsNotTyFn = 23,
    ValueCannotBeUsedAsType = 24,
    NonRuntimeInstantiable = 25,
    UnsupportedTyFnApplication = 26,
    TypeIsNotTrait = 27,

    // Errors in regard to parameter lists
    ParameterLengthMismatch = 35,
    ParameterNameMismatch = 36,
    ParameterInUse = 37,
    AmbiguousFieldOrder = 38,
    InvalidCallSubject = 39,

    // traits
    InvalidMergeElement = 50,
    MultipleNominals = 51,
    TraitDefinitionNotFound = 52,
    NoMatchingTraitImplementations = 53,
    InvalidPropertyAccessOfNonMethod = 54,
    TraitImplMissingMember = 55,
    InvalidUnionElement = 56,

    // Pattern errors
    IdentifierBoundMultipleTimes = 80,
    MissingPatternBounds = 81,
}
