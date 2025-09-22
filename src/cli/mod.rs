use std::env;
use crate::driver::*;

pub fn handle_cli() {
    let mut args = env::args().skip(1); // skip executable name

    match args.next().as_deref() {
        Some("run") => {
            if let Some(json_input) = args.next() {
                run(&json_input);
            } else {
                eprintln!("Usage: jsonparser run '<json_string>'");
            }
        }
        Some("interactive") => {
            if let Err(e) = run_interactive() {
                eprintln!("Error: {}", e);
            }
        }
        Some("file") => {
            if let Some(path) = args.next() {
                if let Err(e) = run_file(&path) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: jsonparser file <file_path>");
            }
        }
        _ => {
            eprintln!("Unknown command or missing arguments");
            eprintln!("Usage:");
            eprintln!("  jsonparser run '<json_string>'");
            eprintln!("  jsonparser interactive");
            eprintln!("  jsonparser file <file_path>");
        }
    }
}
