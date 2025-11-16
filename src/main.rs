pub mod constants;
pub mod leaf_material;
mod player;
pub mod ui;
mod world;

use avian3d::PhysicsPlugins;
use bevy::{
    asset::AssetMetaCheck,
    feathers::FeathersPlugin,
    image::ImageSamplerDescriptor,
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    pbr::{ExtendedMaterial, wireframe::WireframeConfig},
    prelude::*,
    ui_widgets::UiWidgetsPlugins,
};
use puppeteer::PuppeteerPlugin;

use crate::{
    leaf_material::LeafMaterialExtension, player::PlayerPlugin, ui::UiPlugin, world::WorldPlugin,
};

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
    // Third party plugins
    .add_plugins((PhysicsPlugins::default(), PuppeteerPlugin))
    // Game plugins
    .add_plugins((
        UiPlugin,
        PlayerPlugin,
        WorldPlugin,
        MaterialPlugin::<ExtendedMaterial<StandardMaterial, LeafMaterialExtension>>::default(),
    ))
    // Bevy plugins
    .add_plugins((
        //WireframePlugin::new(RenderDebugFlags::default()),
        FeathersPlugin,
        UiWidgetsPlugins,
        InputDispatchPlugin,
        TabNavigationPlugin,
    ));
    app.insert_resource(WireframeConfig {
        global: true,
        ..default()
    });
    app.run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}
