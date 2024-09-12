@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@fragment
fn main(@location(0) in_tex_coord: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in_tex_coord);
}

