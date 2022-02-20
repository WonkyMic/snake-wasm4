.PHONY : build watch
build :
	cargo build --release
watch : 
	w4 watch target/wasm32-unknown-unknown/release/cart.wasm