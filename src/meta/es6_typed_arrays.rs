use crate::feature::{Meta, Subtest};
use crate::features::Es6TypedArrays;
impl Meta for Es6TypedArrays {
    fn name(&self) -> &'static str {
        "typed arrays"
    }
    fn key(&self) -> &'static str {
        "es6_typed_arrays"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-typedarray-objects"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Int8Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Int8Array(buffer);         view[0] = 0x80;\nreturn view[0] === -0x80;"
            }, Subtest { name : "Uint8Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Uint8Array(buffer);        view[0] = 0x100;\nreturn view[0] === 0;"
            }, Subtest { name : "Uint8ClampedArray", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Uint8ClampedArray(buffer); view[0] = 0x100;\nreturn view[0] === 0xFF;"
            }, Subtest { name : "Int16Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Int16Array(buffer);        view[0] = 0x8000;\nreturn view[0] === -0x8000;"
            }, Subtest { name : "Uint16Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Uint16Array(buffer);       view[0] = 0x10000;\nreturn view[0] === 0;"
            }, Subtest { name : "Int32Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Int32Array(buffer);        view[0] = 0x80000000;\nreturn view[0] === -0x80000000;"
            }, Subtest { name : "Uint32Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Uint32Array(buffer);       view[0] = 0x100000000;\nreturn view[0] === 0;"
            }, Subtest { name : "Float32Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Float32Array(buffer);       view[0] = 0.1;\nreturn view[0] === 0.10000000149011612;"
            }, Subtest { name : "Float64Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new Float64Array(buffer);       view[0] = 0.1;\nreturn view[0] === 0.1;"
            }, Subtest { name : "DataView (Int8)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setInt8 (0, 0x80);\nreturn view.getInt8(0) === -0x80;"
            }, Subtest { name : "DataView (Uint8)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setUint8(0, 0x100);\nreturn view.getUint8(0) === 0;"
            }, Subtest { name : "DataView (Int16)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setInt16(0, 0x8000);\nreturn view.getInt16(0) === -0x8000;"
            }, Subtest { name : "DataView (Uint16)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setUint16(0, 0x10000);\nreturn view.getUint16(0) === 0;"
            }, Subtest { name : "DataView (Int32)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setInt32(0, 0x80000000);\nreturn view.getInt32(0) === -0x80000000;"
            }, Subtest { name : "DataView (Uint32)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setUint32(0, 0x100000000);\nreturn view.getUint32(0) === 0;"
            }, Subtest { name : "DataView (Float32)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setFloat32(0, 0.1);\nreturn view.getFloat32(0) === 0.10000000149011612;"
            }, Subtest { name : "DataView (Float64)", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setFloat64(0, 0.1);\nreturn view.getFloat64(0) === 0.1;"
            }, Subtest { name : "ArrayBuffer[Symbol.species]", exec :
            "return typeof ArrayBuffer[Symbol.species] === 'function';" }, Subtest { name
            : "constructors require new", exec :
            "var buffer = new ArrayBuffer(64);\nvar constructors = [\n  'ArrayBuffer',\n  'DataView',\n  'Int8Array',\n  'Uint8Array',\n  'Uint8ClampedArray',\n  'Int16Array',\n  'Uint16Array',\n  'Int32Array',\n  'Uint32Array',\n  'Float32Array',\n  'Float64Array'\n];\nreturn constructors.every(function (constructor) {\n  try {\n    if (constructor in global) {\n      global[constructor](constructor === \"ArrayBuffer\" ? 64 : buffer);\n    }\n    return false;\n  } catch(e) {\n    return true;\n  }\n});"
            }, Subtest { name : "constructors accept generic iterables", exec :
            "var constructors = [\n  'Int8Array',\n  'Uint8Array',\n  'Uint8ClampedArray',\n  'Int16Array',\n  'Uint16Array',\n  'Int32Array',\n  'Uint32Array',\n  'Float32Array',\n  'Float64Array'\n];\nfor(var i = 0; i < constructors.length; i++){\n  var arr = new global[constructors[i]](__createIterableObject([1, 2, 3]));\n  if(arr.length !== 3 || arr[0] !== 1 || arr[1] !== 2 || arr[2] !== 3)return false;\n}\nreturn true;"
            }, Subtest { name : "correct prototype chains", exec :
            "var constructors = [\n  'Int8Array',\n  'Uint8Array',\n  'Uint8ClampedArray',\n  'Int16Array',\n  'Uint16Array',\n  'Int32Array',\n  'Uint32Array',\n  'Float32Array',\n  'Float64Array'\n];\nvar constructor = Object.getPrototypeOf(Int8Array);\nvar prototype = Object.getPrototypeOf(Int8Array.prototype);\nif(constructor === Function.prototype || prototype === Object.prototype)return false;\nfor(var i = 0; i < constructors.length; i+=1) {\n  if (!(constructors[i] in global\n      && Object.getPrototypeOf(global[constructors[i]]) === constructor\n      && Object.getPrototypeOf(global[constructors[i]].prototype) === prototype\n      && Object.getOwnPropertyNames(global[constructors[i]].prototype).sort() + ''\n        === \"BYTES_PER_ELEMENT,constructor\")) {\n    return false;\n  }\n}\nreturn true;"
            }, Subtest { name : "%TypedArray%.from", exec :
            "return typeof Int8Array.from === \"function\" &&\ntypeof Uint8Array.from === \"function\" &&\ntypeof Uint8ClampedArray.from === \"function\" &&\ntypeof Int16Array.from === \"function\" &&\ntypeof Uint16Array.from === \"function\" &&\ntypeof Int32Array.from === \"function\" &&\ntypeof Uint32Array.from === \"function\" &&\ntypeof Float32Array.from === \"function\" &&\ntypeof Float64Array.from === \"function\";"
            }, Subtest { name : "%TypedArray%.of", exec :
            "return typeof Int8Array.of === \"function\" &&\ntypeof Uint8Array.of === \"function\" &&\ntypeof Uint8ClampedArray.of === \"function\" &&\ntypeof Int16Array.of === \"function\" &&\ntypeof Uint16Array.of === \"function\" &&\ntypeof Int32Array.of === \"function\" &&\ntypeof Uint32Array.of === \"function\" &&\ntypeof Float32Array.of === \"function\" &&\ntypeof Float64Array.of === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.subarray", exec :
            "return typeof Int8Array.prototype.subarray === \"function\" &&\ntypeof Uint8Array.prototype.subarray === \"function\" &&\ntypeof Uint8ClampedArray.prototype.subarray === \"function\" &&\ntypeof Int16Array.prototype.subarray === \"function\" &&\ntypeof Uint16Array.prototype.subarray === \"function\" &&\ntypeof Int32Array.prototype.subarray === \"function\" &&\ntypeof Uint32Array.prototype.subarray === \"function\" &&\ntypeof Float32Array.prototype.subarray === \"function\" &&\ntypeof Float64Array.prototype.subarray === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.join", exec :
            "return typeof Int8Array.prototype.join === \"function\" &&\ntypeof Uint8Array.prototype.join === \"function\" &&\ntypeof Uint8ClampedArray.prototype.join === \"function\" &&\ntypeof Int16Array.prototype.join === \"function\" &&\ntypeof Uint16Array.prototype.join === \"function\" &&\ntypeof Int32Array.prototype.join === \"function\" &&\ntypeof Uint32Array.prototype.join === \"function\" &&\ntypeof Float32Array.prototype.join === \"function\" &&\ntypeof Float64Array.prototype.join === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.indexOf", exec :
            "return typeof Int8Array.prototype.indexOf === \"function\" &&\ntypeof Uint8Array.prototype.indexOf === \"function\" &&\ntypeof Uint8ClampedArray.prototype.indexOf === \"function\" &&\ntypeof Int16Array.prototype.indexOf === \"function\" &&\ntypeof Uint16Array.prototype.indexOf === \"function\" &&\ntypeof Int32Array.prototype.indexOf === \"function\" &&\ntypeof Uint32Array.prototype.indexOf === \"function\" &&\ntypeof Float32Array.prototype.indexOf === \"function\" &&\ntypeof Float64Array.prototype.indexOf === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.lastIndexOf", exec :
            "return typeof Int8Array.prototype.lastIndexOf === \"function\" &&\ntypeof Uint8Array.prototype.lastIndexOf === \"function\" &&\ntypeof Uint8ClampedArray.prototype.lastIndexOf === \"function\" &&\ntypeof Int16Array.prototype.lastIndexOf === \"function\" &&\ntypeof Uint16Array.prototype.lastIndexOf === \"function\" &&\ntypeof Int32Array.prototype.lastIndexOf === \"function\" &&\ntypeof Uint32Array.prototype.lastIndexOf === \"function\" &&\ntypeof Float32Array.prototype.lastIndexOf === \"function\" &&\ntypeof Float64Array.prototype.lastIndexOf === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.slice", exec :
            "return typeof Int8Array.prototype.slice === \"function\" &&\ntypeof Uint8Array.prototype.slice === \"function\" &&\ntypeof Uint8ClampedArray.prototype.slice === \"function\" &&\ntypeof Int16Array.prototype.slice === \"function\" &&\ntypeof Uint16Array.prototype.slice === \"function\" &&\ntypeof Int32Array.prototype.slice === \"function\" &&\ntypeof Uint32Array.prototype.slice === \"function\" &&\ntypeof Float32Array.prototype.slice === \"function\" &&\ntypeof Float64Array.prototype.slice === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.every", exec :
            "return typeof Int8Array.prototype.every === \"function\" &&\ntypeof Uint8Array.prototype.every === \"function\" &&\ntypeof Uint8ClampedArray.prototype.every === \"function\" &&\ntypeof Int16Array.prototype.every === \"function\" &&\ntypeof Uint16Array.prototype.every === \"function\" &&\ntypeof Int32Array.prototype.every === \"function\" &&\ntypeof Uint32Array.prototype.every === \"function\" &&\ntypeof Float32Array.prototype.every === \"function\" &&\ntypeof Float64Array.prototype.every === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.filter", exec :
            "return typeof Int8Array.prototype.filter === \"function\" &&\ntypeof Uint8Array.prototype.filter === \"function\" &&\ntypeof Uint8ClampedArray.prototype.filter === \"function\" &&\ntypeof Int16Array.prototype.filter === \"function\" &&\ntypeof Uint16Array.prototype.filter === \"function\" &&\ntypeof Int32Array.prototype.filter === \"function\" &&\ntypeof Uint32Array.prototype.filter === \"function\" &&\ntypeof Float32Array.prototype.filter === \"function\" &&\ntypeof Float64Array.prototype.filter === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.forEach", exec :
            "return typeof Int8Array.prototype.forEach === \"function\" &&\ntypeof Uint8Array.prototype.forEach === \"function\" &&\ntypeof Uint8ClampedArray.prototype.forEach === \"function\" &&\ntypeof Int16Array.prototype.forEach === \"function\" &&\ntypeof Uint16Array.prototype.forEach === \"function\" &&\ntypeof Int32Array.prototype.forEach === \"function\" &&\ntypeof Uint32Array.prototype.forEach === \"function\" &&\ntypeof Float32Array.prototype.forEach === \"function\" &&\ntypeof Float64Array.prototype.forEach === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.map", exec :
            "return typeof Int8Array.prototype.map === \"function\" &&\ntypeof Uint8Array.prototype.map === \"function\" &&\ntypeof Uint8ClampedArray.prototype.map === \"function\" &&\ntypeof Int16Array.prototype.map === \"function\" &&\ntypeof Uint16Array.prototype.map === \"function\" &&\ntypeof Int32Array.prototype.map === \"function\" &&\ntypeof Uint32Array.prototype.map === \"function\" &&\ntypeof Float32Array.prototype.map === \"function\" &&\ntypeof Float64Array.prototype.map === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.reduce", exec :
            "return typeof Int8Array.prototype.reduce === \"function\" &&\ntypeof Uint8Array.prototype.reduce === \"function\" &&\ntypeof Uint8ClampedArray.prototype.reduce === \"function\" &&\ntypeof Int16Array.prototype.reduce === \"function\" &&\ntypeof Uint16Array.prototype.reduce === \"function\" &&\ntypeof Int32Array.prototype.reduce === \"function\" &&\ntypeof Uint32Array.prototype.reduce === \"function\" &&\ntypeof Float32Array.prototype.reduce === \"function\" &&\ntypeof Float64Array.prototype.reduce === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.reduceRight", exec :
            "return typeof Int8Array.prototype.reduceRight === \"function\" &&\ntypeof Uint8Array.prototype.reduceRight === \"function\" &&\ntypeof Uint8ClampedArray.prototype.reduceRight === \"function\" &&\ntypeof Int16Array.prototype.reduceRight === \"function\" &&\ntypeof Uint16Array.prototype.reduceRight === \"function\" &&\ntypeof Int32Array.prototype.reduceRight === \"function\" &&\ntypeof Uint32Array.prototype.reduceRight === \"function\" &&\ntypeof Float32Array.prototype.reduceRight === \"function\" &&\ntypeof Float64Array.prototype.reduceRight === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.reverse", exec :
            "return typeof Int8Array.prototype.reverse === \"function\" &&\ntypeof Uint8Array.prototype.reverse === \"function\" &&\ntypeof Uint8ClampedArray.prototype.reverse === \"function\" &&\ntypeof Int16Array.prototype.reverse === \"function\" &&\ntypeof Uint16Array.prototype.reverse === \"function\" &&\ntypeof Int32Array.prototype.reverse === \"function\" &&\ntypeof Uint32Array.prototype.reverse === \"function\" &&\ntypeof Float32Array.prototype.reverse === \"function\" &&\ntypeof Float64Array.prototype.reverse === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.some", exec :
            "return typeof Int8Array.prototype.some === \"function\" &&\ntypeof Uint8Array.prototype.some === \"function\" &&\ntypeof Uint8ClampedArray.prototype.some === \"function\" &&\ntypeof Int16Array.prototype.some === \"function\" &&\ntypeof Uint16Array.prototype.some === \"function\" &&\ntypeof Int32Array.prototype.some === \"function\" &&\ntypeof Uint32Array.prototype.some === \"function\" &&\ntypeof Float32Array.prototype.some === \"function\" &&\ntypeof Float64Array.prototype.some === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.sort", exec :
            "return typeof Int8Array.prototype.sort === \"function\" &&\ntypeof Uint8Array.prototype.sort === \"function\" &&\ntypeof Uint8ClampedArray.prototype.sort === \"function\" &&\ntypeof Int16Array.prototype.sort === \"function\" &&\ntypeof Uint16Array.prototype.sort === \"function\" &&\ntypeof Int32Array.prototype.sort === \"function\" &&\ntypeof Uint32Array.prototype.sort === \"function\" &&\ntypeof Float32Array.prototype.sort === \"function\" &&\ntypeof Float64Array.prototype.sort === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.copyWithin", exec :
            "return typeof Int8Array.prototype.copyWithin === \"function\" &&\ntypeof Uint8Array.prototype.copyWithin === \"function\" &&\ntypeof Uint8ClampedArray.prototype.copyWithin === \"function\" &&\ntypeof Int16Array.prototype.copyWithin === \"function\" &&\ntypeof Uint16Array.prototype.copyWithin === \"function\" &&\ntypeof Int32Array.prototype.copyWithin === \"function\" &&\ntypeof Uint32Array.prototype.copyWithin === \"function\" &&\ntypeof Float32Array.prototype.copyWithin === \"function\" &&\ntypeof Float64Array.prototype.copyWithin === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.find", exec :
            "return typeof Int8Array.prototype.find === \"function\" &&\ntypeof Uint8Array.prototype.find === \"function\" &&\ntypeof Uint8ClampedArray.prototype.find === \"function\" &&\ntypeof Int16Array.prototype.find === \"function\" &&\ntypeof Uint16Array.prototype.find === \"function\" &&\ntypeof Int32Array.prototype.find === \"function\" &&\ntypeof Uint32Array.prototype.find === \"function\" &&\ntypeof Float32Array.prototype.find === \"function\" &&\ntypeof Float64Array.prototype.find === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.findIndex", exec :
            "return typeof Int8Array.prototype.findIndex === \"function\" &&\ntypeof Uint8Array.prototype.findIndex === \"function\" &&\ntypeof Uint8ClampedArray.prototype.findIndex === \"function\" &&\ntypeof Int16Array.prototype.findIndex === \"function\" &&\ntypeof Uint16Array.prototype.findIndex === \"function\" &&\ntypeof Int32Array.prototype.findIndex === \"function\" &&\ntypeof Uint32Array.prototype.findIndex === \"function\" &&\ntypeof Float32Array.prototype.findIndex === \"function\" &&\ntypeof Float64Array.prototype.findIndex === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.fill", exec :
            "return typeof Int8Array.prototype.fill === \"function\" &&\ntypeof Uint8Array.prototype.fill === \"function\" &&\ntypeof Uint8ClampedArray.prototype.fill === \"function\" &&\ntypeof Int16Array.prototype.fill === \"function\" &&\ntypeof Uint16Array.prototype.fill === \"function\" &&\ntypeof Int32Array.prototype.fill === \"function\" &&\ntypeof Uint32Array.prototype.fill === \"function\" &&\ntypeof Float32Array.prototype.fill === \"function\" &&\ntypeof Float64Array.prototype.fill === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.keys", exec :
            "return typeof Int8Array.prototype.keys === \"function\" &&\ntypeof Uint8Array.prototype.keys === \"function\" &&\ntypeof Uint8ClampedArray.prototype.keys === \"function\" &&\ntypeof Int16Array.prototype.keys === \"function\" &&\ntypeof Uint16Array.prototype.keys === \"function\" &&\ntypeof Int32Array.prototype.keys === \"function\" &&\ntypeof Uint32Array.prototype.keys === \"function\" &&\ntypeof Float32Array.prototype.keys === \"function\" &&\ntypeof Float64Array.prototype.keys === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.values", exec :
            "return typeof Int8Array.prototype.values === \"function\" &&\ntypeof Uint8Array.prototype.values === \"function\" &&\ntypeof Uint8ClampedArray.prototype.values === \"function\" &&\ntypeof Int16Array.prototype.values === \"function\" &&\ntypeof Uint16Array.prototype.values === \"function\" &&\ntypeof Int32Array.prototype.values === \"function\" &&\ntypeof Uint32Array.prototype.values === \"function\" &&\ntypeof Float32Array.prototype.values === \"function\" &&\ntypeof Float64Array.prototype.values === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype.entries", exec :
            "return typeof Int8Array.prototype.entries === \"function\" &&\ntypeof Uint8Array.prototype.entries === \"function\" &&\ntypeof Uint8ClampedArray.prototype.entries === \"function\" &&\ntypeof Int16Array.prototype.entries === \"function\" &&\ntypeof Uint16Array.prototype.entries === \"function\" &&\ntypeof Int32Array.prototype.entries === \"function\" &&\ntypeof Uint32Array.prototype.entries === \"function\" &&\ntypeof Float32Array.prototype.entries === \"function\" &&\ntypeof Float64Array.prototype.entries === \"function\";"
            }, Subtest { name : "%TypedArray%.prototype[Symbol.iterator]", exec :
            "return typeof Int8Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Uint8Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Uint8ClampedArray.prototype[Symbol.iterator] === \"function\" &&\ntypeof Int16Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Uint16Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Int32Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Uint32Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Float32Array.prototype[Symbol.iterator] === \"function\" &&\ntypeof Float64Array.prototype[Symbol.iterator] === \"function\";"
            }, Subtest { name : "%TypedArray%[Symbol.species]", exec :
            "return typeof Int8Array[Symbol.species] === \"function\" &&\ntypeof Uint8Array[Symbol.species] === \"function\" &&\ntypeof Uint8ClampedArray[Symbol.species] === \"function\" &&\ntypeof Int16Array[Symbol.species] === \"function\" &&\ntypeof Uint16Array[Symbol.species] === \"function\" &&\ntypeof Int32Array[Symbol.species] === \"function\" &&\ntypeof Uint32Array[Symbol.species] === \"function\" &&\ntypeof Float32Array[Symbol.species] === \"function\" &&\ntypeof Float64Array[Symbol.species] === \"function\";"
            },
        ]
    }
}
