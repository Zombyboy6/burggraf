pub mod constants;
pub mod ui;

use bevy::{
    asset::{AssetMetaCheck, embedded_asset},
    feathers::FeathersPlugin,
    prelude::*,
};

use crate::ui::UiPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Disabled because of wasm
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }))
    .init_state::<GameState>()
    .insert_resource(UiScale(10.))
    .add_plugins(UiPlugin)
    .add_plugins(FeathersPlugin)
    .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}
