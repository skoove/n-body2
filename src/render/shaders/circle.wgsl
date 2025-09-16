var<private> VERTICES: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    // vec2<f32>(-1.7321,-1.0),
    // vec2<f32>( 1.7321,-1.0), // sqrt(3) ≈ 1.7321
    // vec2<f32>( 0.0   , 2.0),
    vec2<f32>(-1.0,-1.0),
    vec2<f32>( 1.0,-1.0),
    vec2<f32>( 0.0, 1.0),
);

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var out: VertexOutput;
    let local = VERTICES[in_vertex_index];
    out.clip_position = vec4<f32>(local.x, local.y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
