use basic_json_parser::driver::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1️⃣ Run with a hardcoded JSON string
    let json_input = r#"{ "name": "example", "value": 123 }"#;
    println!("--- Running run() with hardcoded JSON ---");
    run(json_input);

    // 2️⃣ Run interactive (reads from stdin)
    println!("--- Running run_interactive() ---");
    println!("Please enter JSON (Ctrl+D to end on Linux/macOS, Ctrl+Z on Windows):");
    run_interactive()?; // will read from stdin and print output

    // 3️⃣ Run file (reads from a file named input.json and writes output.json)
    println!("--- Running run_file() ---");
    // Make sure "input.json" exists with valid JSON
    let input_file_path = "input.json";
    match run_file(input_file_path) {
        Ok(_) => println!("Successfully parsed file and wrote to output.json"),
        Err(e) => eprintln!("Error parsing file: {}", e),
    }

    // 4️⃣ Run prompt (asks user for input)
    println!("--- Running run_prompt() ---");
    run_prompt()?; // prompts user and prints parsed JSON

    let json_input = long_nested_json();
    println!("JSON string length: {}", json_input.len());
    println!("JSON for stress test:\n{}", json_input);

    Ok(())
}

fn long_nested_json() -> String {
    // Using raw string literal r#"..."# for multi-line JSON
    let json_str = r#"
    {
        "name": "StressTest",
        "version": 1.0,
        "active": true,
        "features": ["parser", "serializer", "lexer", "io"],
        "metadata": {
            "author": "OpenAI",
            "license": "MIT",
            "contributors": [
                {"name": "Alice", "role": "developer"},
                {"name": "Bob", "role": "tester"},
                {"name": "Carol", "role": "maintainer"}
            ]
        },
        "nested_levels": {
            "level1": {
                "level2": {
                    "level3": {
                        "level4": {
                            "numbers": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                            "booleans": [true, false, true, false],
                            "strings": ["a", "b", "c", "d"],
                            "nulls": [null, null, null]
                        }
                    }
                }
            }
        },
        "array_of_objects": [
            {"id": 1, "value": 100},
            {"id": 2, "value": 200},
            {"id": 3, "value": 300}
        ]
    }
    "#;

    // Optionally, remove leading/trailing whitespace and return
    json_str.trim().to_string()
}
