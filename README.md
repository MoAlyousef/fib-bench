# Fib Bench

Wasm vs server-side(rust) vs js(browser-side) fibonacci benchmark.

The wasm binary is already included, however, to rebuild it:
```bash
make wasm
```
This requires the wasm32-unknown-unkown target, in addition to wasm-bindgen-cli and wasm-opt, both can be installed via `cargo install`.

To build the server:
```bash
make server
```
This uses tiny-http, which is a low-level http server library. It's mutlithreaded and blocking.

To run server from the root dir:
```bash
make run
```

## Results
With an input value of 1:
- servertime: 6.831298828125 ms
- wasmtime: 0.008056640625 ms
- jstime: 0.004150390625 ms

With an input value of 45:
- servertime: 2983.470703125 ms
- wasmtime: 8184.0751953125 ms
- jstime: 15975.77490234375 ms

The results should appear in the browser's dev console. 

This was run on a windows machine running wsl2 x86_64 GNU/Linux.
Specs:
- Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz
- Speed: 3.40 GHz
- Cores: 4
- Logical processors: 8
- RAM: 16 GB
- HDD: ST1000LM035-1RK172

Rust version: 1.71 stable.
Google chrome: Version 119.0.6045.107 (Official Build) (64-bit)

## Conclusion
- wasm-opt -O3 didn't improve performance by much. It did reduce the generated wasm size by several hundred bytes. It's advantage might be apparent in larger wasm binaries perhaps. 
- Calling server-side computations requires a network call and marshalling data to and from the server, which incurs an unnecessary cost when the computation is trivial. In such cases javascript and wasm offer close enough computation cost.
- For intensive computations, the server cost can be considered negligible since native computation remains faster than wasm. Even then, client-side javascript is only twice as slow as wasm!
