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
    let Ok(Response::FocusedWindow(Some(focused_window))) = socket.send(Request::FocusedWindow)?
    else {
        return Ok(());
    };
    let Ok(Response::Windows(windows)) = socket.send(Request::Windows)? else {
        return Ok(());
    };

    if let Some(scratchpad_number) = args.next().and_then(|s| s.parse::<i32>().ok()) {
        match state_file {
            Ok(mut state) => match state.add_scratchpad(scratchpad_number, focused_window.id, None) {
                Added => {
                    state.update()?;
                }
                AlreadyExists(scratchpad) => {
                    //ipc::stash(&mut socket, &state)?;
                    ipc::summon(&mut socket, &scratchpad)?;
                }
            },
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("No Arg?");
    }

    Ok(())
}
