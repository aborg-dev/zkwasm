use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: main WASM_FILEPATH");
        return;
    }
    let module = fs::read_to_string(&args[1]).expect("Failed to read file");
    let parser = wasmparser::Parser::new(0);
    
    for payload in parser.parse_all(module.as_bytes()) {
        dbg!(payload.expect("Why result?"));
    }
}
