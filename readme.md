# Basic Json Parser On rust

A lightweight JSON parser and serializer implemented in **Rust**, following Rust's strict rules of ownership, borrowing, and error handling.

---

## Overview

This project provides:

- A **custom JSON parser** capable of parsing objects, arrays, strings, numbers, booleans, and null values.
- **Serialization back to JSON string** from parsed structures.
- Simple **I/O helpers** for reading from standard input, reading/writing files, and prompting the user.
- Support for **nested JSON** and **long, multi-level JSON structures**.

The parser demonstrates key Rust principles, including:

- **Memory safety without garbage collection**.
- **Ownership and borrowing** for string and vector handling.
- **Explicit error handling** using `Result<T, E>` and custom `JsonError` enum.
- **Trait implementation** for `Display` and `Error` to integrate with Rust's error ecosystem.

---

## Features

- Parse and serialize JSON values:
    - `Null`, `Bool`, `Number`, `String`
    - `Array` of JSON values
    - `Object` with string keys
- CLI support:
    - `run` — parse hardcoded JSON.
    - `run_interactive` — read JSON from stdin, parse and output serialized JSON.
    - `run_file` — read JSON from a file and write the result to an output file.
    - `run_prompt` — prompt the user for JSON input.
- Error reporting with line and column information using `JsonError`.
- Follows Rust best practices: ownership, borrowing, and error handling without panics.

---

## Installation

1. Clone the repository:

```bash
git clone git@github.com:yourusername/rust-json-parser.git
cd rust-json-parser
