use bevy::{
    ecs::{lifecycle::HookContext, spawn::SpawnableList, world::DeferredWorld},
    feathers::{font_styles::InheritableFont, handle_or_path::HandleOrPath},
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui::{InteractionDisabled, Pressed},
    ui_widgets::Button,
};

use crate::{
    constants::fonts,
    ui::{PAPER_SLICER, PAPER_THICK_SLICER},
};

pub fn button<C: SpawnableList<ChildOf> + Send + Sync + 'static, B: Bundle>(
    overrides: B,
    children: C,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        Button,
        Hovered::default(),
        //EntityCursor::System(bevy_window::SystemCursorIcon::Pointer),
        TabIndex(0),
        overrides,
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(px(8.0)).with_bottom(px(4.0)),
                //flex_grow: 1.0,
                ..Default::default()
            },
            InheritableFont {
                font: HandleOrPath::Path(fonts::REGULAR.to_owned()),
                font_size: 12.0,
            },
            SlicedImage {
                image: HandleOrPath::Path("textures/ui/paper_thick.png".to_string()),
                slicer: TextureSlicer {
                    border: BorderRect {
                        left: 5.0,
                        right: 5.0,
                        top: 5.0,
                        bottom: 7.0,
                    },
                    center_scale_mode: SliceScaleMode::Stretch,
                    sides_scale_mode: SliceScaleMode::Stretch,
                    max_corner_scale: 1.0,
                },
            },
            Children::spawn(children)
        )],
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

pub fn button_hover(
    mut button: Query<
        (
            Entity,
            &Children,
            &mut Node,
            Option<Ref<Pressed>>,
            Option<Ref<InteractionDisabled>>,
            Ref<Hovered>,
        ),
        Without<ImageNode>,
    >,
    mut image_query: Query<(&mut ImageNode, &mut Node)>,
    mut removed_pressed: RemovedComponents<Pressed>,
    mut removed_disabled: RemovedComponents<InteractionDisabled>,
    asset_server: Res<AssetServer>,
) {
    for (entity, children, mut node, pressed, disabled, hovered) in button.iter_mut() {
        let child_entity = children
            .iter()
            .find(|e| image_query.get(*e).is_ok())
            .unwrap();
        let mut image = image_query.get_mut(child_entity).unwrap();
        let is_pressed = pressed.is_some();
        let is_disabled = disabled.is_some();

        if !pressed.is_none_or(|x| x.is_changed())
            && !disabled.is_none_or(|x| x.is_changed())
            && !hovered.is_changed()
            && !removed_pressed.read().any(|e| e == entity)
            && !removed_disabled.read().any(|e| e == entity)
        {
            continue;
        }

        set_button_style(
            is_disabled,
            hovered.0,
            is_pressed,
            (&mut image.0, &mut image.1),
            &mut node,
            &asset_server,
        );
    }
}

fn set_button_style(
    disabled: bool,
    hovered: bool,
    pressed: bool,
    image: (&mut ImageNode, &mut Node),
    node: &mut Node,
    asset_server: &AssetServer,
) {
    match (disabled, hovered, pressed) {
        // Disabled
        (true, _, _) => {
            image.0.color = Color::linear_rgb(0.5, 0.5, 0.5);
        }
        // Pressed and hovered
        (false, true, true) => {
            image.0.color = Color::linear_rgb(0.8, 0.8, 0.8);
            image.0.image = asset_server.load("textures/ui/paper.png");
            image.0.image_mode = NodeImageMode::Sliced(PAPER_SLICER);
            image.1.padding = image.1.padding.with_bottom(px(0.0));
            node.padding = node
                .padding
                .with_top(px(4.0))
                .with_left(px(2))
                .with_right(px(2));
        }
        // Hovered, not pressed
        (false, true, false) => {
            image.0.color = Color::linear_rgb(1.2, 1.2, 1.2);
            image.0.image = asset_server.load("textures/ui/paper_thick.png");
            image.0.image_mode = NodeImageMode::Sliced(PAPER_THICK_SLICER);
            image.1.padding = image.1.padding.with_bottom(px(4.0));
            node.padding = node
                .padding
                .with_top(px(0.0))
                .with_left(px(0))
                .with_right(px(0));
        }
        // Not hovered
        (false, false, _) => {
            image.0.color = Color::WHITE;
        }
    }
}
