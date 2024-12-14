use crate::feature::Feature;
pub const RULES: &[&dyn Feature] = &[
    &ExponentiationOperator,
    &ArrayPrototypeIncludes,
    &GeneratorFunctionsCanTBeUsedWithNew,
    &GeneratorThrowCaughtByInnerGenerator,
    &StrictFnWNonStrictNonSimpleParamsIsError,
    &NestedRestDestructuringDeclarations,
    &NestedRestDestructuringParameters,
    &ProxyEnumerateHandlerRemoved,
    &ProxyInternalCallsArrayPrototypeIncludes,
    &ObjectStaticMethods,
    &StringPadding,
    &TrailingCommasInFunctionSyntax,
    &AsyncFunctions,
    &SharedMemoryAndAtomics,
    &RegExpUFlagCaseFolding,
    &ArgumentsCallerRemoved,
    &ObjectPrototypeGetterSetterMethods,
    &ProxyInternalCallsGetterSetterMethods,
    &AssignmentsAllowedInForInHeadInNonStrictMode,
    &ObjectRestSpreadProperties,
    &PromisePrototypeFinally,
    &SDotAllFlagForRegularExpressions,
    &RegExpNamedCaptureGroups,
    &RegExpLookbehindAssertions,
    &RegExpUnicodePropertyEscapes,
    &AsynchronousIterators,
    &ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets,
    &TemplateLiteralRevision,
    &SymbolPrototypeDescription,
    &ObjectFromEntries,
    &StringTrimming,
    &ArrayPrototypeFlatFlatMap,
    &OptionalCatchBinding,
    &FunctionPrototypeToStringRevision,
    &JsonSuperset,
    &WellFormedJsonStringify,
    &StringPrototypeMatchAll,
    &BigInt,
    &PromiseAllSettled,
    &GlobalThis,
    &OptionalChainingOperator,
    &NullishCoalescingOperator,
    &StringPrototypeReplaceAll,
    &PromiseAny,
    &WeakReferences,
    &LogicalAssignment,
    &NumericSeparators,
    &InstanceClassFields,
    &StaticClassFields,
    &PrivateClassMethods,
    &ErgonomicBrandChecksForPrivateFields,
    &AtMethodOnTheBuiltInIndexables,
    &ObjectHasOwn,
    &ClassStaticInitializationBlocks,
    &ErrorCauseProperty,
    &RegExpMatchIndicesHasIndicesDFlag,
    &ArrayFindFromLast,
    &HashbangGrammar,
    &ChangeArrayByCopy,
    &RegExpVFlag,
    &ArrayBufferPrototypeTransfer,
    &PromiseWithResolvers,
    &ArrayGrouping,
    &DuplicateNamedCapturingGroups,
    &SetMethods,
    &RegExpPatternModifiers,
    &IteratorHelpers,
    &PromiseTry,
];
pub struct ExponentiationOperator;
pub struct ArrayPrototypeIncludes;
pub struct GeneratorFunctionsCanTBeUsedWithNew;
pub struct GeneratorThrowCaughtByInnerGenerator;
pub struct StrictFnWNonStrictNonSimpleParamsIsError;
pub struct NestedRestDestructuringDeclarations;
pub struct NestedRestDestructuringParameters;
pub struct ProxyEnumerateHandlerRemoved;
pub struct ProxyInternalCallsArrayPrototypeIncludes;
pub struct ObjectStaticMethods;
pub struct StringPadding;
pub struct TrailingCommasInFunctionSyntax;
pub struct AsyncFunctions;
pub struct SharedMemoryAndAtomics;
pub struct RegExpUFlagCaseFolding;
pub struct ArgumentsCallerRemoved;
pub struct ObjectPrototypeGetterSetterMethods;
pub struct ProxyInternalCallsGetterSetterMethods;
pub struct AssignmentsAllowedInForInHeadInNonStrictMode;
pub struct ObjectRestSpreadProperties;
pub struct PromisePrototypeFinally;
pub struct SDotAllFlagForRegularExpressions;
pub struct RegExpNamedCaptureGroups;
pub struct RegExpLookbehindAssertions;
pub struct RegExpUnicodePropertyEscapes;
pub struct AsynchronousIterators;
pub struct ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets;
pub struct TemplateLiteralRevision;
pub struct SymbolPrototypeDescription;
pub struct ObjectFromEntries;
pub struct StringTrimming;
pub struct ArrayPrototypeFlatFlatMap;
pub struct OptionalCatchBinding;
pub struct FunctionPrototypeToStringRevision;
pub struct JsonSuperset;
pub struct WellFormedJsonStringify;
pub struct StringPrototypeMatchAll;
pub struct BigInt;
pub struct PromiseAllSettled;
pub struct GlobalThis;
pub struct OptionalChainingOperator;
pub struct NullishCoalescingOperator;
pub struct StringPrototypeReplaceAll;
pub struct PromiseAny;
pub struct WeakReferences;
pub struct LogicalAssignment;
pub struct NumericSeparators;
pub struct InstanceClassFields;
pub struct StaticClassFields;
pub struct PrivateClassMethods;
pub struct ErgonomicBrandChecksForPrivateFields;
pub struct AtMethodOnTheBuiltInIndexables;
pub struct ObjectHasOwn;
pub struct ClassStaticInitializationBlocks;
pub struct ErrorCauseProperty;
pub struct RegExpMatchIndicesHasIndicesDFlag;
pub struct ArrayFindFromLast;
pub struct HashbangGrammar;
pub struct ChangeArrayByCopy;
pub struct RegExpVFlag;
pub struct ArrayBufferPrototypeTransfer;
pub struct PromiseWithResolvers;
pub struct ArrayGrouping;
pub struct DuplicateNamedCapturingGroups;
pub struct SetMethods;
pub struct RegExpPatternModifiers;
pub struct IteratorHelpers;
pub struct PromiseTry;
