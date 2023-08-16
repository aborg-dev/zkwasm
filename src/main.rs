use std::{env, fs};

use anyhow::Result;

mod codegen;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: main WASM_FILEPATH");
    }
    let wasm_filepath = &args[1];
    let module = fs::read(wasm_filepath).expect(&format!("Failed to read file {wasm_filepath}"));
    let program = codegen::parse(&module)?;
    let output_filepath = format!(
        "{}.zkasm",
        wasm_filepath
            .strip_suffix(".wasm")
            .expect("expected extension .wasm")
    );
    fs::write(output_filepath, program)?;
    Ok(())
}
