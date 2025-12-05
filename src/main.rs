use std::io::{BufRead, BufReader, Write, Result};
use std::os::unix::net::UnixStream;
use std::env::var;
use clap::Parser;
pub mod args;
pub mod scratchpad_action;
pub mod state;
pub mod daemon;

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "daemon") {
        return daemon::run_daemon();
    }
    
    let args = args::Args::parse();
    
    let runtime_dir = var("XDG_RUNTIME_DIR")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "XDG_RUNTIME_DIR not set"))?;
    let socket_path = format!("{}/niri-scratchpad.sock", runtime_dir);
    
    let mut stream = UnixStream::connect(&socket_path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotConnected, "Daemon not running"))?;
    
    let request = serde_json::to_string(&args.action)?;
    writeln!(stream, "{}", request)?;
    
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    
    print!("{}", response.trim());
    
    Ok(())
}
