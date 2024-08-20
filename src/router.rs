use crate::network::Client;
use crate::state::STATE;
use crate::ui::global_ui;
use crate::ui::direct_ui;
use crate::ui::rating_ui;
use crate::ui::rooms_ui;
use crate::ui::room;
use crate::ui::landing;
use ratatui::prelude::*;


/// Displays the corresponding page in the UI using Ratatui depending on which tab is selected.
/// (Global Chat, Rooms, Direct Chat).
pub fn ui(
    frame: &mut Frame,
) {

    let state = STATE.lock().unwrap();
    let tab = state.tab;
    drop(state);
    
    match tab {
        0 => global_ui::render(frame),
        1 => rooms_ui::render(frame),
        2 => direct_ui::render(frame),
        3 => rating_ui::render(frame),
        4 => room::render(frame),  
        5 => landing::render(frame),
        _ => { log::info!("Goofed")}
    }
}


/// Handles Key Presses to modify the UI
/// Eg. Typing a message, Tab changes, Arrows to select various options
/// Takes various state as input (selected tabs, messages array, list states etc).
pub async fn handle_events(client: &mut Client) -> Result<bool, std::io::Error> {

    let state = STATE.lock().unwrap();
    let tab = state.tab;
    drop(state);

    match tab {
        0 => global_ui::handle_events(client).await,
        1 => rooms_ui::handle_events(),
        2 => direct_ui::handle_events(client).await,  
        3 => rating_ui::handle_events(client).await,  
        4 => room::handle_events(),
        5 => landing::handle_events().await,
        _ => {Ok(false)}
    }
}