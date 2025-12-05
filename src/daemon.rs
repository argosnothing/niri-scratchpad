use std::{os::unix::net::UnixListener, io::{BufRead, BufReader, Write}};

pub fn run_daemon() {
    let socket_path = std::env::var("XDG_RUNTIME_DIR").unwrap() + "/niri-scratchpad.sock";
    let _ = std::fs::remove_file(&socket_path);
    let listener = UnixListener::bind(&socket_path).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        writeln!(stream, "{{\"status\":\"ok\"}}").unwrap();
    }

}
