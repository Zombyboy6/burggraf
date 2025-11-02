use bevy::{feathers::theme::ThemedText, prelude::*};

use crate::ui::widgets::button::button;

pub(crate) fn spawn_main_menu(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(150.0),
                    ..default()
                })
                .with_child(button((), Spawn((Text::new("Start Game"), ThemedText))));
        });
}
