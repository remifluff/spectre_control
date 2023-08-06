use bytemuck::Pod;
// use ff_macros::StructToWgsl;

use nannou::prelude::*;
use nannou::wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Device, TextureView,
};

pub trait BindGroupSet {
    fn get_bind_group(&self, device: &Device) -> (BindGroupLayout, BindGroup);
    fn get_wgsl_blob(&self, group_number: i32) -> String;
}

trait BindObject {
    fn get_bind_group_entry(&self, binding: u32, group: u32) -> BindGroupEntry;
    fn get_bind_group_layout_entry(&self, binding: u32, group: u32) -> BindGroupLayoutEntry;
    fn get_wgsl_blob(&self, binding: u32, group: u32) -> String;
}

impl BindGroupSet for TextureView {
    fn get_bind_group(&self, device: &Device) -> (BindGroupLayout, BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding:    0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty:         wgpu::BindingType::Texture {
                        multisampled:   false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type:    wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count:      None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding:    1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty:         wgpu::BindingType::Sampler { filtering: true, comparison: false },
                    count:      None,
                },
            ],
            label:   Some("texture_bind_group_layout"),
        });

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &layout,
            label:   Some("diffuse_bind_group"),
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: wgpu::BindingResource::TextureView(self),
                },
                wgpu::BindGroupEntry {
                    binding:  1,
                    resource: wgpu::BindingResource::Sampler(&device.create_sampler(
                        &wgpu::SamplerDescriptor {
                            address_mode_u: wgpu::AddressMode::ClampToEdge,
                            address_mode_v: wgpu::AddressMode::ClampToEdge,
                            address_mode_w: wgpu::AddressMode::ClampToEdge,
                            mag_filter: wgpu::FilterMode::Linear,
                            min_filter: wgpu::FilterMode::Nearest,
                            mipmap_filter: wgpu::FilterMode::Nearest,
                            ..Default::default()
                        },
                    )),
                },
            ],
        });
        (layout, group)
    }

    fn get_wgsl_blob(&self, group_number: i32) -> String {
        let mut wgsl = String::new();
        wgsl.push_str(&format!("[[group({}), binding(0)]]\n", group_number));
        wgsl.push_str(&format!("var tex{}: texture_2d<f32>;\n", group_number));
        wgsl.push_str(&format!("[[group({}), binding(1)]]\n", group_number));
        wgsl.push_str(&format!("var tex_sampler{}: sampler;\n", group_number));
        // wgsl.push_str(&format!(
        //     "let texture{} = textureSample(tex{}, tex_sampler{}, in.tex_coords);\n",
        //     group_number, group_number, group_number
        // ));

        //     let texture1 = textureSample(tex1, tex_sampler1, in.tex_coords);

        wgsl
    }
}

trait UniformStruct {
    fn struct_name() -> &'static str;
    fn field_names() -> &'static str;

    fn get_uniform_wgsl(&self, group: i32, binding: i32) -> String {
        let struct_name = Self::struct_name();
        let mut wgsl = String::new();
        wgsl.push_str(&format!("[[block]]\n"));
        wgsl.push_str(&format!("struct {} {{ \n", struct_name));
        wgsl.push_str(Self::field_names());
        wgsl.push_str(&format!("}}; \n"));

        wgsl.push_str(&format!(
            "[[group({}), binding({})]] \n var<uniform> {}: {}; \n",
            group,
            binding,
            struct_name.to_lowercase(),
            struct_name
        ));
        wgsl
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]

pub struct ParameterSet {
    input: [f32; 8],
}

impl ParameterSet {
    pub fn new(default: f32) -> Self { Self { input: [default; 8] } }
}

impl UniformStruct for ParameterSet {
    fn struct_name() -> &'static str { "ParameterSet" }
    fn field_names() -> &'static str { "input: array<f32, 8>;" }
}
pub struct ParameterPrimitive {
    params:    ParameterSet,
    pub name:  String,
    index:     u32,
    row_count: u32,
}

impl ParameterPrimitive {
    pub fn new(name: &str, index: u32, row_count: u32) -> Self {
        Self { params: ParameterSet::new(0.1), name: name.to_string(), index, row_count }
    }
    pub fn get_mut_val(&mut self) -> Vec<&mut f32> {
        let mut x: Vec<&mut f32> = vec![];

        for i in &mut self.params.input {
            x.push(i);
        }

        x
    }

    pub fn get_val(&mut self) -> Vec<f32> {
        let mut x: Vec<f32> = vec![];

        for i in self.params.input {
            x.push(i);
        }

        x
    }

    pub fn update(&mut self) -> () {
        let x = self.get_mut_val();
        for i in x {
            *i = random_f32();
        }
    }
    fn create_parameter_sampler(&self, input_count: u32) -> String {
        let parameter_index = self.index;
        let mut string = String::new();
        string.push_str(&format!(
            "fn PARAMATER{}(location: vec2<f32>) -> vec4<f32>{{ \n",
            parameter_index
        ));

        string.push_str(&format!("var param = vec4<f32>(0.0, 0.0, 0.0, 0.0); \n"));

        for i in 0..input_count {
            string.push_str(&format!(
                "var sample{} = textureSample(tex{}, tex_sampler{}, location); \n",
                i, i, i
            ));
            string.push_str(&format!("sample{} = sample{} * parameterset.input[{}]; \n", i, i, i));

            string.push_str(&format!("param = param + sample{}; \n", i));
        }

        string.push_str(&format!("return param; }} \n"));

        string
    }

    // pub fn draw() -> () {}
}

impl BindGroupSet for ParameterPrimitive {
    fn get_bind_group(&self, device: &Device) -> (BindGroupLayout, BindGroup) {
        self.params.get_bind_group(device)
    }

    fn get_wgsl_blob(&self, group_number: i32) -> String {
        let mut string = String::new();
        string.push_str(&self.params.get_uniform_wgsl(group_number, 0));
        string.push_str(&self.create_parameter_sampler(4));
        string
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]

pub struct MouseUniform {
    x: f32,
    y: f32,
}
impl UniformStruct for MouseUniform {
    fn struct_name() -> &'static str { "MouseUniform" }
    fn field_names() -> &'static str { "x: f32; y: f32;" }
}

impl MouseUniform {
    pub fn new(app: &App, dim: UVec2) -> Self {
        let mouse = app.mouse.position();
        let mouse = mouse / dim.as_f32();

        Self { x: mouse.x, y: mouse.y * -1.0 }
    }
    pub fn update_mouse(&mut self, app: &App) {
        let (x, y) = app.main_window().inner_size_pixels();
        let dim = UVec2::new(x, y);

        let mouse = app.mouse.position();
        let mouse = mouse / dim.as_f32();

        self.x = mouse.x;
        self.y = mouse.y * -1.0;
    }
}

impl<T> BindGroupSet for T
where
    T: UniformStruct + Pod,
{
    fn get_bind_group(&self, device: &Device) -> (BindGroupLayout, BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty:         wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count:      None,
            }],
            label:   Some("mouse_bind_group_layout"),
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some(&format!("Buffer")),
            contents: bytemuck::cast_slice(&[*self]),
            usage:    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &bind_group_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
            label:   Some("mouse_bind_group"),
        });
        (bind_group_layout, bind_group)
    }
    fn get_wgsl_blob(&self, group_number: i32) -> String { self.get_uniform_wgsl(group_number, 0) }
}
