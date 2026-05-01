use std::io::{self, IsTerminal, Write};

pub fn print_json(value: &impl serde::Serialize) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    if stdout.is_terminal() {
        let v = serde_json::to_value(value)?;
        let colored = colored_json::to_colored_json_auto(&v)?;
        println!("{colored}");
    } else {
        serde_json::to_writer_pretty(stdout.lock(), value)?;
        stdout.lock().write_all(b"\n")?;
    }
    Ok(())
}

/// Print a slice of items as newline-delimited JSON (one JSON object per line).
/// TTY gets colored output, pipes get plain.
pub fn print_ndjson(items: &[impl serde::Serialize]) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let is_tty = stdout.is_terminal();
    let mut out = stdout.lock();
    for item in items {
        if is_tty {
            let v = serde_json::to_value(item)?;
            let colored = colored_json::to_colored_json_auto(&v)?;
            writeln!(out, "{colored}")?;
        } else {
            serde_json::to_writer(&mut out, item)?;
            out.write_all(b"\n")?;
        }
    }
    Ok(())
}
