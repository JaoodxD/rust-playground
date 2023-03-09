const fs = require('fs');

async function start() {
  const wasmBuffer = fs.readFileSync('./hello/target/wasm32-unknown-unknown/release/hello.wasm');
  let { instance } = await WebAssembly.instantiate(wasmBuffer, { });
  const a = new Float64Array([1.0, 2.0, 3.0]);
  const b = new Float64Array([1.0, 2.0, 3.0]);
  console.log(a, b);
  const cosineDistance = instance.exports.cosine_distance;
  const distance = cosineDistance(a, b);
  console.log(distance);
}

start()

