use bevy::{
    app::{App, Plugin},
    state::state::OnEnter,
};

use crate::{GameState, ui::main_menu::spawn_main_menu};

mod main_menu;
pub mod widgets;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
    }
}
