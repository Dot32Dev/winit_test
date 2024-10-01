use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        // No idea why this has to be a const
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

#[rustfmt::skip] // For nicer formatting of the vertices array
pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: [Vertex; 3] = [
        Vertex {position: [0.75,  -0.75, 0.0], colour: [1.0, 0.0, 0.0]},
        Vertex {position: [0.0,    0.75, 0.0], colour: [0.0, 1.0, 0.0]},
        Vertex {position: [-0.75, -0.75, 0.0], colour: [0.0, 0.0, 1.0]},
    ];

    let buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Vertex buffer"),
        contents: unsafe { any_as_u8_slice(&vertices) }, // Vertices to u8 slice
        usage: wgpu::BufferUsages::VERTEX,
    };

    device.create_buffer_init(&buffer_descriptor)
}
