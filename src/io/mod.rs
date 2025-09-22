use std::io::{self, Write, Read};
use std::fs::File;

pub(crate)  fn read_from_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub(crate) fn write_to_file(path: &str, s: &str) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

