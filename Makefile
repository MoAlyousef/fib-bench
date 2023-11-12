.PHONY: wasm server run

wasm:
	cd wasmfib; \
	cargo build --release --target wasm32-unknown-unknown; \
	wasm-bindgen ./target/wasm32-unknown-unknown/release/wasmfib.wasm --target web --no-typescript --out-dir ../www/; \
	cd ..;

server:
	cd server; \
	cargo build --release; \
	cd ..;

run:
	./server/target/release/server www
