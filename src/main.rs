use std::env;

use raftik::ast::Module;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <wasm_file>", args[0]);
        std::process::exit(1);
    }

    let wasm_file = &args[1];
    match std::fs::read(wasm_file) {
        Ok(data) => match Module::from_slice(&data) {
            Ok(module) => println!("{:#?}", module),
            Err(e) => eprintln!("Error parsing module: {}", e),
        },
        Err(e) => eprintln!("Error reading file {}: {}", wasm_file, e),
    }
}
