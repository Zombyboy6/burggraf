use bevy::{
    pbr::MaterialExtension, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
};
const SHADER_ASSET_PATH: &str = "shaders/leaf_material_extension.wgsl";
const VERTEX_SHADER_ASSET_PATH: &str = "shaders/leaf_material_vert.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct LeafMaterialExtension {}

impl MaterialExtension for LeafMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn vertex_shader() -> ShaderRef {
        VERTEX_SHADER_ASSET_PATH.into()
    }
    fn deferred_vertex_shader() -> ShaderRef {
        VERTEX_SHADER_ASSET_PATH.into()
    }
}
