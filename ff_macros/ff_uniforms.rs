// use ff_macros::StructToWgsl;

// #[repr(C)]
// #[derive(Debug, Copy, Clone, bytemuck::Pod, StructToWgsl,
// bytemuck::Zeroable)]

// pub struct MouseUniformRaw {
//     x: f32,
//     y: f32,
// }

// pub trait UniformStruct {
//     fn field_names() -> (&'static [&'static str], &'static [&'static str]);
//     fn struct_name() -> &'static str;
//     fn new_wgsl_block(group: i32, binding: i32) -> String;
// }
