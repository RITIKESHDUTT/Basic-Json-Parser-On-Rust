# Basic Json Parser On rust

A lightweight JSON parser and serializer implemented in **Rust**..

---

## Overview

This project provides:

- A **custom JSON parser** capable of parsing objects, arrays, strings, numbers, booleans, and null values.
- **Serialization back to JSON string** from parsed structures.
- Simple **I/O helpers** for reading/writing files.
- Support for **nested JSON** and **long, multi-level JSON structures**.

The parser demonstrates key Rust principles, including:

- **Memory safety without garbage collection** — safe parsing and serialization with no unsafe code.
- **Ownership and Borrowing** — efficient handling of strings and vectors without unnecessary clones.
- **Iterators and Pattern Matching** — lexical analysis with `Peekable<Chars>` and recursive descent parsing.
- **Error Handling** — a custom `JsonError` enum with line/column reporting.
- **Traits and Integration** — implementations of `Display` and `Error` for idiomatic error usage.
- **Modular Design** — organized into `core`, `parser`, `serialization`, `engine`, `cli`, etc.

---

## Features

- Parse and serialize JSON values:
    - `Null`, `Bool`, `Number`, `String`
    - `Array` of JSON values
    - `Object` with string keys
- CLI support:
    - `run -serialize` —  serialize json in cli
    - `run -deserialize` — deserialize json in cli
    - `file -serialize` — read JSON from a file and write the result to an output file, currently only results in output.json.  Would need to make sure that it already doesn't exist.
    - `file -deserialize`- read JSON from a file and write the deserialized result to an output file
- Error reporting with line and column information using `JsonError`.
- Follows Rust best practices: ownership, borrowing, and error handling without panics.

---

## Installation

1. Clone the repository:

```bash
git@github.com:RITIKESHDUTT/Basic-Json-Parser-On-Rust.git

cd Basic-Json-Parser-On-Rust
```

