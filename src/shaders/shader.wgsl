struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) colour: vec3<f32>,
}

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = vec4<f32>(vertex.position, 1.0);
    out.colour = vertex.colour;

    return out;
}

@fragment
fn fs_main(input: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(input.colour, 1.0);
}
