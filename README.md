# Gui_NewsApi_Rust
Install all dependencies with :
```shell
    cargo check or cargo update
```
Test web Assembly : 
1/install wasm target :
```shell
rustup target add wasm32-unknown-unknown
```
2/test with:
```shell
cargo build --target wasm32-unknown-unknown
```

Run app with libraries not in wasm target :
cargo build --lib -p headlines --target wasm32-unknown-unknown
Mode Release : 
cargo build --release -p headlines --target wasm32-unknown-unknown
