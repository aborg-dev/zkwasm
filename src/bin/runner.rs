use std::{env, fs};

use anyhow::Result;
use wasmi::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: runner WASM_FILEPATH");
    }
    let wasm_filepath = &args[1];
    let wat = fs::File::open(wasm_filepath).expect("Failed to read file");
    let engine = Engine::default();
    let module = Module::new(&engine, wat)?;

    type HostState = ();
    let mut store = Store::new(&engine, ());
    let host_assert = Func::wrap(
        &mut store,
        |_caller: Caller<'_, HostState>, lhs: i32, rhs: i32| {
            assert_eq!(lhs, rhs);
        },
    );

    let mut linker = <Linker<HostState>>::new(&engine);
    linker.define("env", "assert_eq", host_assert)?;
    let _ = linker.instantiate(&mut store, &module)?.start(&mut store)?;

    Ok(())
}
