use niri_ipc::socket::Socket;
use niri_ipc::{Action::Spawn, Request, Response};
use state::State;

use crate::state::AddResult::{Added, AlreadyExists};
pub mod ipc;
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
            Ok(mut state) => match focused_window {
                Some(window) => match state.add_scratchpad(scratchpad_number, window.id, None) {
                    Added => {
                        state.update()?;
                    }
                    AlreadyExists(scratchpad) => {
                        if window.id == scratchpad.id {
                            ipc::stash(&mut socket, &state)?;
                        } else {
                            ipc::summon(&mut socket, &scratchpad)?;
                        }
                    }
                },
                None => {
                    if let Some(scratchpad) = state
                        .scratchpads
                        .iter()
                        .find(|scratchpad| scratchpad.scratchpad_number == scratchpad_number)
                    {
                        ipc::summon(&mut socket, scratchpad)?;
                        return Ok(());
                    };
                }
            },
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("No Arg?");
    }

    Ok(())
}
