use std::env;

use raftik_core::ast::Module;
use raftik_core::validation::validate_module;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <wasm_file>", args[0]);
        std::process::exit(1);
    }

    let wasm_file = &args[1];
    let data = std::fs::read(wasm_file)?;
    let module = Module::from_slice(&data)?;
    println!("{:#?}", module);
    validate_module(&module)?;
    Ok(())
}
