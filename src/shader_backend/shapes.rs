use nannou::prelude::*;
use nannou::wgpu::Device;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]

pub struct Vertex {
    pub position:   [f32; 3],
    pub tex_coords: [f32; 2], // NEW!
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   &[
                wgpu::VertexAttribute {
                    offset:          0,
                    shader_location: 0,
                    format:          wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset:          std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format:          wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}

pub struct Rectangle {
    pub xy: Vec2,
    pub wh: Vec2,

    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices:  u32,
    pub index_buffer:  wgpu::Buffer,
    pub num_indices:   u32,
}

impl Rectangle {
    pub fn new(app: &App, xy: Vec2, wh: Vec2) -> ShapeBuffer {
        let binding = app.main_window();
        let device = binding.device();
        let points = Rectangle::xywh_into_shape_points(xy, wh);
        let num_vertices = points.vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label:    Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&points.vertices),
                usage:    wgpu::BufferUsages::VERTEX,
            }),
        );

        let num_indices = points.indices.len() as u32;
        let index_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label:    Some("Index Buffer"),
                contents: bytemuck::cast_slice(&points.indices),
                usage:    wgpu::BufferUsages::INDEX,
            }),
        );
        ShapeBuffer { vertex_buffer, num_vertices, index_buffer, num_indices }
    }
    pub fn default(app: &App) -> ShapeBuffer { Rectangle::new(app, vec2(0.0, 0.0), vec2(1.0, 1.0)) }

    fn xywh_into_shape_points(xy: Vec2, wh: Vec2) -> ShapePoints {
        let x = xy.x;
        let y = xy.y;
        let w = wh.x;
        let h = wh.y;

        let right_edge = x - w / 1.0;
        let left_edge = x + w / 1.0;
        let top_edge = y - h / 1.0;
        let bottom_edge = y + h / 1.0;

        let out = ShapePoints {
            vertices: vec![
                Vertex { position: [right_edge, top_edge, 0.0], tex_coords: [0.0, 1.0] }, // A
                Vertex { position: [left_edge, top_edge, 0.0], tex_coords: [1.0, 1.0] },  // B
                Vertex { position: [left_edge, bottom_edge, 0.0], tex_coords: [1.0, 0.0] }, // C
                Vertex { position: [right_edge, bottom_edge, 0.0], tex_coords: [0.0, 0.0] }, // D
            ],

            indices: vec![
                0, 1, 3, //1
                1, 2, 3, //2
            ],
        };
        out
    }
}

pub struct ShapePoints {
    pub vertices: Vec<Vertex>,
    pub indices:  Vec<u16>,
}

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
pub struct ShapeBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices:  u32,
    pub index_buffer:  wgpu::Buffer,
    pub num_indices:   u32,
}
impl ShapeBuffer {
    pub fn new(device: &Device, points: ShapePoints) -> ShapeBuffer {
        let num_vertices = points.vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label:    Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&points.vertices),
                usage:    wgpu::BufferUsages::VERTEX,
            }),
        );

        let num_indices = points.indices.len() as u32;

        let index_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label:    Some("Index Buffer"),
                contents: bytemuck::cast_slice(&points.indices),
                usage:    wgpu::BufferUsages::INDEX,
            }),
        );
        ShapeBuffer { vertex_buffer, num_vertices, index_buffer, num_indices }
    }
}
