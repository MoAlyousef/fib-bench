.PHONY: wasm server run

wasm:
	cd wasmfib; \
	cargo build --release --target wasm32-unknown-unknown; \
	wasm-bindgen ./target/wasm32-unknown-unknown/release/wasmfib.wasm --target web --no-typescript --out-dir ../www/; \
	cd ..; \
	wasm-opt www/wasmfib_bg.wasm -O3 -o www/wasmfib_bg.wasm;

server:
	cd server; \
	cargo build --release; \
	cd ..;

run:
	./server/target/release/server www
