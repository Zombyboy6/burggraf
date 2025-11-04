use bevy::{
    app::{App, Plugin, Update},
    sprite::{BorderRect, SliceScaleMode, TextureSlicer},
    state::state::OnEnter,
};

use crate::{
    GameState,
    ui::{main_menu::spawn_main_menu, widgets::button::button_hover},
};

mod main_menu;
pub mod widgets;

pub const PAPER_SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect {
        left: 5.0,
        right: 5.0,
        top: 5.0,
        bottom: 5.0,
    },
    center_scale_mode: SliceScaleMode::Stretch,
    sides_scale_mode: SliceScaleMode::Stretch,
    max_corner_scale: 1.0,
};

pub const PAPER_THICK_SLICER: TextureSlicer = TextureSlicer {
    border: BorderRect {
        left: 5.0,
        right: 5.0,
        top: 5.0,
        bottom: 7.0,
    },
    center_scale_mode: SliceScaleMode::Stretch,
    sides_scale_mode: SliceScaleMode::Stretch,
    max_corner_scale: 1.0,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(Update, button_hover);
    }
}
