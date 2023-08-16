all: build-wasm validate compile

# Converts WAT to WASM.
build-wasm:
	for f in data/*.wat; do wat2wasm $f -o data/$(basename $f .wat).wasm; done

# Validates generated WASM.
validate:
	cargo build --bin runner
	for f in data/*.wasm; do echo "Validating $f" && target/debug/runner $f; done

# Generates ZKASM from WASM.
compile:
	cargo build --bin zkwasm
	for f in data/*.wasm; do echo "Compiling $f" && target/debug/zkwasm $f; done
