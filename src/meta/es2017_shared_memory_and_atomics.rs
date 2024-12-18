use crate::{
    feature::{Meta, Subtest},
    features::Es2017SharedMemoryAndAtomics,
};
impl Meta for Es2017SharedMemoryAndAtomics {
    fn name(&self) -> &'static str {
        "shared memory and atomics"
    }

    fn key(&self) -> &'static str {
        "es2017_shared_memory_and_atomics"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2017 features"
    }

    fn spec(&self) -> &'static str {
        ""
    }

    fn significance(&self) -> &'static str {
        "medium"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "SharedArrayBuffer",
                exec: "return typeof SharedArrayBuffer === 'function';",
            },
            Subtest {
                name: "SharedArrayBuffer[Symbol.species]",
                exec: "return SharedArrayBuffer[Symbol.species] === SharedArrayBuffer;",
            },
            Subtest {
                name: "SharedArrayBuffer.prototype.byteLength",
                exec: "return 'byteLength' in SharedArrayBuffer.prototype;",
            },
            Subtest {
                name: "SharedArrayBuffer.prototype.slice",
                exec: "return typeof SharedArrayBuffer.prototype.slice === 'function';",
            },
            Subtest {
                name: "SharedArrayBuffer.prototype[Symbol.toStringTag]",
                exec: "return SharedArrayBuffer.prototype[Symbol.toStringTag] === 'SharedArrayBuffer';",
            },
            Subtest { name: "Atomics.add", exec: "return typeof Atomics.add === 'function';" },
            Subtest { name: "Atomics.and", exec: "return typeof Atomics.and === 'function';" },
            Subtest {
                name: "Atomics.compareExchange",
                exec: "return typeof Atomics.compareExchange === 'function';",
            },
            Subtest {
                name: "Atomics.exchange",
                exec: "return typeof Atomics.exchange === 'function';",
            },
            Subtest { name: "Atomics.wait", exec: "return typeof Atomics.wait === 'function';" },
            Subtest {
                name: "Atomics.notify",
                exec: "return typeof Atomics.notify === 'function';",
            },
            Subtest {
                name: "Atomics.isLockFree",
                exec: "return typeof Atomics.isLockFree === 'function';",
            },
            Subtest { name: "Atomics.load", exec: "return typeof Atomics.load === 'function';" },
            Subtest { name: "Atomics.or", exec: "return typeof Atomics.or === 'function';" },
            Subtest { name: "Atomics.store", exec: "return typeof Atomics.store === 'function';" },
            Subtest { name: "Atomics.sub", exec: "return typeof Atomics.sub === 'function';" },
            Subtest { name: "Atomics.xor", exec: "return typeof Atomics.xor === 'function';" },
        ]
    }
}
