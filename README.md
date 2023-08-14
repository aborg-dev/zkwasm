# WASM -> ZK ASM compiler

## Dependencies

* https://github.com/WebAssembly/wabt is generally useful to manipulate WASM and WAT files
* https://github.com/0xPolygonHermez/zkevm-proverjs/ for testing resulting ZK ASM files

## Usage

### Preparing WASM from WAT

You can convert WAT to WASM with:
```sh
wat2wasm data/add.wat -o data/add.wasm
```

### Validating resulting WASM

You can check the execution (assertions) in the WASM file with:
```sh
cargo run --bin runner data/add.wasm
```
