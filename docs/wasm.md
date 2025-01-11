# install the toolchain for wasm32

```sh
rustup target install wasm32-unknown-unknown
```

- build the wasm file

```sh
cargo build --release --target wasm32-unknown-unknown
```

- test locally using a runner

```sh
cargo install wasm-server-runner
```

```sh
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner
```

```sh
export WASM_SERVER_RUNNER_CUSTOM_INDEX_HTML=index.html
```
