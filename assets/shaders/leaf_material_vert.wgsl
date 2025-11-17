#import bevy_pbr::{
    mesh_bindings::mesh,
    mesh_view_bindings::view,
    mesh_functions,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput},
    view_transformations::{position_world_to_clip, position_world_to_view, position_view_to_clip},
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.world_normal = mesh_functions::mesh_normal_local_to_world(
            vertex.normal,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex.instance_index
        );


    let base_world_position = mesh_functions::mesh_position_local_to_world(
        mesh_functions::get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0)
    );

    let billboard_size = fract(sin(vertex.position.x + vertex.position.y + vertex.position.z) * 43758.5453123) * 0.7;
    let uv_scaled = (1.0 - vertex.uv) * 2.0 - 1.0;
    let offset = vec4<f32>(-uv_scaled.x, uv_scaled.y, 0.0, 0.0) * billboard_size;

    out.world_position = view.world_from_view * offset  + base_world_position;

    out.position = position_world_to_clip(out.world_position.xyz);

    out.uv = vertex.uv;

    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
#ifdef VERTEX_COLORS
    return mesh.color;
#else
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
#endif
}
