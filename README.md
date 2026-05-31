# Deepseek's POW(Proof of Work) Challange solver
A reimplimentation of deepseek's web chat pow solver in rust and wasm to get near native speed in python using wasmtime.

# Building Yourself
The .wasm file is uploaded arleady in releases but if you wanna build it yourself then 
- Clone the repo
- Install rust
- Add wasm32-unknown-unknown target( Run ``` rustup target add wasm32-unknown-unknown```)

Then run following command to build the wasm file
```bash
cargo build --target wasm32-unknown-unknown --release
```

Wasm file can be found in ```target/wasm32-unknown-unknown/release/deepseek_pow_solver.wasm```
