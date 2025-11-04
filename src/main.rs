pub mod constants;
pub mod ui;

use bevy::{
    asset::AssetMetaCheck,
    feathers::FeathersPlugin,
    image::ImageSamplerDescriptor,
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    ui_widgets::UiWidgetsPlugins,
};

use crate::ui::UiPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Disabled because of wasm
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            }),
    )
    .init_state::<GameState>()
    .insert_resource(UiScale(4.0))
    .add_plugins(UiPlugin)
    .add_plugins((
        FeathersPlugin,
        UiWidgetsPlugins,
        InputDispatchPlugin,
        TabNavigationPlugin,
    ))
    .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}
