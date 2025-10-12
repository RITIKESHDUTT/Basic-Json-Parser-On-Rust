use std::{env};
use crate::driver::*;
use crate::io::*;

pub fn handle_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1); // skip executable name

    match args.next().as_deref() {
        Some("run") => {
            match args.next().as_deref() {
                Some("-serialize") => {
                    if let Some(json_input) = args.next() {
                        let value = deserialize(&json_input)?;
                        println!("{}", serialize_pretty(&value));
                    } else {
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
                Some(json_input) => run(json_input),
                None => eprintln!("Usage: basic_json_parser run -serialize|-deserialize '<json_string>'"),
            }
        }
        Some("file") => {
            match args.next().as_deref() {
                Some("-serialize") => {
                    if let (Some(input_path), Some(output_path)) = (args.next(), args.next()) {
                        let input = read_from_file(&input_path)?;
                        let value = deserialize(&input)?;
                        write_to_file(&output_path, &serialize_pretty(&value))?;
                    } else {
                        eprintln!("Usage: basic_json_parser file serialize <input_file> <output_file>");
                    }
                }
                Some("-deserialize") => {
                    if let (Some(input_path), Some(output_path)) = (args.next(), args.next()) {
                        run_file(&input_path, &output_path)?;
                    } else {
                        eprintln!("Usage: basic_json_parser file deserialize <input_file> <output_file>");
                    }
                }
                Some(path) => {
                    eprintln!("Unknown argument `{}` or missing output file.", path);
                    eprintln!("Usage: basic_json_parser file serialize|deserialize <input_file> <output_file>");
                }
                None => eprintln!("Usage: basic_json_parser file serialize|deserialize <input_file> <output_file>"),
            }
        }
        _ => {
            eprintln!("Unknown command or missing arguments");
            eprintln!("Usage:");
            eprintln!("  basic_json_parser run -serialize '<json_string>'");
            eprintln!("  basic_json_parser run -deserialize '<json_string>'");
            eprintln!("  basic_json_parser file serialize <input_file> <output_file>");
            eprintln!("  basic_json_parser file deserialize <input_file> <output_file>");
        }
    }

    Ok(())
}


pub(crate) fn run(json_input: &str) {
    match deserialize(&json_input) {
        Ok(value) => {
            println!("{}", value.to_json_string());
        }
        Err(e) => eprintln!("Error parsing JSON: {}", e),
    }
}

pub(crate) fn run_file(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = read_from_file(input_path)?;
    let value = deserialize(&input)?;
    write_to_file(output_path, &serialize(&value))?;
    Ok(())
}
