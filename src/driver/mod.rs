use crate::engine::Parser;
use crate::io::*;


pub fn run(json_input: &str) {
    let mut parser = Parser::new(json_input);
    match parser.parse() {
        Ok(value) => {
            println!("Successfully parsed JSON:\n{:#?}", value);
            println!("\nSerialized back to JSON:\n{}", value.to_json_string());
        }
        Err(e) => eprintln!("Error parsing JSON: {}", e),
    }
}

pub fn run_interactive() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_from_stdin()?; // read JSON string from stdin
    let mut parser = Parser::new(&input);
    match parser.parse() {
        Ok(value) => {
            // Print success message
            write_to_stdout(&format!("Successfully parsed JSON:\n{:#?}", value))?;
            // Print serialized JSON
            write_to_stdout(&format!("\nSerialized back to JSON:\n{}", value.to_json_string()))?;
        }
        Err(e) => eprintln!("Error parsing JSON: {}", e),
    }
    Ok(())
}


pub fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input = read_from_file(path)?;
    let mut parser = Parser::new(&input);
    let value = parser.parse()?;
    write_to_file("output.json", &value.to_json_string())?;
    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let input = prompt_user("Enter JSON (finish with empty line):")?;
    let mut parser = Parser::new(&input);
    let value = parser.parse()?;
    println!("Parsed JSON:\n{:#?}", value);
    println!("\nSerialized back to JSON:\n{}", value.to_json_string());
    Ok(())
}
