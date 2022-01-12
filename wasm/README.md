# RustyHack on WASM backend
This is the web version of the RustyHack. You can check it [here](https://nouvadam.github.io/hack_cpu_emu/).
  
## How to build and run
Install necessary tools when needed.

1. Compile project to WASM.
 
`cargo build --target wasm32-unknown-unknown --release`

2. Generate bindings from compiled WASM to JavaScript.
 
`wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/release/wasm_hack.wasm`

3. Optional: Reduce size of generated WASM.
 
`wasm-gc www/wasm_hack_bg.wasm`

4. Run http server from www folder.

`cargo install simple-http-server`
`simple-http-server`

5 Enter localhost:8000/index.html in your web brower. 
