use crate::{
    feature::{Meta, Subtest},
    features::Es5StrictMode,
};
impl Meta for Es5StrictMode {
    fn name(&self) -> &'static str {
        "Strict mode"
    }

    fn key(&self) -> &'static str {
        "es5_strict_mode"
    }

    fn target(&self) -> &'static str {
        "es5"
    }

    fn category(&self) -> &'static str {
        ""
    }

    fn spec(&self) -> &'static str {
        ""
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Strict_mode"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "reserved words",
                exec: "'use strict';\nvar words = 'implements,interface,let,package,private,protected,public,static,yield'.split(',');\nfor (var i = 0; i < 9; i+=1) {\n  try { eval('var ' + words[i]); return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\n}\nreturn true;",
            },
            Subtest {
                name: "\"this\" is undefined in functions",
                exec: "'use strict';\nreturn this === void undefined && (function(){ return this === void undefined; }).call();",
            },
            Subtest {
                name: "\"this\" is not coerced to object in primitive methods",
                exec: "'use strict';\nreturn (function(){ return typeof this === 'string' }).call('')\n  && (function(){ return typeof this === 'number' }).call(1)\n  && (function(){ return typeof this === 'boolean' }).call(true);",
            },
            Subtest {
                name: "\"this\" is not coerced to object in primitive accessors",
                exec: "'use strict';\n\nfunction test(Class, instance) {\n  Object.defineProperty(Class.prototype, 'test', {\n    get: function () { passed = passed && this === instance; },\n    set: function () { passed = passed && this === instance; },\n    configurable: true\n  });\n\n  var passed = true;\n  instance.test;\n  instance.test = 42;\n  return passed;\n}\n\nreturn test(String, '')\n  && test(Number, 1)\n  && test(Boolean, true);",
            },
            Subtest {
                name: "legacy octal is a SyntaxError",
                exec: "'use strict';\ntry { eval('010');     return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('\"\\\\010\"'); return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\nreturn true;",
            },
            Subtest {
                name: "assignment to unresolvable identifiers is a ReferenceError",
                exec: "'use strict';\ntry { eval('__i_dont_exist = 1'); } catch (err) { return err instanceof ReferenceError; }",
            },
            Subtest {
                name: "assignment to eval or arguments is a SyntaxError",
                exec: "'use strict';\ntry { eval('eval = 1');      return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('arguments = 1'); return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('eval++');        return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('arguments++');   return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\nreturn true;",
            },
            Subtest {
                name: "assignment to non-writable properties is a TypeError",
                exec: "'use strict';\ntry { Object.defineProperty({},\"x\",{ writable: false }).x = 1; return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\ntry { Object.preventExtensions({}).x = 1;                      return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\ntry { ({ get x(){ } }).x = 1;                                  return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\ntry { (function f() { f = 123; })();                           return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\nreturn true;",
            },
            Subtest {
                name: "eval or arguments bindings is a SyntaxError",
                exec: "'use strict';\ntry { eval('var eval');                return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('var arguments');           return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('(function(eval){})');      return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('(function(arguments){})'); return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('try{}catch(eval){}');      return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\ntry { eval('try{}catch(arguments){}'); return false; } catch (err) { if (!(err instanceof SyntaxError)) return false; }\nreturn true;",
            },
            Subtest {
                name: "arguments.caller removed or is a TypeError",
                exec: "'use strict';\nif ('caller' in arguments) {\n  try { arguments.caller; return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\n}\nreturn true;",
            },
            Subtest {
                name: "arguments.callee is a TypeError",
                exec: "'use strict';\ntry { arguments.callee; return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\nreturn true;",
            },
            Subtest {
                name: "(function(){}).caller and (function(){}).arguments is a TypeError",
                exec: "'use strict';\ntry { (function(){}).caller;    return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\ntry { (function(){}).arguments; return false; } catch (err) { if (!(err instanceof TypeError)) return false; }\nreturn true;",
            },
            Subtest {
                name: "arguments is unmapped",
                exec: "'use strict';\nreturn (function(x){\n  x = 2;\n  return arguments[0] === 1;\n})(1) && (function(x){\n  arguments[0] = 2;\n  return x === 1;\n})(1);",
            },
            Subtest {
                name: "eval() can't create bindings",
                exec: "'use strict';\ntry { eval('var __some_unique_variable;'); __some_unique_variable; } catch (err) { return err instanceof ReferenceError; }",
            },
            Subtest {
                name: "deleting bindings is a SyntaxError",
                exec: "'use strict';\ntry { eval('var x; delete x;'); } catch (err) { return err instanceof SyntaxError; }",
            },
            Subtest {
                name: "deleting non-configurable properties is a TypeError",
                exec: "'use strict';\ntry { delete Object.prototype; } catch (err) { return err instanceof TypeError; }",
            },
            Subtest {
                name: "\"with\" is a SyntaxError",
                exec: "'use strict';\ntry { eval('with({}){}'); } catch (err) { return err instanceof SyntaxError; }",
            },
            Subtest {
                name: "repeated parameter names is a SyntaxError",
                exec: "'use strict';\ntry { eval('function f(x, x) { }'); } catch (err) { return err instanceof SyntaxError; }",
            },
            Subtest {
                name: "function expressions with matching name and argument are valid",
                exec: "var foo = function bar(bar) {'use strict'};\nreturn typeof foo === 'function';",
            },
        ]
    }
}
