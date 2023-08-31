
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) 
}

@vertex
fn main() {
    let adjust: mat4x4<f32> = mat4x4<f32>(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );


    var out: VertexOutput;
    // out.clip_position = adjust * camera.projection * camera.view * model * model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(0.2, 0.2, 0.2, 1.0);
}