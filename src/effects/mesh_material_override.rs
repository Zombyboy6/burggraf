use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Clone, Debug)]
#[component(on_remove = Self::on_remove)]
#[component(on_insert= Self::on_insert)]
#[component(on_replace = Self::on_replace)]
pub struct MeshMaterialOverride<T, M>
where
    T: Material,
    M: Material,
{
    pub material: Handle<T>,
    previous_material: Handle<M>,
}

impl<T, M> MeshMaterialOverride<T, M>
where
    T: Material,
    M: Material,
{
    pub fn new(material: Handle<T>) -> Self {
        Self {
            material,
            previous_material: Handle::<M>::default(),
        }
    }

    fn on_remove(mut world: DeferredWorld, hook_context: HookContext) {
        let Some(override_mat) = world
            .get::<MeshMaterialOverride<T, M>>(hook_context.entity)
            .cloned()
        else {
            return;
        };

        if override_mat.previous_material == Handle::<M>::default() {
            return;
        }

        world
            .commands()
            .entity(hook_context.entity)
            .insert(MeshMaterial3d(override_mat.previous_material));
    }
    fn on_insert(mut world: DeferredWorld, hook_context: HookContext) {
        let Some(mat) = world.get::<MeshMaterial3d<M>>(hook_context.entity).cloned() else {
            return;
        };
        let Some(mut override_mat) =
            world.get_mut::<MeshMaterialOverride<T, M>>(hook_context.entity)
        else {
            return;
        };

        override_mat.previous_material = mat.0;
        let new_mat = override_mat.material.clone();
        world
            .commands()
            .entity(hook_context.entity)
            .insert(MeshMaterial3d(new_mat));
    }

    fn on_replace(mut world: DeferredWorld, hook_context: HookContext) {
        let Some(override_mat) = world
            .get::<MeshMaterialOverride<T, M>>(hook_context.entity)
            .cloned()
        else {
            return;
        };
        let Some(mut mat) = world.get_mut::<MeshMaterial3d<M>>(hook_context.entity) else {
            return;
        };
        mat.0 = override_mat.previous_material;
    }
}
