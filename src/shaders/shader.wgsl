@group(0) @binding(0) var my_texture: texture_2d<f32>;
@group(0) @binding(1) var my_sampler: sampler;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) colour: vec3<f32>,
}

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) texture_coordinate: vec2<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = vec4<f32>(vertex.position, 1.0);
    out.colour = vertex.colour;
    out.texture_coordinate = vec2<f32>((vertex.position.x + 1)*0.5, (-vertex.position.y + 1)*0.5);

    return out;
}

@fragment
fn fs_main(input: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(input.colour, 1.0) * textureSample(my_texture, my_sampler, input.texture_coordinate);
}
