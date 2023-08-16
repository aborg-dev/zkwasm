all: build-wasm validate generate

build-wasm:
	echo "Converting WAT to WASM"
	for f in data/*.wat; do wat2wasm $f -o data/$(basename $f .wat).wasm; done

validate:
	echo "Validating generated WASM"
	cargo build --bin runner
	for f in data/*.wasm; do echo "Running $f" && target/debug/runner $f; done

generate:
	echo "Generating ZKASM from WASM"
	cargo build --bin zkwasm
	for f in data/*.wasm; do echo "Compiling $f" && target/debug/zkwasm $f; done
