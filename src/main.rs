use std::{env, fs};

use anyhow::Result;

fn parse(module: &[u8]) -> Result<()> {
    let parser = wasmparser::Parser::new(0);
    
    for payload in parser.parse_all(module) {
        let payload = payload?;
        dbg!(payload);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: main WASM_FILEPATH");
    }
    let module = fs::read_to_string(&args[1]).expect("Failed to read file");
    parse(module.as_bytes())?;
    Ok(())
}
