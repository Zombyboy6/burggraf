use bevy::{
    pbr::MaterialExtension, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
};
const SHADER_ASSET_PATH: &str = "shaders/leaf_material_extension.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct LeafMaterialExtension {
    #[uniform(100)]
    pub fade_angle: f32,
    #[uniform(100)]
    pub cluster_center: Vec3,
    // Web examples WebGL2 support: structs must be 16 byte aligned.
    /*
    #[cfg(feature = "webgl2")]
    #[uniform(100)]
    _webgl2_padding_8b: u32,
    #[cfg(feature = "webgl2")]
    #[uniform(100)]
    _webgl2_padding_12b: u32,
    #[cfg(feature = "webgl2")]
    #[uniform(100)]
    _webgl2_padding_16b: u32,
    */
}

impl MaterialExtension for LeafMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
