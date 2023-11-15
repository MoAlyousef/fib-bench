# Fib Bench

Wasm vs server-side(rust) vs js(browser-side) fibonacci benchmark.

WebAssembly can't directly access the DOM, it has to call javascript to do so and is known to incur a cost when manipulating the DOM. How about raw computation, how does wasm compare to server-side computation or client-side javascript computation?

## Building

The wasm binary is already included, however, to rebuild it:
```bash
make wasm
```
This requires the wasm32-unknown-unkown target, in addition to wasm-bindgen-cli and wasm-opt, both can be installed via `cargo install`.

To build the server:
```bash
make server
```
This is a simple thread per request and blocking server based on tiny-http.

To run server from the root dir:
```bash
make run
```

Results and conclusions can be found here:

https://moalyousef.github.io/blog/WasmCompute.html
