use niri_ipc::{Request, Response, Action::{Spawn}};
use niri_ipc::socket::Socket;
use state::State;

use crate::state::AddResult;
pub mod state;
fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let state_file = State::new();
    let mut socket = Socket::connect()?;
    let Ok(Response::FocusedOutput(focused_output)) = socket.send(Request::FocusedOutput)? else {
        return Ok(());
    };
    let Ok(Response::FocusedWindow(focused_window)) = socket.send(Request::FocusedWindow)? else {
        return Ok(());
    };
    let Ok(Response::Windows(windows)) = socket.send(Request::Windows)? else {
        return Ok(());
    };

    if let Some(scratchpad_number) = args.next().and_then(|s| s.parse::<i32>().ok()) {
        match state_file {
            Ok(mut state) => {
                match focused_window {
                    Some(window) => {
                        match state.add_scratchpad(scratchpad_number, window.id, None) {
                            AddResult::Added => {
                                state.update();
                            },
                            AddResult::AlreadyExists => {
                                print!("This is when you'd summon an existing scratch");
                            },
                        }
                    },
                    None => eprintln!("No Focused window"),
                }
            },
            Err(err) => eprintln!("{}", err.to_string()),
        }
    } else {
        eprintln!("No Arg?");
    }

    for ws in windows {
        println!(
            "Workspace {} (id {}), focused: {}, id: {}",
            ws.title.unwrap_or_else(|| "<unnamed>".to_string()),
            ws.app_id.unwrap_or_else(|| "no id".to_string()),
            ws.is_focused,
            ws.id
        );
    }
    let command = Spawn { command: vec!["kitty".to_string()]};

    Ok(())
}
