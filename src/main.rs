// #![feature(trace_macros)]
#[cfg(feature = "std")]
//imports ------------------
use std::path::Path;
use std::{env, fs, io, usize};

use nannou::image::open;
use nannou::prelude::*;
use nannou::wgpu::Texture;

pub mod shader_ui;
use shader_ui::FluffUi;

pub mod sub_divide;
use sub_divide::SubdivideExt;

pub mod serial_handler;
use serial_handler::SerialHandler;

pub use serial2::SerialPort;

use ascii::AsAsciiStr;

//Constants -----
const PORT_DEFAULT: &str = "/dev/tty";
pub const BAUDRATE: u32 = 115200;
pub const SERIAL_DEBUG: bool = true;

//code ---------
fn main() { nannou::app(controller).update(update).run(); }

struct Model {
    ui:    FluffUi,
    count: i32,

    port: SerialHandler,
}

fn controller(app: &App) -> Model {
    let row_lables: Vec<&str> = vec![
        "invert_x_0",
        "invert_x_1",
        "invert_x_2",
        "invert_x_3",
        "invert_x_4",
        "invert_x_5",
        "invert_x_6",
        "invert_x_7",
        "invert_x_8",
        "invert_y_0",
        "invert_y_1",
        "invert_y_2",
        "invert_y_3",
        "invert_y_4",
        "invert_y_5",
        "invert_y_6",
        "invert_y_7",
        "invert_y_8",
    ];
    let col_lables: Vec<&str> = vec![
        "counter_x_0",
        "counter_x_1",
        "counter_x_2",
        "counter_x_3",
        "counter_x_4",
        "counter_x_5",
        "counter_x_6",
        "counter_x_7",
        "counter_x_8",
        "counter_y_0",
        "counter_y_1",
        "counter_y_2",
        "counter_y_3",
        "counter_y_4",
        "counter_y_5",
        "counter_y_6",
        "counter_y_7",
        "counter_y_8",
    ];

    //setup window and device
    let wgpu_limits = wgpu::Limits { max_bind_groups: 8, ..Default::default() };

    let device = wgpu::DeviceDescriptor {
        label:    Some("My Device"),
        features: wgpu::Features::empty(),
        limits:   wgpu_limits,
    };

    app.new_window()
        .size(1800, 900)
        .device_descriptor(device)
        // .mouse_wheel(scroll_event)
        .event(event_fn)
        .msaa_samples(1)
        .view(view)
        .build()
        .unwrap();

    let window = app.main_window();
    let (x, y) = window.inner_size_pixels();
    let dim = UVec2::new(x, y);

    //serial stuff
    SerialHandler::print_avaliable_ports();
    let args: Vec<_> = env::args().collect();

    let port_name = if args.len() > 1 { &args[1] } else { PORT_DEFAULT };
    println!("attempting to open port: {}", port_name);
    let mut port = SerialHandler::new(port_name, BAUDRATE, SERIAL_DEBUG);

    //setup shader model
    let path = app.assets_path().unwrap().join("happy-tree.png");
    let image = open(path).unwrap();
    let image_texture = Texture::from_image(app, &image);

    //half screen bounds
    let bounds = app.window_rect().pad(100.0);
    let bounds = bounds.divide_rows_cols(1, 2);
    let bounds = *bounds.iter().flatten().skip(1).next().unwrap();

    //setup shadewr model
    let path = app.assets_path().unwrap().join("fragments");

    let shader_paths = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|file| file.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    // let shader_model = ShaderModel::new(shader_paths, &app, &image_texture);

    let ui = FluffUi::new(app, &row_lables, &col_lables);

    Model { ui, count: 30, port }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.ui.update(app);

    let output_string = model.ui.get_serial_output(app);

    let ascii = output_string.as_ascii_str().unwrap();
    // print!("{}", ascii);

    model.port.write(ascii);
    // if !output_string.is_empty() {

    // }

    // shader stuff
}

fn event_fn(app: &App, model: &mut Model, event: WindowEvent) {
    model.ui.event_handler(app, &event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = &app.draw();

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();

    model.ui.draw(app, &frame);
}
