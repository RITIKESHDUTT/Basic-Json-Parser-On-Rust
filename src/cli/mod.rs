use std::env;
use crate::driver::*;
use crate::io::*;

pub fn handle_cli() {
    let mut args = env::args().skip(1); // skip executable name

    match args.next().as_deref() {
        Some("run") => {
            match args.next().as_deref() {
                Some("-serialize") => {
                    if let Some(json_input) = args.next() {
                        match deserialize(&json_input) {
                            Ok(value) => println!("{}", serialize_pretty(&value)),
                            Err(e) => eprintln!("Error parsing JSON: {}", e),
                        }
                    } else{
                        eprintln!("Usage: basic_json_parser run -serialize '<json_string>'");
                    }
                }
                Some("-deserialize") => {
                    if let Some(json_input) = args.next() {
                        run(&json_input);
                    } else {
                        eprintln!("Usage: basic_json_parser run -deserialize '<json_string>'");
                    }
                }
                Some(json_input) => {
                    run(json_input);
                }
                None => {
                    eprintln!("Usage: basic_json_parser run -serialize|-deserialize '<json_string>'");
                }
            }
        }
        Some("file") => {
            match args.next().as_deref() {
                Some("-serialize") => {
                    if let Some(path) = args.next() {
                        if let Err(e) = {
                            let input = read_from_file(&path).unwrap();
                            let value = deserialize(&input).unwrap();
                            write_to_file("output.json", &serialize_pretty(&value))
                        } {
                            eprintln!("Error: {}", e);
                        }
                    } else {
                        eprintln!("Usage: basic_json_parser file serialize <file_path>");
                    }
                }
                Some("-deserialize") => {
                    if let Some(path) = args.next() {
                        if let Err(e) = run_file(&path) {
                            eprintln!("Error: {}", e);
                        }
                    } else {
                        eprintln!("Usage: basic_json_parser file deserialize <file_path>");
                    }
                }
                Some(path) => {
                    // fallback to old file behavior
                    if let Err(e) = run_file(path) {
                        eprintln!("Error: {}", e);
                    }
                }
                None => {
                    eprintln!("Usage: basic_json_parser file serialize|deserialize <file_path>");
                }
            }
        } _ => {
            eprintln!("Unknown command or missing arguments");
            eprintln!("Usage:");
            eprintln!("  basic_json_parser run -serialize '<json_string>'");
            eprintln!("  basic_json_parser run -deserialize '<json_string>'");
            eprintln!("  basic_json_parser file serialize <file_path>");
            eprintln!("  basic_json_parser file deserialize <file_path>");
        }
    }
}


pub(crate) fn run(json_input: &str) {
    match deserialize(&json_input) {
        Ok(value) => {
            println!("{}", value.t0_json_string());
        }
        Err(e) => eprintln!("Error parsing JSON: {}", e),
    }
}

pub(crate) fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = read_from_file(path)?;
    let value = deserialize(&input)?;
    write_to_file("output.json", &serialize(&value))?;
    Ok(())
}
