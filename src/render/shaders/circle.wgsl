var<private> VERTICES: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(-1.7321,-1.0),
    vec2<f32>( 1.7321,-1.0), // sqrt(3) ≈ 1.7321
    vec2<f32>( 0.0   , 2.0),
    // vec2<f32>(-1.0,-1.0),
    // vec2<f32>( 1.0,-1.0),
    // vec2<f32>( 0.0, 1.0),
);

struct VertexOutput {
    @builtin(position) clip_space: vec4<f32>,
};

struct Camera {
    position: vec2<f32>,
    scale: f32,
    xy: u32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var out: VertexOutput;

    let x_pixels = f32((camera.xy       ) & 0xffffu);
    let y_pixels = f32((camera.xy >> 16u) & 0xffffu);
    // let aspect = f32(y) / f32(x);

    let world_space = (VERTICES[in_vertex_index] + camera.position) / camera.scale;

    let ndc_x = (world_space.x / x_pixels) * 2.0;
    let ndc_y = (world_space.y / y_pixels) * 2.0;

    let clip_space = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);

    out.clip_space = clip_space;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
