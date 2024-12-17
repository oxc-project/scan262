use crate::feature::Feature;
pub const FEATURES: &[&dyn Feature] = &[
    &Es5ObjectArrayLiteralExtensions,
    &Es5ObjectStaticMethods,
    &Es5ArrayMethods,
    &Es5StringPropertiesAndMethods,
    &Es5DateMethods,
    &Es5FunctionPrototypeBind,
    &Es5Json,
    &Es5ImmutableGlobals,
    &Es5NumberMethods,
    &Es5Miscellaneous,
    &Es5StrictMode,
    &Es6ProperTailCallsTailCallOptimisation,
    &Es6DefaultFunctionParameters,
    &Es6RestParameters,
    &Es6SpreadSyntaxForIterableObjects,
    &Es6ObjectLiteralExtensions,
    &Es6ForOfLoops,
    &Es6OctalAndBinaryLiterals,
    &Es6TemplateLiterals,
    &Es6RegExpYAndUFlags,
    &Es6DestructuringDeclarations,
    &Es6DestructuringAssignment,
    &Es6DestructuringParameters,
    &Es6UnicodeCodePointEscapes,
    &Es6NewTarget,
    &Es6Const,
    &Es6Let,
    &Es6BlockLevelFunctionDeclaration,
    &Es6ArrowFunctions,
    &Es6Class,
    &Es6Super,
    &Es6Generators,
    &Es6TypedArrays,
    &Es6Map,
    &Es6Set,
    &Es6WeakMap,
    &Es6WeakSet,
    &Es6Proxy,
    &Es6Reflect,
    &Es6Promise,
    &Es6Symbol,
    &Es6WellKnownSymbols,
    &Es6ObjectStaticMethods,
    &Es6FunctionNameProperty,
    &Es6StringStaticMethods,
    &Es6StringPrototypeMethods,
    &Es6RegExpPrototypeProperties,
    &Es6ArrayStaticMethods,
    &Es6ArrayPrototypeMethods,
    &Es6NumberProperties,
    &Es6MathMethods,
    &Es6DatePrototypeSymbolToPrimitive,
    &Es6ArrayIsSubclassable,
    &Es6RegExpIsSubclassable,
    &Es6FunctionIsSubclassable,
    &Es6PromiseIsSubclassable,
    &Es6MiscellaneousSubclassables,
    &Es6PrototypeOfBoundFunctions,
    &Es6ProxyInternalGetCalls,
    &Es6ProxyInternalSetCalls,
    &Es6ProxyInternalDefinePropertyCalls,
    &Es6ProxyInternalDeletePropertyCalls,
    &Es6ProxyInternalGetOwnPropertyDescriptorCalls,
    &Es6ProxyInternalOwnKeysCalls,
    &Es6ObjectStaticMethodsAcceptPrimitives,
    &Es6OwnPropertyOrder,
    &Es6UpdatedIdentifierSyntax,
    &Es6Miscellaneous,
    &Es6NonStrictFunctionSemantics,
    &Es6ProtoInObjectLiterals,
    &Es6ObjectPrototypeProto,
    &Es6StringPrototypeHtmlMethods,
    &Es6RegExpPrototypeCompile,
    &Es6RegExpSyntaxExtensions,
    &Es6HtmlStyleComments,
    &Es2016ExponentiationOperator,
    &Es2016ArrayPrototypeIncludes,
    &Es2016GeneratorFunctionsCanTBeUsedWithNew,
    &Es2016GeneratorThrowCaughtByInnerGenerator,
    &Es2016StrictFnWNonStrictNonSimpleParamsIsError,
    &Es2016NestedRestDestructuringDeclarations,
    &Es2016NestedRestDestructuringParameters,
    &Es2016ProxyEnumerateHandlerRemoved,
    &Es2016ProxyInternalCallsArrayPrototypeIncludes,
    &Es2017ObjectStaticMethods,
    &Es2017StringPadding,
    &Es2017TrailingCommasInFunctionSyntax,
    &Es2017AsyncFunctions,
    &Es2017SharedMemoryAndAtomics,
    &Es2017RegExpUFlagCaseFolding,
    &Es2017ArgumentsCallerRemoved,
    &Es2017ObjectPrototypeGetterSetterMethods,
    &Es2017ProxyInternalCallsGetterSetterMethods,
    &Es2017AssignmentsAllowedInForInHeadInNonStrictMode,
    &Es2018ObjectRestSpreadProperties,
    &Es2018PromisePrototypeFinally,
    &Es2018SDotAllFlagForRegularExpressions,
    &Es2018RegExpNamedCaptureGroups,
    &Es2018RegExpLookbehindAssertions,
    &Es2018RegExpUnicodePropertyEscapes,
    &Es2018AsynchronousIterators,
    &Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets,
    &Es2018TemplateLiteralRevision,
    &Es2019SymbolPrototypeDescription,
    &Es2019ObjectFromEntries,
    &Es2019StringTrimming,
    &Es2019ArrayPrototypeFlatFlatMap,
    &Es2019OptionalCatchBinding,
    &Es2019FunctionPrototypeToStringRevision,
    &Es2019JsonSuperset,
    &Es2019WellFormedJsonStringify,
    &Es2020StringPrototypeMatchAll,
    &Es2020BigInt,
    &Es2020PromiseAllSettled,
    &Es2020GlobalThis,
    &Es2020OptionalChainingOperator,
    &Es2020NullishCoalescingOperator,
    &Es2021StringPrototypeReplaceAll,
    &Es2021PromiseAny,
    &Es2021WeakReferences,
    &Es2021LogicalAssignment,
    &Es2021NumericSeparators,
    &Es2022InstanceClassFields,
    &Es2022StaticClassFields,
    &Es2022PrivateClassMethods,
    &Es2022ErgonomicBrandChecksForPrivateFields,
    &Es2022AtMethodOnTheBuiltInIndexables,
    &Es2022ObjectHasOwn,
    &Es2022ClassStaticInitializationBlocks,
    &Es2022ErrorCauseProperty,
    &Es2022RegExpMatchIndicesHasIndicesDFlag,
    &Es2023ArrayFindFromLast,
    &Es2023HashbangGrammar,
    &Es2023ChangeArrayByCopy,
    &Es2024RegExpVFlag,
    &Es2024ArrayBufferPrototypeTransfer,
    &Es2024PromiseWithResolvers,
    &Es2024ArrayGrouping,
    &Es2025DuplicateNamedCapturingGroups,
    &Es2025SetMethods,
    &Es2025RegExpPatternModifiers,
    &Es2025IteratorHelpers,
    &Es2025PromiseTry,
    &EsnextClassAndPropertyDecorators,
    &EsnextLegacyRegExpFeaturesInJavaScript,
    &EsnextRegExpEscaping,
    &EsnextUintArrayToFromBaseAndHex,
    &EsnextShadowRealm,
    &EsnextGeneratorFunctionSentMetaProperty,
    &EsnextThrowExpressions,
    &EsnextMapPrototypeUpsert,
    &EsnextArrayIsTemplateObject,
    &EsnextAsyncIteratorHelpers,
];
pub struct Es5ObjectArrayLiteralExtensions;
pub struct Es5ObjectStaticMethods;
pub struct Es5ArrayMethods;
pub struct Es5StringPropertiesAndMethods;
pub struct Es5DateMethods;
pub struct Es5FunctionPrototypeBind;
pub struct Es5Json;
pub struct Es5ImmutableGlobals;
pub struct Es5NumberMethods;
pub struct Es5Miscellaneous;
pub struct Es5StrictMode;
pub struct Es6ProperTailCallsTailCallOptimisation;
pub struct Es6DefaultFunctionParameters;
pub struct Es6RestParameters;
pub struct Es6SpreadSyntaxForIterableObjects;
pub struct Es6ObjectLiteralExtensions;
pub struct Es6ForOfLoops;
pub struct Es6OctalAndBinaryLiterals;
pub struct Es6TemplateLiterals;
pub struct Es6RegExpYAndUFlags;
pub struct Es6DestructuringDeclarations;
pub struct Es6DestructuringAssignment;
pub struct Es6DestructuringParameters;
pub struct Es6UnicodeCodePointEscapes;
pub struct Es6NewTarget;
pub struct Es6Const;
pub struct Es6Let;
pub struct Es6BlockLevelFunctionDeclaration;
pub struct Es6ArrowFunctions;
pub struct Es6Class;
pub struct Es6Super;
pub struct Es6Generators;
pub struct Es6TypedArrays;
pub struct Es6Map;
pub struct Es6Set;
pub struct Es6WeakMap;
pub struct Es6WeakSet;
pub struct Es6Proxy;
pub struct Es6Reflect;
pub struct Es6Promise;
pub struct Es6Symbol;
pub struct Es6WellKnownSymbols;
pub struct Es6ObjectStaticMethods;
pub struct Es6FunctionNameProperty;
pub struct Es6StringStaticMethods;
pub struct Es6StringPrototypeMethods;
pub struct Es6RegExpPrototypeProperties;
pub struct Es6ArrayStaticMethods;
pub struct Es6ArrayPrototypeMethods;
pub struct Es6NumberProperties;
pub struct Es6MathMethods;
pub struct Es6DatePrototypeSymbolToPrimitive;
pub struct Es6ArrayIsSubclassable;
pub struct Es6RegExpIsSubclassable;
pub struct Es6FunctionIsSubclassable;
pub struct Es6PromiseIsSubclassable;
pub struct Es6MiscellaneousSubclassables;
pub struct Es6PrototypeOfBoundFunctions;
pub struct Es6ProxyInternalGetCalls;
pub struct Es6ProxyInternalSetCalls;
pub struct Es6ProxyInternalDefinePropertyCalls;
pub struct Es6ProxyInternalDeletePropertyCalls;
pub struct Es6ProxyInternalGetOwnPropertyDescriptorCalls;
pub struct Es6ProxyInternalOwnKeysCalls;
pub struct Es6ObjectStaticMethodsAcceptPrimitives;
pub struct Es6OwnPropertyOrder;
pub struct Es6UpdatedIdentifierSyntax;
pub struct Es6Miscellaneous;
pub struct Es6NonStrictFunctionSemantics;
pub struct Es6ProtoInObjectLiterals;
pub struct Es6ObjectPrototypeProto;
pub struct Es6StringPrototypeHtmlMethods;
pub struct Es6RegExpPrototypeCompile;
pub struct Es6RegExpSyntaxExtensions;
pub struct Es6HtmlStyleComments;
pub struct Es2016ExponentiationOperator;
pub struct Es2016ArrayPrototypeIncludes;
pub struct Es2016GeneratorFunctionsCanTBeUsedWithNew;
pub struct Es2016GeneratorThrowCaughtByInnerGenerator;
pub struct Es2016StrictFnWNonStrictNonSimpleParamsIsError;
pub struct Es2016NestedRestDestructuringDeclarations;
pub struct Es2016NestedRestDestructuringParameters;
pub struct Es2016ProxyEnumerateHandlerRemoved;
pub struct Es2016ProxyInternalCallsArrayPrototypeIncludes;
pub struct Es2017ObjectStaticMethods;
pub struct Es2017StringPadding;
pub struct Es2017TrailingCommasInFunctionSyntax;
pub struct Es2017AsyncFunctions;
pub struct Es2017SharedMemoryAndAtomics;
pub struct Es2017RegExpUFlagCaseFolding;
pub struct Es2017ArgumentsCallerRemoved;
pub struct Es2017ObjectPrototypeGetterSetterMethods;
pub struct Es2017ProxyInternalCallsGetterSetterMethods;
pub struct Es2017AssignmentsAllowedInForInHeadInNonStrictMode;
pub struct Es2018ObjectRestSpreadProperties;
pub struct Es2018PromisePrototypeFinally;
pub struct Es2018SDotAllFlagForRegularExpressions;
pub struct Es2018RegExpNamedCaptureGroups;
pub struct Es2018RegExpLookbehindAssertions;
pub struct Es2018RegExpUnicodePropertyEscapes;
pub struct Es2018AsynchronousIterators;
pub struct Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets;
pub struct Es2018TemplateLiteralRevision;
pub struct Es2019SymbolPrototypeDescription;
pub struct Es2019ObjectFromEntries;
pub struct Es2019StringTrimming;
pub struct Es2019ArrayPrototypeFlatFlatMap;
pub struct Es2019OptionalCatchBinding;
pub struct Es2019FunctionPrototypeToStringRevision;
pub struct Es2019JsonSuperset;
pub struct Es2019WellFormedJsonStringify;
pub struct Es2020StringPrototypeMatchAll;
pub struct Es2020BigInt;
pub struct Es2020PromiseAllSettled;
pub struct Es2020GlobalThis;
pub struct Es2020OptionalChainingOperator;
pub struct Es2020NullishCoalescingOperator;
pub struct Es2021StringPrototypeReplaceAll;
pub struct Es2021PromiseAny;
pub struct Es2021WeakReferences;
pub struct Es2021LogicalAssignment;
pub struct Es2021NumericSeparators;
pub struct Es2022InstanceClassFields;
pub struct Es2022StaticClassFields;
pub struct Es2022PrivateClassMethods;
pub struct Es2022ErgonomicBrandChecksForPrivateFields;
pub struct Es2022AtMethodOnTheBuiltInIndexables;
pub struct Es2022ObjectHasOwn;
pub struct Es2022ClassStaticInitializationBlocks;
pub struct Es2022ErrorCauseProperty;
pub struct Es2022RegExpMatchIndicesHasIndicesDFlag;
pub struct Es2023ArrayFindFromLast;
pub struct Es2023HashbangGrammar;
pub struct Es2023ChangeArrayByCopy;
pub struct Es2024RegExpVFlag;
pub struct Es2024ArrayBufferPrototypeTransfer;
pub struct Es2024PromiseWithResolvers;
pub struct Es2024ArrayGrouping;
pub struct Es2025DuplicateNamedCapturingGroups;
pub struct Es2025SetMethods;
pub struct Es2025RegExpPatternModifiers;
pub struct Es2025IteratorHelpers;
pub struct Es2025PromiseTry;
pub struct EsnextClassAndPropertyDecorators;
pub struct EsnextLegacyRegExpFeaturesInJavaScript;
pub struct EsnextRegExpEscaping;
pub struct EsnextUintArrayToFromBaseAndHex;
pub struct EsnextShadowRealm;
pub struct EsnextGeneratorFunctionSentMetaProperty;
pub struct EsnextThrowExpressions;
pub struct EsnextMapPrototypeUpsert;
pub struct EsnextArrayIsTemplateObject;
pub struct EsnextAsyncIteratorHelpers;
