use crate::feature::{Meta, Subtest};
use crate::features::EsnextAsyncIteratorHelpers;
impl Meta for EsnextAsyncIteratorHelpers {
    fn name(&self) -> &'static str {
        "Async Iterator Helpers"
    }
    fn key(&self) -> &'static str {
        "esnext_async_iterator_helpers"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 2"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-async-iterator-helpers"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "instanceof AsyncIterator", exec :
            "return (async function*() {})() instanceof AsyncIterator;", }, Subtest {
            name : "extends AsyncIterator", exec :
            "class Class extends AsyncIterator { }\nconst instance = new Class();\nreturn instance[Symbol.asyncIterator]() === instance;",
            }, Subtest { name : "AsyncIterator.from, async iterable", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\nconst iterator = AsyncIterator.from(async function*() { yield * [1, 2, 3] }());\n\nif (!('next' in iterator) || !(iterator instanceof AsyncIterator)) return false;\n\ntoArray(iterator).then(it => {\n  if (it.join() === '1,2,3') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.from, iterable", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\nconst iterator = AsyncIterator.from([1, 2, 3]);\n\nif (!('next' in iterator) || !(iterator instanceof AsyncIterator)) return false;\n\ntoArray(iterator).then(it => {\n  if (it.join() === '1,2,3') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.from, iterator", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\nconst iterator = AsyncIterator.from([1, 2, 3].values());\n\nif (!('next' in iterator) || !(iterator instanceof AsyncIterator)) return false;\n\ntoArray(iterator).then(it => {\n  if (it.join() === '1,2,3') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.drop", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\ntoArray(async function*() { yield * [1, 2, 3] }().drop(1)).then(it => {\n  if (it.join() === '2,3') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.every", exec :
            "(async function*() { yield * [1, 2, 3] })().every(it => typeof it === 'number').then(it => {\n  if (it === true) asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.filter", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\ntoArray(async function*() { yield * [1, 2, 3] }().filter(it => it % 2)).then(it => {\n  if (it.join() === '1,3') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.find", exec :
            "(async function*() { yield * [1, 2, 3] })().find(it => it % 2).then(it => {\n  if (it === 1) asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.flatMap", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\ntoArray(async function*() { yield * [1, 2, 3] }().flatMap(it => [it, 0])).then(it => {\n  if (it.join() === '1,0,2,0,3,0') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.forEach", exec :
            "let result = '';\n(async function*() { yield * [1, 2, 3] })().forEach(it => result += it).then(() => {\n  if (result === '123') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.map", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\ntoArray(async function*() { yield * [1, 2, 3] }().map(it => it * it)).then(it => {\n  if (it.join() === '1,4,9') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.reduce", exec :
            "(async function*() { yield * [1, 2, 3] })().reduce((a, b) => a + b).then(it => {\n  if (it === 6) asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.some", exec :
            "(async function*() { yield * [1, 2, 3] })().some(it => typeof it === 'number').then(it => {\n  if (it === true) asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.take", exec :
            "async function toArray(iterator) {\n  const result = [];\n  for await (const it of iterator) result.push(it);\n  return result;\n}\n\ntoArray(async function*() { yield * [1, 2, 3] }().take(2)).then(it => {\n  if (it.join() === '1,2') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype.toArray", exec :
            "(async function*() { yield * [1, 2, 3] })().toArray().then(it => {\n  if (Array.isArray(it) && it.join() === '1,2,3') asyncTestPassed();\n});",
            }, Subtest { name : "AsyncIterator.prototype[@@toStringTag]", exec :
            "return AsyncIterator.prototype[Symbol.toStringTag] === 'AsyncIterator';", },
        ]
    }
}
