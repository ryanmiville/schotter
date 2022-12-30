use nannou::prelude::*;

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

#[derive(Debug, Clone)]
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

    fn stop(&mut self) {
        self.x_velocity = 0.0;
        self.y_velocity = 0.0;
        self.rot_velocity = 0.0;
        self.cycles = random_range(50, 300);
    }

    fn reset(&mut self, adj: &Adjustment) {
        let factor = self.y as f32 / ROWS as f32;
        let disp_factor = factor * adj.x_y;
        let rot_factor = factor * adj.rot;
        let new_x = disp_factor * random_range(-5.5, 5.5);
        let new_y = disp_factor * random_range(-5.5, 5.5);
        let new_rot = rot_factor * random_range(-PI / 4.0, PI / 4.0);
        let new_cycles = random_range(50, 300);
        self.x_velocity = (new_x - self.x_offset) / new_cycles as f32;
        self.y_velocity = (new_y - self.y_offset) / new_cycles as f32;
        self.rot_velocity = (new_rot - self.rotation) / new_cycles as f32;
        self.cycles = new_cycles;
    }

    fn update(&mut self) {
        self.x_offset += self.x_velocity;
        self.y_offset += self.y_velocity;
        self.rotation += self.rot_velocity;
        self.cycles -= 1;
    }
}

#[derive(Debug, Clone, Copy)]
struct Adjustment {
    x_y: f32,
    rot: f32,
}

#[derive(Debug, Clone)]
struct Model {
    random_seed: u64,
    adj: Adjustment,
    gravel: Vec<Stone>,
    main_window: WindowId,
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

    let random_seed = random_range(0, 1_000_000);
    let x_y = 1.0;
    let rot = 1.0;
    let adj = Adjustment { x_y, rot };

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            gravel.push(Stone::new(x as f32, y as f32));
        }
    }

    let motion = 0.5;
    Model {
        random_seed,
        adj,
        gravel,
        main_window,
        motion,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for stone in &mut model.gravel {
        let should_stop = random_f32() > model.motion;
        match (stone.cycles, should_stop) {
            (0, true) => stone.stop(),
            (0, false) => stone.reset(&model.adj),
            _ => stone.update(),
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
        Key::Up | Key::K => model.adj.x_y += 0.1,
        Key::Down | Key::J => {
            if model.adj.x_y > 0.0 {
                model.adj.x_y -= 0.1;
            }
        }
        Key::Right | Key::L => model.adj.rot += 0.1,
        Key::Left | Key::H => {
            if model.adj.rot > 0.0 {
                model.adj.rot -= 0.1;
            }
        }
        _ => {}
    }
}
