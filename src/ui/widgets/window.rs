use bevy::{
    ecs::spawn::SpawnableList,
    feathers::{font_styles::InheritableFont, handle_or_path::HandleOrPath, theme::ThemedText},
    prelude::*,
};

use crate::{
    constants::fonts,
    ui::{PAPER_SLICER, SCROLL_SLICER, widgets::SlicedImage},
};

pub fn window<C: SpawnableList<ChildOf> + Send + Sync + 'static>(
    title: impl Into<String>,
    node: Node,
    children: C,
) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            (
                Node {
                    min_height: px(24),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(px(10.0)).with_bottom(px(3.0)),
                    margin: UiRect::bottom(px(-5)),
                    ..Default::default()
                },
                ZIndex(100),
                InheritableFont {
                    font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
                    font_size: 14.0,
                },
                SlicedImage {
                    image: HandleOrPath::Path("textures/ui/scroll.png".to_string()),
                    slicer: SCROLL_SLICER,
                },
                children![(
                    Text::new(title),
                    ThemedText,
                    TextColor(Color::srgb_u8(130, 85, 45))
                )]
            ),
            (
                Node {
                    padding: UiRect::all(px(4)).with_top(px(8)),
                    ..node
                },
                InheritableFont {
                    font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
                    font_size: 12.0,
                },
                SlicedImage {
                    image: HandleOrPath::Path("textures/ui/paper.png".to_string()),
                    slicer: PAPER_SLICER,
                },
                Children::spawn(children)
            )
        ],
    )
}
