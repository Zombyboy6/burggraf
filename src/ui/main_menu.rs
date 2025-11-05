use bevy::{
    feathers::theme::ThemedText,
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui::InteractionDisabled,
    ui_widgets::{Activate, observe},
};

use crate::ui::widgets::{button, window};

pub(crate) fn spawn_main_menu(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut root = commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        TabGroup::default(),
    ));

    root.with_child(window(
        "Main Menu",
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        (
            Spawn((
                Node {
                    min_width: px(80),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(px(4)),
                    ..default()
                },
                children![button(
                    (),
                    Spawn((
                        Text::new("Start Game"),
                        ThemedText,
                        TextColor(Color::srgb_u8(130, 85, 45)),
                    ))
                )],
            )),
            Spawn((
                Node {
                    min_width: px(80),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(px(4)),
                    ..default()
                },
                children![button(
                    InteractionDisabled,
                    Spawn((
                        Text::new("Settings"),
                        ThemedText,
                        TextColor(Color::srgb_u8(130, 85, 45)),
                    ))
                )],
            )),
            Spawn((
                Node {
                    min_width: px(80),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::bottom(px(4)),
                    ..default()
                },
                children![button(
                    observe(
                        |_activate: On<Activate>, mut message: MessageWriter<AppExit>| {
                            message.write(AppExit::Success);
                        },
                    ),
                    Spawn((
                        Text::new("Quit Game"),
                        ThemedText,
                        TextColor(Color::srgb_u8(130, 85, 45)),
                    ))
                )],
            )),
        ),
    ));
}
