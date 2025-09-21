use crate::driver::{run, run_file, run_prompt, run_interactive};

#[cfg(test)]

#[test]
fn test_run_valid_json() {
    let json_input = r#"{ "name": "test", "value": 42 }"#;
    // Ensure function runs
    run(json_input);
}

#[test]
fn test_run_invalid_json() {
    let json_input = r#"{"name": "test", "value": }"#; // invalid
    // Ensure function runs without panic even with invalid JSON
    run(json_input);
}

#[test]
fn test_run_file_creates_output() {
    use std::fs::File;
    use std::io::Write;
    use std::fs;

    // Write a temporary input file
    let mut file = File::create("input.json").unwrap();
    writeln!(file, r#"{{ "a": 1 }}"#).unwrap();
    // Call run_file
    run_file("input.json").unwrap();

    // Check that output.json exists and contains expected content
    let output = fs::read_to_string("output.json").unwrap();
    assert!(output.contains("1"));

    // Clean up files
    fs::remove_file("input.json").unwrap();
    fs::remove_file("output.json").unwrap();
}

