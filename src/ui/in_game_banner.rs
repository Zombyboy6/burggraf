use bevy::{
    feathers::{font_styles::InheritableFont, handle_or_path::HandleOrPath, theme::ThemedText},
    prelude::*,
};

use crate::{
    GameState,
    constants::fonts,
    game_resources::GameResources,
    ui::{PAPER_SLICER, SCROLL_SLICER, widgets::SlicedImage},
};

#[derive(Component)]
pub struct UpdateResource;

pub(crate) fn spawn_banner(mut commands: Commands, game_resources: Res<GameResources>) {
    let mut root = commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            //justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        DespawnOnExit(GameState::InGame),
    ));

    root.with_child((
        Node {
            min_height: px(24),
            max_width: percent(100),
            //justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(px(10.0)),
            margin: UiRect::bottom(px(-5)),
            ..Default::default()
        },
        SlicedImage {
            image: HandleOrPath::Path("textures/ui/paper.png".to_string()),
            slicer: PAPER_SLICER,
        },
        InheritableFont {
            font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
            font_size: 14.0,
        },
        ZIndex(100),
        children![(
            Text::new(format!("{}", *game_resources)),
            TextLayout::new_with_linebreak(LineBreak::NoWrap),
            ThemedText,
            TextColor(Color::srgb_u8(130, 85, 45)),
            UpdateResource
        ),],
    ));
    root.with_child((
        Node {
            min_height: px(24),
            max_width: percent(100),
            //justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(px(10.0)).with_bottom(px(3.0)),
            ..Default::default()
        },
        SlicedImage {
            image: HandleOrPath::Path("textures/ui/scroll.png".to_string()),
            slicer: SCROLL_SLICER,
        },
        InheritableFont {
            font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
            font_size: 14.0,
        },
        ZIndex(100),
        children![(
            Text::new("Day 1"),
            TextLayout::new_with_linebreak(LineBreak::NoWrap),
            ThemedText,
            TextColor(Color::srgb_u8(130, 85, 45))
        ),],
    ));
}

pub fn update_resources(
    mut query: Query<&mut Text, With<UpdateResource>>,
    resources: Res<GameResources>,
) {
    if !resources.is_changed() {
        return;
    }
    for mut text in query.iter_mut() {
        text.0 = format!("{}", *resources);
    }
}
