mod button;
mod window;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    feathers::handle_or_path::HandleOrPath,
    prelude::*,
};
pub use button::button;
pub(crate) use button::button_hover;
pub use window::window;

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
