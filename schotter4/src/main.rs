use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
    x_velocity: f32,
    y_velocity: f32,
    rot_velocity: f32,
    cycles: u32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        Stone {
            x,
            y,
            x_offset: 0.0,
            y_offset: 0.0,
            rotation: 0.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            rot_velocity: 0.0,
            cycles: 0,
        }
    }
}
struct Model {
    random_seed: u64,
    disp_adj: f32,
    rot_adj: f32,
    gravel: Vec<Stone>,
    main_window: WindowId,
    ui: Egui,
    motion: f32,
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window_id = ui(&app);
    let ui_window_ref = app.window(ui_window_id).unwrap();
    let ui = Egui::from_window(&ui_window_ref);
    let random_seed = random_range(0, 1_000_000);
    let disp_adj = 1.0;
    let rot_adj = 1.0;

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            gravel.push(Stone::new(x as f32, y as f32));
        }
    }

    let motion = 0.5;
    Model {
        random_seed,
        disp_adj,
        rot_adj,
        gravel,
        main_window,
        ui,
        motion,
    }
}

fn ui(app: &App) -> WindowId {
    app.new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(280, 130)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap()
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap()
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event)
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Schotter Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
            ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
            ui.add(egui::Slider::new(&mut model.motion, 0.0..=1.0).text("Motion"));
        });
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    for stone in &mut model.gravel {
        if stone.cycles == 0 {
            if random_f32() > model.motion {
                stone.x_velocity = 0.0;
                stone.y_velocity = 0.0;
                stone.rot_velocity = 0.0;
                stone.cycles = random_range(50, 300);
            } else {
                let factor = stone.y as f32 / ROWS as f32;
                let disp_factor = factor * model.disp_adj;
                let rot_factor = factor * model.rot_adj;
                let new_x = disp_factor * random_range(-0.5, 0.5);
                let new_y = disp_factor * random_range(-0.5, 0.5);
                let new_rot = rot_factor * random_range(-PI / 4.0, PI / 4.0);
                let new_cycles = random_range(50, 300);
                stone.x_velocity = (new_x - stone.x_offset) / new_cycles as f32;
                stone.y_velocity = (new_y - stone.y_offset) / new_cycles as f32;
                stone.rot_velocity = (new_rot - stone.rotation) / new_cycles as f32;
                stone.cycles = new_cycles;
            }
        } else {
            stone.x_offset += stone.x_velocity;
            stone.y_offset += stone.y_velocity;
            stone.rotation += stone.rot_velocity;
            stone.cycles -= 1;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SNOW);

    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    for stone in &model.gravel {
        gdraw
            .x_y(stone.x, stone.y)
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.rotation);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => model.random_seed = random_range(0, 1_000_000),
        Key::S => app
            .window(model.main_window)
            .unwrap()
            .capture_frame(app.exe_name().unwrap() + ".png"),
        Key::Up | Key::K => model.disp_adj += 0.1,
        Key::Down | Key::J => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right | Key::L => model.rot_adj += 0.1,
        Key::Left | Key::H => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }
}
