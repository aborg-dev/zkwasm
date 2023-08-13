use std::{env, fs};

use anyhow::Result;

mod codegen;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: main WASM_FILEPATH");
    }
    let module = fs::read_to_string(&args[1]).expect("Failed to read file");
    let program = codegen::parse(module.as_bytes())?;
    fs::write("out.zkasm", program)?;
    Ok(())
}
