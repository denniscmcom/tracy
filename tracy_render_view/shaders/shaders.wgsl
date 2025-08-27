@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var sampl: sampler;

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );

    return vec4<f32>(pos[idx], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
	let texture_dim = vec2<f32>(textureDimensions(texture, 0));

    if pos.x >= texture_dim.x || pos.y >= texture_dim.y {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let uv = pos.xy / texture_dim;
    return textureSample(texture, sampl, uv);
}
