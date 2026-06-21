use std::env;

use raftik_core::{Module, Store};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <wasm_file>", args[0]);
        std::process::exit(1);
    }

    let wasm_file = &args[1];
    let data = std::fs::read(wasm_file)?;
    let mut store = Store::default();
    let module = Module::from_slice(&data, &mut store)?;
    println!("{:#?}", module);
    Ok(())
}
