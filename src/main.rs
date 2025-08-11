use raftik::binary::raw_module::RawModule;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <wasm_file>", args[0]);
        std::process::exit(1);
    }

    let wasm_file = &args[1];
    match std::fs::read(wasm_file) {
        Ok(data) => match RawModule::try_from(data.as_slice()) {
            Ok(module) => println!("{:#?}", module),
            Err(e) => eprintln!("Error parsing module: {}", e),
        },
        Err(e) => eprintln!("Error reading file {}: {}", wasm_file, e),
    }
}
