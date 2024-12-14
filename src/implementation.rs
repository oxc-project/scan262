use oxc::{semantic::AstNode, span::Span};

use crate::{feature::Feature, features::*};

impl Feature for ExponentiationOperator {
    fn name(&self) -> &'static str {
        "exponentiation (**) operator"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ArrayPrototypeIncludes {
    fn name(&self) -> &'static str {
        "Array.prototype.includes"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for GeneratorFunctionsCanTBeUsedWithNew {
    fn name(&self) -> &'static str {
        "generator functions can't be used with \"new\""
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for GeneratorThrowCaughtByInnerGenerator {
    fn name(&self) -> &'static str {
        "generator throw() caught by inner generator"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for StrictFnWNonStrictNonSimpleParamsIsError {
    fn name(&self) -> &'static str {
        "strict fn w/ non-strict non-simple params is error"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for NestedRestDestructuringDeclarations {
    fn name(&self) -> &'static str {
        "nested rest destructuring, declarations"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for NestedRestDestructuringParameters {
    fn name(&self) -> &'static str {
        "nested rest destructuring, parameters"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ProxyEnumerateHandlerRemoved {
    fn name(&self) -> &'static str {
        "Proxy, \"enumerate\" handler removed"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ProxyInternalCallsArrayPrototypeIncludes {
    fn name(&self) -> &'static str {
        "Proxy internal calls, Array.prototype.includes"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ObjectStaticMethods {
    fn name(&self) -> &'static str {
        "Object static methods"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for StringPadding {
    fn name(&self) -> &'static str {
        "String padding"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for TrailingCommasInFunctionSyntax {
    fn name(&self) -> &'static str {
        "trailing commas in function syntax"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for AsyncFunctions {
    fn name(&self) -> &'static str {
        "async functions"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for SharedMemoryAndAtomics {
    fn name(&self) -> &'static str {
        "shared memory and atomics"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpUFlagCaseFolding {
    fn name(&self) -> &'static str {
        "RegExp \"u\" flag, case folding"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ArgumentsCallerRemoved {
    fn name(&self) -> &'static str {
        "arguments.caller removed"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ObjectPrototypeGetterSetterMethods {
    fn name(&self) -> &'static str {
        "Object.prototype getter/setter methods"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ProxyInternalCallsGetterSetterMethods {
    fn name(&self) -> &'static str {
        "Proxy internal calls, getter/setter methods"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for AssignmentsAllowedInForInHeadInNonStrictMode {
    fn name(&self) -> &'static str {
        "assignments allowed in for-in head in non-strict mode"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ObjectRestSpreadProperties {
    fn name(&self) -> &'static str {
        "object rest/spread properties"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for PromisePrototypeFinally {
    fn name(&self) -> &'static str {
        "Promise.prototype.finally"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for SDotAllFlagForRegularExpressions {
    fn name(&self) -> &'static str {
        "s (dotAll) flag for regular expressions"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpNamedCaptureGroups {
    fn name(&self) -> &'static str {
        "RegExp named capture groups"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpLookbehindAssertions {
    fn name(&self) -> &'static str {
        "RegExp Lookbehind Assertions"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpUnicodePropertyEscapes {
    fn name(&self) -> &'static str {
        "RegExp Unicode Property Escapes"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for AsynchronousIterators {
    fn name(&self) -> &'static str {
        "Asynchronous Iterators"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets {
    fn name(&self) -> &'static str {
        "Proxy \"ownKeys\" handler, duplicate keys for non-extensible targets"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for TemplateLiteralRevision {
    fn name(&self) -> &'static str {
        "template literal revision"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for SymbolPrototypeDescription {
    fn name(&self) -> &'static str {
        "Symbol.prototype.description"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ObjectFromEntries {
    fn name(&self) -> &'static str {
        "Object.fromEntries"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for StringTrimming {
    fn name(&self) -> &'static str {
        "string trimming"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ArrayPrototypeFlatFlatMap {
    fn name(&self) -> &'static str {
        "Array.prototype.{flat, flatMap}"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for OptionalCatchBinding {
    fn name(&self) -> &'static str {
        "optional catch binding"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for FunctionPrototypeToStringRevision {
    fn name(&self) -> &'static str {
        "Function.prototype.toString revision"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for JsonSuperset {
    fn name(&self) -> &'static str {
        "JSON superset"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for WellFormedJsonStringify {
    fn name(&self) -> &'static str {
        "Well-formed JSON.stringify"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for StringPrototypeMatchAll {
    fn name(&self) -> &'static str {
        "String.prototype.matchAll"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for BigInt {
    fn name(&self) -> &'static str {
        "BigInt"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for PromiseAllSettled {
    fn name(&self) -> &'static str {
        "Promise.allSettled"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for GlobalThis {
    fn name(&self) -> &'static str {
        "globalThis"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for OptionalChainingOperator {
    fn name(&self) -> &'static str {
        "optional chaining operator (?.)"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for NullishCoalescingOperator {
    fn name(&self) -> &'static str {
        "nullish coalescing operator (??)"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for StringPrototypeReplaceAll {
    fn name(&self) -> &'static str {
        "String.prototype.replaceAll"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for PromiseAny {
    fn name(&self) -> &'static str {
        "Promise.any"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for WeakReferences {
    fn name(&self) -> &'static str {
        "WeakReferences"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for LogicalAssignment {
    fn name(&self) -> &'static str {
        "Logical Assignment"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for NumericSeparators {
    fn name(&self) -> &'static str {
        "numeric separators"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for InstanceClassFields {
    fn name(&self) -> &'static str {
        "instance class fields"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for StaticClassFields {
    fn name(&self) -> &'static str {
        "static class fields"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for PrivateClassMethods {
    fn name(&self) -> &'static str {
        "private class methods"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ErgonomicBrandChecksForPrivateFields {
    fn name(&self) -> &'static str {
        "Ergonomic brand checks for private fields"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for AtMethodOnTheBuiltInIndexables {
    fn name(&self) -> &'static str {
        ".at() method on the built-in indexables"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ObjectHasOwn {
    fn name(&self) -> &'static str {
        "Object.hasOwn"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ClassStaticInitializationBlocks {
    fn name(&self) -> &'static str {
        "Class static initialization blocks"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ErrorCauseProperty {
    fn name(&self) -> &'static str {
        "Error.cause property"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpMatchIndicesHasIndicesDFlag {
    fn name(&self) -> &'static str {
        "RegExp Match Indices (`hasIndices` / `d` flag)"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ArrayFindFromLast {
    fn name(&self) -> &'static str {
        "Array find from last"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for HashbangGrammar {
    fn name(&self) -> &'static str {
        "Hashbang Grammar"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ChangeArrayByCopy {
    fn name(&self) -> &'static str {
        "Change Array by copy"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpVFlag {
    fn name(&self) -> &'static str {
        "RegExp `v` flag"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ArrayBufferPrototypeTransfer {
    fn name(&self) -> &'static str {
        "ArrayBuffer.prototype.transfer"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for PromiseWithResolvers {
    fn name(&self) -> &'static str {
        "Promise.withResolvers"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for ArrayGrouping {
    fn name(&self) -> &'static str {
        "Array Grouping"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for DuplicateNamedCapturingGroups {
    fn name(&self) -> &'static str {
        "Duplicate named capturing groups"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for SetMethods {
    fn name(&self) -> &'static str {
        "Set methods"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for RegExpPatternModifiers {
    fn name(&self) -> &'static str {
        "RegExp Pattern Modifiers"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for IteratorHelpers {
    fn name(&self) -> &'static str {
        "Iterator Helpers"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}

impl Feature for PromiseTry {
    fn name(&self) -> &'static str {
        "Promise.try"
    }

    fn test(&self, _node: &AstNode<'_>) -> Option<Span> {
        None
    }
}
