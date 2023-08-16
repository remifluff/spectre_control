use std::path::{self, Path};

use nannou::image::{DynamicImage, ImageBuffer};
use nannou::prelude::*;

use crate::sub_divide;
use hecs::*;
use std::iter;
use sub_divide::SubdivideExt;
use Orientation::*;
const line_weight: f32 = 2.0;
//componants
pub struct FluffUi {
    scrolling: bool,
    cursor:    Vec2,
    world:     World,

    screen: Rect,

    // cols_rows: UVec2,
    bounds:  Rect,
    rows:    u32,
    columns: u32,

    current_cell: Option<((usize, usize))>,

    // The texture that we will draw to.
    texture:          wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    draw:             nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer:         nannou::draw::Renderer,
    // The type used to capture the texture.
    texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    texture_reshaper: wgpu::TextureReshaper,
    dynamic_image:    DynamicImage,
}

impl FluffUi {
    pub fn new(app: &App, row_names: &Vec<&str>, col_names: &Vec<&str>) -> Self {
        let path = app.assets_path().unwrap().join("fonts/Inconsolata-Regular.ttf");
        let font = text::font::from_file(path).unwrap();

        let screen = app.window_rect();

        //downsample texture stuff
        let texture_size = [200, 180];

        let dynamic_image = DynamicImage::new_rgba8(800, 600);

        let mut world = World::new();

        let window = app.main_window();

        // Retrieve the wgpu device.
        let device = window.device();

        // Create our custom texture.
        let sample_count = window.msaa_samples();
        let texture = wgpu::TextureBuilder::new()
            .size(texture_size)
            // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
            // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            // Use nannou's default multisampling sample count.
            .sample_count(sample_count)
            // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
            .format(wgpu::TextureFormat::Rgba16Float)
            // Build it!
            .build(device);

        // Create our `Draw` instance and a renderer for it.
        let draw = nannou::Draw::new();
        let descriptor = texture.descriptor();
        let renderer =
            nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

        // Create the texture capturer.
        let texture_capturer = wgpu::TextureCapturer::default();

        // Create the texture reshaper.
        let texture_view = texture.view().build();
        let texture_sample_type = texture.sample_type();
        let dst_format = Frame::TEXTURE_FORMAT;
        let texture_reshaper = wgpu::TextureReshaper::new(
            device,
            &texture_view,
            sample_count,
            texture_sample_type,
            sample_count,
            dst_format,
        );

        let bounds = &screen.pad(100.0);
        // let bounds = bounds.first().unwrap();

        let default_bounds = Rect::from_w_h(10.0, 10.0);

        let (row_count, col_count) = (row_names.len() as u32 + 1, col_names.len() as u32 + 1);

        let default_bounds = Bounds { shape: default_bounds, update: false };
        let spawn_label = |name: &str, direction: Orientation| {
            (
                default_bounds,
                Title {
                    text:        name.to_string(),
                    orientation: direction,
                    fill:        Rgb::from_format(WHITE),
                },
                Stroke { weight: line_weight, colour: Rgb::from_format(WHITE) },
                Fill(Rgb::from_format(BLACK)),
            )
        };

        // make the colum of row titles

        let row_titles: Vec<_> =
            row_names.iter().map(|name| world.spawn(spawn_label(name, Vertical))).collect();

        let col_titles: Vec<_> =
            col_names.iter().map(|name| world.spawn(spawn_label(name, Horizontal))).collect();

        let mut more_column = vec![];

        for (i, col) in col_titles.iter().enumerate() {
            let mut column = vec![];

            // let title = world.spawn(spawn_label(name, Horizontal));

            column.push(*col);

            for (j, row) in row_titles.iter().enumerate() {
                let c1: Cell = Box::new(FloatCell { val: 0.0 });
                let c2: Cell = Box::new(BoolCell { val: false });

                let cell = world.spawn((
                    default_bounds,
                    c2,
                    SerialUpdate(true),
                    Index { row: i, column: j },
                    Focus(false),
                    Headings { row: *row, column: *col },
                    OnScroll(Box::new(|a| a + 1.0)),
                ));
                column.push(cell);
            }
            more_column.push(column);
        }
        let mut row_names = vec![world.spawn((Spacer,))];
        row_names.extend(row_titles);
        let row_names = system_vertical_group(&mut world, row_names, bounds);
        let mut v_groups = vec![];

        v_groups.push(row_names);

        for col in more_column {
            let v = system_vertical_group(&mut world, col, bounds);
            v_groups.push(v);
        }

        world.spawn((
            Group { children: v_groups, orentation: Orientation::Horizontal },
            Bounds { shape: bounds.clone(), update: true },
            WindowRect,
            NeedsRefresh,
        ));

        Self {
            world,
            scrolling: false,
            cursor: app.mouse.position(),
            screen,

            current_cell: None,
            rows: row_count,
            columns: col_count,

            bounds: bounds.clone(),

            //renderer stuff
            texture,
            draw,
            renderer,
            texture_capturer,
            texture_reshaper,
            dynamic_image,
        }
    }

    pub fn event_handler(&mut self, app: &App, event: &WindowEvent) -> () {
        match *event {
            MouseWheel(delta, phase) => {
                let change = match delta {
                    MouseScrollDelta::LineDelta(x, y) => vec2(x as f32, y as f32),
                    MouseScrollDelta::PixelDelta(position) =>
                        vec2(position.x as f32, position.y as f32),
                };
                system_scroll_update(&mut self.world, change);
            }
            MousePressed(mouse) => {
                system_button_pressed(&mut self.world, mouse);
            }
            MouseReleased(_) => (),
            Moved(_) => (),
            KeyPressed(_) => (),
            KeyReleased(_) => (),
            ReceivedCharacter(_) => (),
            MouseMoved(_) => (),
            MouseEntered => (),
            MouseExited => (),
            Resized(_) => system_resize_window(&mut self.world, app),
            HoveredFile(_) => (),
            DroppedFile(_) => (),
            HoveredFileCancelled => (),
            Touch(_) => (),
            TouchPressure(_) => (),
            Focused => (),
            Unfocused => (),
            Closed => (),
        }
    }

    pub fn draw_function(&self, draw: &Draw) {
        let font = text::font::from_file(Path::new("assets/fonts/Inconsolata-Regular.ttf")).unwrap();

        system_draw_connecting_lines(&self.world, &draw);

        system_draw_basic(&self.world, &draw);
        system_draw_titles(&self.world, &draw, &font);
        system_draw_value(&self.world, &draw, &font);

        // let draw = &app.draw();
    }

    pub fn low_rez_render(&mut self, app: &App) {
        let draw = &self.draw;
        draw.reset();

        draw.background().color(BLACK);

        self.draw_function(draw);

        let window = app.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor { label: Some("texture renderer") };
        let mut encoder = device.create_command_encoder(&ce_desc);
        self.renderer.render_to_texture(device, &mut encoder, draw, &self.texture);

        window.queue().submit(Some(encoder.finish()));
    }

    pub fn update(&mut self, app: &App) -> () {
        self.cursor = system_snap_mouse(&self.world, app.mouse.position());
        // self.grid.update(app);
        // let mouse = ;

        system_check_focus(&mut self.world, app.mouse.position());
        system_update_flexbox(&mut self.world);
        self.low_rez_render(app);
    }

    pub fn draw(&self, app: &App, frame: &Frame) {
        let window = app.main_window();
        let screen = window.rect();

        let draw = &app.draw();

        self.draw_function(draw);

        // draw.texture(&self.texture).wh(screen.wh()).xy(screen.xy());
        draw.to_frame(app, frame).unwrap();
    }
    pub fn get_serial_output(&mut self, app: &App) -> String { system_print_value(&mut self.world) }
    pub fn get_cell_values(&self) -> Vec<(f32, usize, usize)> { system_get_cell_values(&self.world) }
}

//componants-------------------------------------
#[derive(Debug, Clone, Copy)]

struct Bounds {
    shape:  Rect,
    update: bool,
}
struct NeedsRefresh;
enum Orientation {
    Horizontal,
    Vertical,
}
struct Title {
    text:        String,
    orientation: Orientation,
    fill:        nannou::color::rgb::Rgb,
}

pub struct Fill(nannou::color::rgb::Rgb);

type Cell = Box<dyn CellType + Send + Sync>;

pub struct SerialUpdate(bool);

pub trait CellType {
    fn visible(&self) -> bool;
    fn as_string(&self) -> String;
    fn get_value(&self) -> f32;
    fn scoll_update(&mut self, change: Vec2) {}
    fn bool_ascii(&self) -> char { '0' }
    fn clicked_left(&mut self) {}
    fn clicked_right(&mut self) {}
}
#[derive(Debug, Clone, Copy)]

pub struct FloatCell {
    val: f32,
}
pub struct Spacer;
impl CellType for FloatCell {
    fn visible(&self) -> bool { self.val != 0.0 }
    fn as_string(&self) -> String { format!("{:.2}", self.val) }
    fn get_value(&self) -> f32 { self.val }
    fn scoll_update(&mut self, change: Vec2) { self.val += change.y }
    fn clicked_left(&mut self) { self.val = 0.0 }
}
#[derive(Debug, Clone, Copy)]

pub struct BoolCell {
    val: bool,
}
impl CellType for BoolCell {
    fn visible(&self) -> bool { self.val == true }
    fn as_string(&self) -> String { format!("{:}", self.val) }
    fn get_value(&self) -> f32 {
        if self.val {
            1.0
        } else {
            0.0
        }
    }
    fn clicked_left(&mut self) { self.val = !self.val }

    fn bool_ascii(&self) -> char {
        if self.val {
            '1'
        } else {
            '0'
        }
    }
}

// pub struct ButtonPress(Box<dyn Fn(&mut Value) + Send + Sync>);

struct Stroke {
    weight: f32,
    colour: nannou::color::rgb::Rgb,
}
struct Focus(bool);

struct Group {
    children:   Vec<Entity>,
    orentation: Orientation,
}

impl Group {
    fn AddChild(&mut self, entity: Entity) { self.children.push(entity); }
    fn AddChildren(&mut self, entity: &Vec<Entity>) { self.children.extend(entity); }
}

struct OnScroll(Box<dyn Fn(f32) -> f32 + Send + Sync>);

struct Index {
    row:    usize,
    column: usize,
}
struct Headings {
    row:    Entity,
    column: Entity,
}

struct WindowRect;

pub fn system_resize_window(world: &mut World, app: &App) {
    let screen = app.window_rect();
    let rect = screen.pad(100.0);

    let mut id_group = vec![];
    for (id, (window, mut bounds)) in &mut world.query::<(&WindowRect, &mut Bounds)>() {
        bounds.shape = rect;
        bounds.update = true;
        id_group.push(id);
    }
    for id in id_group {
        world.insert_one(id, NeedsRefresh);
    }
}
pub fn system_vertical_group(world: &mut World, entitys: Vec<Entity>, rect: &Rect) -> Entity {
    world.spawn((
        Group { children: entitys, orentation: Orientation::Vertical },
        Bounds { shape: rect.clone(), update: true },
        // NeedsRefresh,
    ))
}
pub fn system_horizontal_group(world: &mut World, entitys: Vec<Entity>, rect: &Rect) -> Entity {
    world.spawn((
        Group { children: entitys, orentation: Orientation::Horizontal },
        Bounds { shape: rect.clone(), update: true },
        NeedsRefresh,
    ))
}
//systems -------------------------------------
pub fn system_snap_mouse(world: &World, mouse_position: Vec2) -> Vec2 {
    let mut pos = vec2(0.0, 0.0);

    for (id, (bounds, focus)) in &mut world.query::<(&Bounds, &mut Focus)>() {
        if bounds.shape.contains(mouse_position) {
            pos = bounds.shape.xy()
        };
    }
    pos
}
pub fn system_draw_titles(world: &World, draw: &Draw, font: &text::Font) {
    for (id, (title, bounds)) in &mut world.query::<(&Title, &Bounds)>() {
        let text = draw
            .text(&title.text)
            .xy(bounds.shape.xy())
            .font(font.clone())
            .color(title.fill)
            .no_line_wrap()
            .font_size(12)
            .wh(bounds.shape.wh());
        match title.orientation {
            Orientation::Horizontal => text.rotate(TAU / 4.0).left_justify(),
            Orientation::Vertical => text.right_justify(),
        };
    }
}
pub fn system_scroll_update(world: &mut World, change: Vec2) {
    for (id, (value, focus, update)) in &mut world.query::<(&mut Cell, &Focus, &mut SerialUpdate)>()
    {
        if focus.0 {
            value.scoll_update(change);
            update.0 = true;
        }
    }
}

pub fn system_button_pressed(world: &mut World, mouse: MouseButton) {
    for (id, (cell, focus, update)) in &mut world.query::<(&mut Cell, &Focus, &mut SerialUpdate)>() {
        if focus.0 {
            match mouse {
                MouseButton::Left => cell.clicked_left(),
                MouseButton::Right => todo!(),
                MouseButton::Middle => todo!(),
                MouseButton::Other(_) => todo!(),
            }
            update.0 = true;
        }
    }
}

pub fn system_draw_connecting_lines(world: &World, draw: &Draw) {
    for (id, (value, focus, headings, bounds)) in
        &mut world.query::<(&mut Cell, &Focus, &Headings, &Bounds)>()
    {
        let (mut x, mut y) = (
            world.query_one::<(&Bounds)>(headings.row).unwrap(),
            world.query_one::<(&Bounds)>(headings.column).unwrap(),
        );
        let (row_bounds, col_bounds) = (x.get().unwrap(), y.get().unwrap());

        if value.visible() {
            draw.line()
                .start(bounds.shape.xy())
                .end(row_bounds.shape.xy())
                .color(GRAY)
                .weight(line_weight);
            draw.line()
                .start(bounds.shape.xy())
                .end(col_bounds.shape.xy())
                .color(GRAY)
                .weight(line_weight);
        }
    }

    for (id, (value, focus, headings, bounds)) in
        &mut world.query::<(&mut Cell, &Focus, &Headings, &Bounds)>()
    {
        let (mut x, mut y) = (
            world.query_one::<(&Bounds)>(headings.row).unwrap(),
            world.query_one::<(&Bounds)>(headings.column).unwrap(),
        );
        let (row_bounds, col_bounds) = (x.get().unwrap(), y.get().unwrap());

        if focus.0 {
            draw.line()
                .start(bounds.shape.xy())
                .end(row_bounds.shape.xy())
                .color(PINK)
                .weight(line_weight);
            draw.line()
                .start(bounds.shape.xy())
                .end(col_bounds.shape.xy())
                .color(PINK)
                .weight(line_weight);
        }
    }
}
pub fn system_print_value(world: &World) -> String {
    let mut serial_output = String::new();
    for (id, (cell, index, update)) in &mut world.query::<(&Cell, &Index, &mut SerialUpdate)>() {
        if update.0 {
            serial_output.push_str(&format!(
                "{:02}:{:02}:{}",
                index.row,
                index.column,
                cell.bool_ascii()
            ));
            serial_output.push_str("\n");

            update.0 = false;
        }
    }

    serial_output
}
pub fn system_draw_value(world: &World, draw: &Draw, font: &text::Font) {
    for (id, (cell, bounds, focus)) in &mut world.query::<(&Cell, &Bounds, &Focus)>() {
        let color = if focus.0 { PINK } else { GRAY };

        if cell.visible() || focus.0 {
            draw.ellipse()
                .xy(bounds.shape.xy())
                .radius(bounds.shape.w_h().0 / 2.0)
                .color(BLACK)
                .stroke_color(color)
                .stroke_weight(line_weight);
            draw.text(&cell.as_string())
                .font(font.clone())
                .xy(bounds.shape.xy())
                .color(color)
                .font_size(12)
                .wh(bounds.shape.wh());
        }
    }
}
pub fn system_draw_basic(world: &World, draw: &Draw) {
    for (id, (stroke, bounds)) in &mut world.query::<(&Stroke, &Bounds)>() {
        draw.rect()
            .xy(bounds.shape.xy())
            .wh(bounds.shape.wh())
            .no_fill()
            .stroke_color(stroke.colour)
            .stroke_weight(stroke.weight);
    }
    for (id, (fill, bounds)) in &mut world.query::<(&Fill, &Bounds)>() {
        draw.rect().xy(bounds.shape.xy()).wh(bounds.shape.wh()).color(fill.0);
    }
}

pub fn system_check_focus(world: &mut World, mouse: Vec2) {
    for (id, (bounds, focus)) in &mut world.query::<(&Bounds, &mut Focus)>() {
        let intersect = bounds.shape.contains(mouse);
        focus.0 = intersect;
    }
}

pub fn system_get_cell_values(world: &World) -> Vec<(f32, usize, usize)> {
    let mut v: Vec<(f32, usize, usize)> = vec![];
    for (id, (value, index)) in &mut world.query::<(&Cell, &Index)>() {
        v.push((value.get_value(), index.row, index.column));
    }
    v
}
pub fn system_update_flexbox(world: &mut World) {
    let mut refresh_loop_counter = 0;
    loop {
        let mut refresh_completed = vec![];
        let mut refresh_needed = vec![];
        //borrow the first entity to need a refresh, only look at one at a time
        if refresh_loop_counter > 1000 {
            panic!("flexbox got caught in a feeback loop");
        } else if let Some(first_entity) =
            world.query::<(&Bounds, &Group, &NeedsRefresh)>().iter().next()
        {
            let (id, (bounds, flexbox, _)) = first_entity;
            refresh_completed.push(id);

            let sub_devisions = match flexbox.orentation {
                Orientation::Horizontal =>
                    bounds.shape.divide_columns(flexbox.children.len() as u32),
                Orientation::Vertical => bounds.shape.divide_rows(flexbox.children.len() as u32),
            };

            for (child_id, new_bounds) in flexbox.children.iter().zip(sub_devisions) {
                refresh_needed.push(*child_id);
                // *world.insert(*child, ());
                // Bounds { shape: todo!(), update: todo!()
                let mut child_bounds = world.query_one::<(&mut Bounds)>(*child_id).unwrap();

                if let Some(child_bounds) = child_bounds.get() {
                    child_bounds.shape = new_bounds;
                }
            }
        } else {
            return;
        }

        for id in refresh_completed {
            world.remove_one::<NeedsRefresh>(id);
        }

        for id in refresh_needed {
            world.insert_one(id, NeedsRefresh);
        }

        refresh_loop_counter += 1;
    }
}
