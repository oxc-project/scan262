use crate::feature::{Meta, Subtest};
use crate::features::Es2024ArrayBufferPrototypeTransfer;
impl Meta for Es2024ArrayBufferPrototypeTransfer {
    fn name(&self) -> &'static str {
        "ArrayBuffer.prototype.transfer"
    }
    fn key(&self) -> &'static str {
        "es2024_array_buffer_prototype_transfer"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2024 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-arraybuffer-transfer"
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
            Subtest { name : "ArrayBuffer.prototype.transfer()", exec :
            "const buffer1 = new Uint8Array([1, 2]).buffer;\nconst buffer2 = buffer1.transfer();\nreturn buffer1.byteLength === 0\n  && buffer2.byteLength === 2;",
            }, Subtest { name : "ArrayBuffer.prototype.transferToFixedLength()", exec :
            "const buffer1 = new Uint8Array([1, 2]).buffer;\nconst buffer2 = buffer1.transferToFixedLength();\nreturn buffer1.byteLength === 0\n  && buffer2.byteLength === 2;",
            }, Subtest { name : "ArrayBuffer.prototype.detached", exec :
            "const buffer1 = new Uint8Array([1, 2]).buffer;\nconst buffer2 = buffer1.transfer();\nreturn buffer1.detached && !buffer2.detached;",
            },
        ]
    }
}
