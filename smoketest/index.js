import * as wasm from "omr-engine";

// wasm.greet();

const res = wasm.execute("query { favoriteEpisode }")
console.log(res)
