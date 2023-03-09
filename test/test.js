const wasm = require('./hello/pkg');

const a = new Float32Array([1.0, 2.0, 3.0]);
const b = new Float32Array([4.0, 5.0, 6.0]);
const result = wasm.cosine_distance(a,b);
console.log(result);
