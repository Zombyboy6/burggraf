#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_view_bindings::view
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct LeafMaterialExtension {
    fade_angle: f32,
    cluster_center: vec3<f32>,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // Web examples WebGL2 support: structs must be 16 byte aligned.
    _webgl2_padding_8b: u32,
    _webgl2_padding_12b: u32,
    _webgl2_padding_16b: u32,
#endif
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> leaf_material_extension: LeafMaterialExtension;

fn calculate_fade(world_normal: vec3<f32>, world_position: vec3<f32>, camera_position: vec3<f32>, fade_angle: f32) -> f32 {
    let view_direction = normalize(camera_position - world_position);

    let dot_product = abs(dot(world_normal, view_direction));

    let fade_threshold = 1.0 - fade_angle;

    let fade_factor = smoothstep(0.0, fade_threshold, dot_product);

    return fade_factor;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    let camera_pos = view.world_position.xyz ;

    let fade_factor = calculate_fade(
        in.world_normal,
        in.world_position.xyz,
        camera_pos,
        leaf_material_extension.fade_angle,
    );

    // Point normal vector away from center of leaf cluster
    var new_in = in;
    new_in.world_normal = normalize(( in.world_position.xyz) - leaf_material_extension.cluster_center.xyz);

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(new_in, is_front);

    pbr_input.material.base_color.a *= fade_factor;
    //pbr_input.material.base_color = vec4(new_in.world_normal, 1.0);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(new_in, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

#endif

    return out;
}
