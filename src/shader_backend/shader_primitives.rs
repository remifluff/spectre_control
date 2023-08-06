use std::sync::Arc;

use nannou::prelude::*;
use nannou::wgpu::{
    BindGroupLayout, CommandEncoderDescriptor, Device, ImageCopyTexture, RenderPipeline,
    ShaderModule, Texture, TextureDescriptor, TextureFormat, TextureUsages,
};

use super::shapes;

pub struct ShaderPrimitive {
    pub shader_source: String,
    pub shader_name:   String,
    // shader_module:     ShaderModule,
}

// impl ShaderPrimitive {
//     pub fn new(app: &App, path: &PathBuf) -> ShaderPrimitive {
//         // let shader_module =
// device.create_shader_module(&wgpu::ShaderModuleDescriptor         // {
// label:  Some(&shader_name),         //     source:
// wgpu::ShaderSource::Wgsl(shader_source.clone().into()),         // });

//         ShaderPrimitive { shader_source, shader_name }
//     }

//     pub fn source(&self) -> &String { &self.shader_source }
//     pub fn name(&self) -> &String { &self.shader_name }

//     // pub fn shader_module(&self, device: &Device) -> ShaderModule {}
// }

pub fn create_pipeline(
    device: &Device,
    shader_module: &ShaderModule,
    outpute_texture_format: &TextureFormat,
    bind_group_layouts: &[&BindGroupLayout],
    samples: u32,
) -> RenderPipeline {
    // let shader = self.shader_module(device);

    let vertex = wgpu::VertexState {
        module:      shader_module,
        entry_point: "main",                    // 1.
        buffers:     &[shapes::Vertex::desc()], // 2.
    };

    let fragment = wgpu::FragmentState {
        module:      shader_module,
        entry_point: "main",
        targets:     &[wgpu::ColorTargetState {
            format:     *outpute_texture_format,
            blend:      Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        }],
    };

    let render_pipeline_layout = device.create_pipeline_layout(
        &(wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        }),
    );
    let primitive = wgpu::PrimitiveState {
        topology:           wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face:         wgpu::FrontFace::Ccw,
        cull_mode:          Some(wgpu::Face::Back),
        polygon_mode:       wgpu::PolygonMode::Fill,
        conservative:       false,
        clamp_depth:        false,
    };

    device.create_render_pipeline(
        &(wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex,
            fragment: Some(fragment),
            primitive,
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: samples,                   // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
        }),
    )
}

pub fn texture_desc(
    texture_w: u32,
    texture_h: u32,
    texture_format: TextureFormat,
) -> TextureDescriptor<'static> {
    wgpu::TextureDescriptor {
        size:            wgpu::Extent3d {
            width:                 texture_w,
            height:                texture_h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count:    1,
        dimension:       wgpu::TextureDimension::D2,
        format:          texture_format,
        usage:           wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST,
        label:           None,
    }
}

pub trait DuplicateExt {
    fn duplicate(&self, app: &App) -> Self;
}

impl DuplicateExt for Texture {
    fn duplicate(&self, app: &App) -> Self {
        let binding = app.main_window();
        let pair = binding.device_queue_pair();
        let (device, queue) = (pair.device(), pair.queue());

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        let desc = self.descriptor();

        let output_texture = device.create_texture(&TextureDescriptor {
            label:           Some("duplicate texture"),
            size:            desc.size,
            mip_level_count: desc.mip_level_count,
            sample_count:    desc.sample_count,
            dimension:       desc.dimension,
            format:          desc.format,
            usage:           TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC,
        });
        let output_texture =
            Texture::from_handle_and_descriptor(Arc::new(output_texture), desc.clone());

        encoder.copy_texture_to_texture(
            ImageCopyTexture {
                texture:   self,
                mip_level: 0,
                origin:    Default::default(),
                aspect:    self.view().build().aspect(),
            },
            ImageCopyTexture {
                texture:   &output_texture,
                mip_level: 0,
                origin:    Default::default(),
                aspect:    output_texture.view().build().aspect(),
            },
            desc.size,
        );
        queue.submit([encoder.finish()]);
        output_texture
    }
}
