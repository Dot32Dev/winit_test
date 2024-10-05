use std::env::current_dir;
use std::fs;

use wgpu::Origin3d;

use crate::renderer_backend::bind_group;

pub struct Texture {
    pub bind_group: wgpu::BindGroup,
}

impl Texture {
    pub fn new(
        filename: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let mut filepath = current_dir().unwrap();
        filepath.push("src/textures/");
        filepath.push(filename);
        let filepath = filepath.into_os_string().into_string().unwrap();
        println!("{}", filepath);
        let bytes = fs::read(filepath).expect("Cant read texture file");

        let image = image::load_from_memory(&bytes)
            .expect("Texture file is not an image")
            .to_rgba8();
        let size = image.dimensions();
        println!("size: {:?}", size);

        let texture_size = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some(filename),
            mip_level_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            size: texture_size,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST, // To copy the image into it
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        };

        let texture = device.create_texture(&texture_descriptor);
        // Copy the texture
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(size.0 * 4),
                rows_per_image: Some(size.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler_descriptor = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_descriptor);

        let mut builder = bind_group::Builder::new(device, layout);
        builder.add_material(&view, &sampler);
        let bind_group = builder.build(filename);

        Self { bind_group }
    }
}
