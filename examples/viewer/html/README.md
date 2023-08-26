```
cargo build --target wasm32-unknown-unknown --example viewer --release
../target/wasm32-unknown-unknown/release/examples/viewer.wasm html/
cd html
basic-http-server .
```
