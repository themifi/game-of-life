test:
	wasm-pack test --firefox --headless

build:
	wasm-pack build --target web --out-dir ./www/pkg
