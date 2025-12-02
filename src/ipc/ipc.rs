use std::io::Result;

use niri_ipc::{socket::Socket, Request, Response, Action::{MoveWindowToMonitor, MoveWindowToWorkspace}};
use crate::state::State;
// Ensures all scratchpads are stashed
pub fn stash(socket: &mut Socket, state: &State) -> Result<()> {
    let Ok(Response::Windows(windows)) = socket.send(Request::Windows)? else {
        return Ok(());
    };
    let Ok(Response::Workspaces(workspaces)) = socket.send(Request::Workspaces)? else {
        return Ok(());
    };
    let Some(stash_workspace) = workspaces.iter()
        .find(|workspace| workspace.name.as_deref() == Some("stash")) else {
            return Ok(())
        };
    for window in windows.iter().filter(|window| state.scratchpads.iter().any(|scratchpad| scratchpad.id == window.id)) {
        let move_action = MoveWindowToWorkspace { window_id: Some(window.id), reference: niri_ipc::WorkspaceReferenceArg::Id(stash_workspace.id), focus: false};
        let _ = socket.send(Request::Action(move_action));
    }
    Ok(())
}
