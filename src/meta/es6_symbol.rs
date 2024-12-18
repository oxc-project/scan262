use crate::{
    feature::{Meta, Subtest},
    features::Es6Symbol,
};
impl Meta for Es6Symbol {
    fn name(&self) -> &'static str {
        "Symbol"
    }

    fn key(&self) -> &'static str {
        "es6_symbol"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-ins"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-symbol-constructor"
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "basic functionality",
                exec: "var object = {};\nvar symbol = Symbol();\nvar value = {};\nobject[symbol] = value;\nreturn object[symbol] === value;",
            },
            Subtest { name: "typeof support", exec: "return typeof Symbol() === \"symbol\";" },
            Subtest {
                name: "symbol keys are hidden to pre-ES6 code",
                exec: "var object = {};\nvar symbol = Symbol();\nobject[symbol] = 1;\n\nfor (var x in object){}\nvar passed = !x;\n\nif (Object.keys && Object.getOwnPropertyNames) {\n  passed &= Object.keys(object).length === 0\n    && Object.getOwnPropertyNames(object).length === 0;\n}\n\nreturn passed;",
            },
            Subtest {
                name: "Object.defineProperty support",
                exec: "var object = {};\nvar symbol = Symbol();\nvar value = {};\n\nif (Object.defineProperty) {\n  Object.defineProperty(object, symbol, { value: value });\n  return object[symbol] === value;\n}\n\nreturn passed;",
            },
            Subtest {
                name: "symbols inherit from Symbol.prototype",
                exec: "var symbol = Symbol();\nvar passed = symbol.foo === void undefined;\nSymbol.prototype.foo = 2;\npassed &= symbol.foo === 2;\ndelete Symbol.prototype.foo;\nreturn passed;",
            },
            Subtest {
                name: "cannot coerce to string or number",
                exec: "var symbol = Symbol();\n\ntry {\n  symbol + \"\";\n  return false;\n}\ncatch(e) {}\n\ntry {\n  symbol + 0;\n  return false;\n} catch(e) {}\n\nreturn true;",
            },
            Subtest {
                name: "can convert with String()",
                exec: "return String(Symbol(\"foo\")) === \"Symbol(foo)\";",
            },
            Subtest {
                name: "new Symbol() throws",
                exec: "var symbol = Symbol();\ntry {\n  new Symbol();\n} catch(e) {\n  return true;\n}",
            },
            Subtest {
                name: "Object(symbol)",
                exec: "var symbol = Symbol();\nvar symbolObject = Object(symbol);\n\nreturn typeof symbolObject === \"object\" &&\n  symbolObject instanceof Symbol &&\n  symbolObject == symbol && // eslint-disable-line eqeqeq\n  symbolObject !== symbol &&\n  symbolObject.valueOf() === symbol;",
            },
            Subtest {
                name: "JSON.stringify ignores symbol primitives",
                exec: "var object = { foo: Symbol() };\nobject[Symbol()] = 1;\nvar array = [Symbol()];\nreturn JSON.stringify(object) === '{}' && JSON.stringify(array) === '[null]' && JSON.stringify(Symbol()) === void undefined;",
            },
            Subtest {
                name: "JSON.stringify ignores symbol objects",
                exec: "var testSymbolObject = function (sym) {\n  var object = { foo: sym };\n  try {\n    // some browsers throw a TypeError when setting symbol object keys.\n    // this isn't part of this test, so, ignore it if so.\n    object[sym] = 1;\n  } catch (e) {} // some browsers throw a TypeError when setting symbol object keys.\n  var array = [sym];\n  return JSON.stringify(object) === '{\"foo\":{}}' && JSON.stringify(array) === '[{}]' && JSON.stringify(sym) === '{}';\n};\nvar objSym = Object(Symbol());\nvar symNoToJSON = Object(Symbol());\nObject.defineProperty(symNoToJSON, 'toJSON', { enumerable: false, value: null }); // ensure it overrides the prototype, but is not callable\nreturn testSymbolObject(objSym) && testSymbolObject(symNoToJSON);",
            },
            Subtest {
                name: "global symbol registry",
                exec: "var symbol = Symbol.for('foo');\nreturn Symbol.for('foo') === symbol &&\n   Symbol.keyFor(symbol) === 'foo';",
            },
        ]
    }
}
