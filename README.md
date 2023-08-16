# WASM -> ZK ASM compiler

## Dependencies

* https://github.com/WebAssembly/wabt is generally useful to manipulate WASM and WAT files
* https://github.com/0xPolygonHermez/zkevm-proverjs/ for testing resulting ZK ASM files
* https://github.com/casey/just to simplify running common commands

## Usage

### Preparing WASM from WAT

You can convert WAT to WASM with:
```sh
wat2wasm data/add.wat -o data/add.wasm

# Or do this for all files with:
just build-wasm
```

### Validating resulting WASM

You can check the execution (assertions) in the WASM file with:
```sh
cargo run --bin runner data/add.wasm

# Or do this for all files with:
just validate
```

### Generating ZK ASM

You can generate `.zkasm` file from WASM file with:
```sh
cargo run --bin zkwasm data/add.wasm

# Or do this for all files with:
just generate
```

The result will be stored in `data/add.zkasm`.
