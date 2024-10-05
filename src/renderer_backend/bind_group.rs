use wgpu::ShaderStages;

pub struct Builder<'a> {
    entries: Vec<wgpu::BindGroupEntry<'a>>,
    layout: &'a wgpu::BindGroupLayout,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new(
        device: &'a wgpu::Device,
        layout: &'a wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            entries: Vec::new(),
            layout,
            device,
        }
    }

    fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn add_material(
        &mut self,
        texture_view: &'a wgpu::TextureView,
        sampler: &'a wgpu::Sampler,
    ) {
        // Texture
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::TextureView(texture_view),
        });

        // Sampler
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::Sampler(sampler),
        });
    }

    pub fn build(&mut self, label: &str) -> wgpu::BindGroup {
        let bind_group =
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &self.layout,
                entries: &self.entries,
            });
        self.reset();

        bind_group
    }
}
