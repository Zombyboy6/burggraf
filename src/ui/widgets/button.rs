use bevy::{
    ecs::{lifecycle::HookContext, spawn::SpawnableList, world::DeferredWorld},
    feathers::{font_styles::InheritableFont, handle_or_path::HandleOrPath},
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
};

use crate::constants::fonts;

pub fn button<C: SpawnableList<ChildOf> + Send + Sync + 'static, B: Bundle>(
    overrides: B,
    children: C,
) -> impl Bundle {
    (
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(8.0), Val::Px(0.)),
            flex_grow: 1.0,
            ..Default::default()
        },
        Button,
        Hovered::default(),
        //EntityCursor::System(bevy_window::SystemCursorIcon::Pointer),
        TabIndex(0),
        SlicedImage {
            image: HandleOrPath::Path("textures/ui/paper_thick.png".to_string()),
            slicer: TextureSlicer {
                border: BorderRect {
                    left: 20.0,
                    right: 20.0,
                    top: 20.0,
                    bottom: 36.0,
                },
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            },
        },
        InheritableFont {
            font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
            font_size: 14.0,
        },
        overrides,
        Children::spawn(children),
    )
}

#[derive(Component)]
#[component(on_insert = resolve_path )]
pub struct SlicedImage {
    pub image: HandleOrPath<Image>,
    pub slicer: TextureSlicer,
}

fn resolve_path(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let sliced_image = world.get::<SlicedImage>(entity).unwrap();
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    let image = match sliced_image.image.clone() {
        HandleOrPath::Handle(handle) => handle,
        HandleOrPath::Path(ref path) => asset_server.load(path.clone()),
    };

    let slicer = sliced_image.slicer.clone();
    let image_node = world.get::<ImageNode>(entity).cloned().unwrap_or_default();
    world
        .commands()
        .entity(entity)
        .insert(ImageNode {
            image,
            image_mode: NodeImageMode::Sliced(slicer),
            ..image_node
        })
        .remove::<SlicedImage>();
}
