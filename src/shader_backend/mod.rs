use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use nannou::image::{open, DynamicImage};
use nannou::prelude::*;
use nannou::wgpu::{self, Texture, *};

pub mod bindgroups;
use bindgroups::*;

pub mod shader_primitives;
use shader_primitives::*;

pub mod shapes;
use crate::sub_divide;
use shapes::*;
use sub_divide::SubdivideExt;

pub struct ShaderModel {
    // draw_frames:          Vec<(usize, Rect)>,
    pub shader_fragments: Vec<Fragment>,
    outputs:              Option<Vec<(Texture, String)>>,
    // bounds:               Rect,
}

impl ShaderModel {
    pub fn new(
        shader_paths: Vec<PathBuf>,
        app: &App,
        sample_texture: &Texture,
        // bounds: Rect,
    ) -> ShaderModel {
        let win = app.main_window();

        let r = win.rect();
        let r = r.divide_rows_cols(1, 2);
        let r = r.iter().flatten().next().unwrap();

        let draw_frame = geom::Rect::from_xy_wh(
            (random::<Vec2>() * r.wh()) * 0.8 - r.wh() / 2.0,
            Vec2::splat(r.wh().max_element() * 0.3),
        );

        let fragements: Vec<Fragment> = shader_paths
            .iter()
            .map(|path| Fragment::new(app, sample_texture, path, UVec2::splat(512u32), draw_frame))
            .collect();

        ShaderModel { shader_fragments: fragements, outputs: None }
    }
    pub fn update(&mut self, app: &App) -> () { self.outputs = Some(self.get_textures(app)) }

    pub fn get_textures(&self, app: &App) -> Vec<(Texture, String)> {
        self.shader_fragments
            .iter()
            .map(|frag| (frag.output_texture.clone(), frag.name.to_owned()))
            .collect()
    }
    pub fn get_parameters(&self) -> Vec<&ParameterPrimitive> {
        self.shader_fragments.iter().flat_map(|frag| frag.params.iter()).collect()
    }
    pub fn get_mut_frames(&mut self) -> Vec<&mut Rect> {
        self.shader_fragments.iter_mut().map(|fragment| &mut fragment.draw_frame).collect()
    }
    pub fn get_mut_val(&mut self) -> Vec<Vec<&mut f32>> {
        self.shader_fragments
            .iter_mut()
            .flat_map(|frag| frag.params.iter_mut())
            .map(|param| param.get_mut_val())
            .collect()
    }

    pub fn draw(&self, app: &App, frame: &Frame, mouse_uniform: &MouseUniform, draw: &Draw) -> () {
        if let Some(s) = &self.outputs {
            let samples: Vec<_> = s.iter().map(|(t, n)| t.duplicate(app)).collect();
            self.shader_fragments
                .iter()
                .for_each(|frag| frag.render(frame, mouse_uniform, &samples));
        }

        for fragment in &self.shader_fragments {
            fragment.draw(draw)
        }

        draw.to_frame(app, frame).unwrap();
    }
}

pub struct Fragment {
    shape:          ShapeBuffer,
    samples:        Vec<TextureView>,
    output_texture: Texture,
    params:         Vec<ParameterPrimitive>,
    pub source:     String,
    pub name:       String,
    draw_frame:     Rect,
}

impl Fragment {
    pub fn new(
        app: &App,
        default_sample_texture: &Texture,
        shader_path: &PathBuf,
        dim: UVec2,
        draw_frame: Rect,
    ) -> Fragment {
        let binding = app.main_window();
        let device = binding.device();

        let shader_source = {
            let mut source = String::new();
            File::open(shader_path).unwrap().read_to_string(&mut source).unwrap();
            source
        };

        let params = Fragment::find_parameters(&shader_source);

        let shader_source = Fragment::import_wgsl_snippets(app, &shader_source);

        let shader_name =
            shader_path.file_name().and_then(|filename| filename.to_str()).unwrap().to_owned();

        Fragment {
            output_texture: Texture::from_image(app, &DynamicImage::new_rgba8(dim.x, dim.y)),
            shape: Rectangle::default(app),
            samples: vec![
                default_sample_texture.view().build(),
                default_sample_texture.view().build(),
            ],
            params,
            source: shader_source,
            name: shader_name,
            draw_frame,
        }
    }
    fn import_wgsl_snippets(app: &App, wgsl_code: &str) -> String {
        let lines = wgsl_code.lines().map(str::to_owned);

        // Parse the TOML configuration block from the beginning of the WGSL code
        let mut wgsl_code = String::new();

        let mut source = String::new();

        let path = app.assets_path().unwrap().join("utility_shaders/frag_utility.wgsl");
        File::open(path).unwrap().read_to_string(&mut source).unwrap();

        lines.for_each(|line| {
            if !line.starts_with("#import") {
                wgsl_code.push_str(line.as_str());
                wgsl_code.push('\n');
            } else {
                wgsl_code.push_str(source.as_str())
            }
        });

        wgsl_code
    }

    fn find_parameters(code: &str) -> Vec<ParameterPrimitive> {
        let mut params: Vec<ParameterPrimitive> = vec![];

        for i in 0..8 {
            let function_name = format!("PARAMATER{}", i);

            for line in code.lines() {
                if line.contains(&function_name) && line.contains('=') {
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    // && line.contains("=")
                    if let Some(name) = parts.get(1) {
                        if !name.contains('(') && !name.contains(')') {
                            print!("{} \n", name);
                            params.push(ParameterPrimitive::new(name, i, 8));

                            break;
                        }
                    }
                }
            }
        }
        params
    }
    pub fn draw(&self, draw: &Draw) {
        draw.texture(&self.output_texture).wh(self.draw_frame.wh()).xy(self.draw_frame.xy());
    }

    pub fn render(&self, frame: &Frame, mouse_uniform: &MouseUniform, samples: &[Texture]) {
        let output_texture = self.output_texture.view().build();

        let samples: Vec<_> = samples.iter().map(|s| s.view().build()).collect();
        // let samples = vec![self.samples[0].clone(), samples.view().build()];

        let device = frame.device_queue_pair().device();
        let clear_colour = None;

        let mut binds: Vec<&dyn BindGroupSet> = vec![];
        for t in samples.iter() {
            binds.push(t);
        }
        for p in self.params.iter() {
            binds.push(p);
        }
        binds.push(mouse_uniform);

        //create array of shader elements

        let (layouts, bind_group): (Vec<_>, Vec<_>) =
            binds.iter().map(|i| i.get_bind_group(device)).unzip();

        let string = binds
            .iter()
            .enumerate()
            .map(|(i, e)| e.get_wgsl_blob(i.as_()))
            .reduce(|cur: String, nxt: String| cur + &nxt)
            .unwrap();

        let shader_source = string + (&self.source);

        // print!("\n ------------------ \n{} \n ------------------ \n ",
        // shader_source);

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label:  Some(&self.name),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let shading_pipeline = create_pipeline(
            device,
            &shader_module,
            &output_texture.format(),
            &layouts.iter().collect::<Vec<&BindGroupLayout>>(),
            1,
        );
        // start render pass
        let mut encoder = frame.command_encoder();
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(&output_texture, |attatchment| {
                match clear_colour {
                    Some(colour) => attatchment.load_op(LoadOp::Clear(colour)).store_op(true),
                    None => attatchment.load_op(LoadOp::Load),
                }
                .store_op(true)
            })
            .begin(&mut encoder);

        for (i, bind_group) in bind_group.iter().enumerate() {
            render_pass.set_bind_group(i.as_(), bind_group, &[])
        }

        render_pass.set_pipeline(&shading_pipeline);
        render_pass.set_vertex_buffer(0, self.shape.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.shape.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.shape.num_indices, 0, 0..1);
    }
}

// ------------------------
