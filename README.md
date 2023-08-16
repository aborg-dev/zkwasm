# WASM -> ZK ASM compiler

## Dependencies

* https://github.com/0xPolygonHermez/zkevm-proverjs/ for testing resulting ZK ASM files

## Usage

### Generating ZK ASM

You can generate `.zkasm` file from WASM file with:
```sh
cargo run data/add.wasm
```

The result will be stored in `data/add.zkasm`.

Alternatively, you can add a new test WAT file into `data/file.wat` and declare it in `tests/integration_test.rs`.

Then, running

```sh
UPDATE_EXPECT=1 cargo test
```

will generate a new associated ZKASM file in `data/file.zkasm`.
