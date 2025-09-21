use std::io::{self, Write, BufRead};
use std::fs::File;


pub fn read_from_stdin() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut line = String::new();

    loop {
        line.clear();
        let n = stdin.lock().read_line(&mut line)?;
        if n == 0 || line.trim().is_empty() {
            break;
        }
        buffer.push_str(&line);
    }

    Ok(buffer.trim().to_string())
}



pub fn read_from_file(path: &str) -> Result<String, io::Error> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_stdout(s: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", s)
}

pub fn write_to_file(path: &str, s: &str) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

pub fn prompt_user(message: &str) -> Result<String,  io::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    read_from_stdin()
}
