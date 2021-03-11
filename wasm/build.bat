cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/release/wasm_hack.wasm
wasm-gc www/wasm_hack_bg.wasm
cd www
xcopy ..\..\rom .\rom /Y
python -m http.server
cd ..