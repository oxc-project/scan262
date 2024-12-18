use crate::feature::{Meta, Subtest};
use crate::features::EsnextUintArrayToFromBaseAndHex;
impl Meta for EsnextUintArrayToFromBaseAndHex {
    fn name(&self) -> &'static str {
        "Uint8Array to/from base64 and hex"
    }
    fn key(&self) -> &'static str {
        "esnext_uint_array_to_from_base_and_hex"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 3"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-arraybuffer-base64"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Uint8Array.toBase64()", exec :
            "const arr = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);\nreturn arr.toBase64() === \"SGVsbG8gV29ybGQ=\";",
            }, Subtest { name : "Uint8Array.fromBase64()", exec :
            "const arr1 = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);\nconst arr2 = Uint8Array.fromBase64(\"SGVsbG8gV29ybGQ=\");\nreturn arr1.length === arr2.length &&\n       arr1.every((element, index) => element === arr2[index]);",
            }, Subtest { name : "Uint8Array.setFromBase64()", exec :
            "const arr1 = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);\nlet arr2 = new Uint8Array(16);\nlet { read, written } = arr2.setFromBase64(\"SGVsbG8gV29ybGQ=\");\nreturn read == 16 && written == 11 &&\n       arr1.every((element, index) => element === arr2[index]);",
            }, Subtest { name : "Uint8Array.toHex()", exec :
            "const arr = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);\nreturn arr.toHex() === \"48656c6c6f20576f726c64\";",
            }, Subtest { name : "Uint8Array.fromHex()", exec :
            "const arr1 = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);\nconst arr2 = Uint8Array.fromHex(\"48656c6c6f20576f726c64\");\nreturn arr1.length === arr2.length &&\n       arr1.every((element, index) => element === arr2[index]);",
            }, Subtest { name : "Uint8Array.setFromHex()", exec :
            "const arr1 = new Uint8Array([72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);\nlet arr2 = new Uint8Array(16);\nlet { read, written } = arr2.setFromHex(\"48656c6c6f20576f726c64\");\nreturn read == 22 && written == 11 &&\n       arr1.every((element, index) => element === arr2[index]);",
            },
        ]
    }
}
