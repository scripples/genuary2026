#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}

#import bevy_render::globals::Globals;
@group(0) @binding(1) var<uniform> globals: Globals;


@group(#{MATERIAL_BIND_GROUP}) @binding(0) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    var viewport_uv = coords_to_viewport_uv(mesh.position.xy, view.viewport);

    var texture_in = textureSample(base_color_texture, base_color_sampler, viewport_uv);

    let warpSpeed = 0.1;
    let warpDistance = 0.02;

    let luma = vec3(0.299, 0.587, 0.114) / 4;

    var power: f32 = dot(vec3(texture_in.x, texture_in.y, texture_in.z), luma);

    power = sin(3.1415927 * 2.0 * ((power + globals.time * warpSpeed) % 1.0));

    viewport_uv = viewport_uv + vec2(0, power) * warpDistance;

    var texture = textureSample(base_color_texture, base_color_sampler, viewport_uv);

    return texture;
}
