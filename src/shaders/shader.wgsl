struct Uniforms {
    rows: u32,
    cols: u32,
}

struct VertexOut {
    @builtin(position) pos: vec4f,
    @location(0) color: vec4f,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;
@group(0) @binding(1)
var<storage, read> positions: array<vec2f>;
@group(0) @binding(2)
var<storage, read> colors: array<vec4f>;

@vertex
fn vert_main(
    @builtin(instance_index) instance_index: u32,
    @location(0) pos: vec2f,
) -> VertexOut {
    let scale_y: f32 = -2.0 / f32(uniforms.rows);
    let scale_x: f32 = 2.0 / f32(uniforms.cols);
    var position = pos;
    position += vec2f(positions[instance_index]);
    position *= vec2f(scale_x, scale_y);
    position += vec2f(-1.0, 1.0);

    var out: VertexOut;
    out.pos = vec4f(position, 0.0, 1.0);
    out.color = colors[instance_index];
    //var gray = f32(instance_index)/256.0;
    //out.color = vec3<f32>(gray, gray, gray);
    return out;
}

@fragment
fn frag_main(
    @location(0) color: vec4f,
) -> @location(0) vec4f {
    return vec4f(color);
}
