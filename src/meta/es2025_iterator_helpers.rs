use crate::feature::{Meta, Subtest};
use crate::features::Es2025IteratorHelpers;
impl Meta for Es2025IteratorHelpers {
    fn name(&self) -> &'static str {
        "Iterator Helpers"
    }
    fn key(&self) -> &'static str {
        "es2025_iterator_helpers"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2025 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-iterator-helpers"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator#iterator_helpers"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "instanceof Iterator", exec :
            "return [1, 2, 3].values() instanceof Iterator;" }, Subtest { name :
            "extends Iterator", exec :
            "class Class extends Iterator { }\nconst instance = new Class();\nreturn instance[Symbol.iterator]() === instance;"
            }, Subtest { name : "Iterator.from, iterable", exec :
            "const iterator = Iterator.from([1, 2, 3]);\nreturn 'next' in iterator\n  && iterator instanceof Iterator\n  && Array.from(iterator).join() === '1,2,3';"
            }, Subtest { name : "Iterator.from, iterator", exec :
            "const iterator = Iterator.from({\n  i: 0,\n  next() {\n    return { value: ++this.i, done: this.i > 3 };\n  }\n});\nreturn 'next' in iterator\n  && iterator instanceof Iterator\n  && Array.from(iterator).join() === '1,2,3';"
            }, Subtest { name : "Iterator.prototype.drop", exec :
            "return Array.from([1, 2, 3].values().drop(1)).join() === '2,3';" }, Subtest
            { name : "Iterator.prototype.every", exec :
            "return [1, 2, 3].values().every(it => typeof it === 'number');" }, Subtest {
            name : "Iterator.prototype.filter", exec :
            "return Array.from([1, 2, 3].values().filter(it => it % 2)).join() === '1,3';"
            }, Subtest { name : "Iterator.prototype.find", exec :
            "return [1, 2, 3].values().find(it => it % 2) === 1;" }, Subtest { name :
            "Iterator.prototype.flatMap", exec :
            "return Array.from([1, 2, 3].values().flatMap(it => [it, 0])).join() === '1,0,2,0,3,0';"
            }, Subtest { name : "Iterator.prototype.forEach", exec :
            "let result = '';\n[1, 2, 3].values().forEach(it => result += it);\nreturn result === '123';"
            }, Subtest { name : "Iterator.prototype.map", exec :
            "return Array.from([1, 2, 3].values().map(it => it * it)).join() === '1,4,9';"
            }, Subtest { name : "Iterator.prototype.reduce", exec :
            "return [1, 2, 3].values().reduce((a, b) => a + b) === 6;" }, Subtest { name
            : "Iterator.prototype.some", exec :
            "return [1, 2, 3].values().some(it => typeof it === 'number');" }, Subtest {
            name : "Iterator.prototype.take", exec :
            "return Array.from([1, 2, 3].values().take(2)).join() === '1,2';" }, Subtest
            { name : "Iterator.prototype.toArray", exec :
            "const array = [1, 2, 3].values().toArray();\nreturn Array.isArray(array) && array.join() === '1,2,3';"
            }, Subtest { name : "Iterator.prototype[@@toStringTag]", exec :
            "return Iterator.prototype[Symbol.toStringTag] === 'Iterator';" },
        ]
    }
}
