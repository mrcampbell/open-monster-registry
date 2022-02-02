import * as wasm from "omr-engine";

// wasm.greet();

const res = wasm.execute("query { speciesByID(id: 1) { name id elements} }")
console.log(res)
