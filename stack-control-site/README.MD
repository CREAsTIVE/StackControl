Stack control site
---

### Building:
1. Generate documentation:
    ```
    cargo run --manifest-path ../stack-control-doc/Cargo.toml -- ./resources/documentation.json
    ```
2. Build wasm:
    ```
    wasm-pack build ../stack-control-wasm --out-dir ../stack-control-site/resources/stack-control-wasm-module
    ```
3. Install all modules:
    ```
    npm install
    ```
4. Build:
    ```
    npm run build
    ```