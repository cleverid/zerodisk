@binding(0) @group(0) var<uniform> viewport : vec2<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var x = (model.position[0] / viewport[0]) - 1.0;
    var y = 1.0 - (model.position[1] / viewport[1]);
    var out: VertexOutput;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
