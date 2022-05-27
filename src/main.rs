mod boid;
mod quadtree;
mod simulation;
use nannou::prelude::*;
use quadtree::Rectangle;
use simulation::Simulation;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mouse_position: Vec2,
    simulation: Simulation,
    counter: u32,
}

impl Model {
    pub fn update(&mut self, app: &App) {
        self.mouse_position = app.mouse.position();
        self.simulation.update(self.mouse_position);
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .fullscreen()
        .event(event)
        .view(view)
        .power_preference(wgpu::PowerPreference::HighPerformance)
        .build()
        .unwrap();
    let w_rect = app.window_rect();
    let bounds = Rectangle {
        x: -w_rect.w() / 2.0,
        y: -w_rect.h() / 2.0,
        width: w_rect.w(),
        height: w_rect.h(),
    };
    dbg!(bounds);
    let model = Model {
        simulation: Simulation::new(50_000, bounds),
        mouse_position: Vec2::new(0.0, 0.0),
        counter: 0,
    };
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.counter += 1;
    if _model.counter > 60 {
        _model.counter = 0;
    }
    _model.update(_app);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let w_rect = app.window_rect();
    draw.background().color(BLACK);
    _model.simulation.draw(&draw);
    if _model.counter == 0 {
        dbg!(app.fps());
    }
    draw.to_frame(app, &frame).unwrap();
}

fn handle_key_press(key: nannou::event::Key, simulation: &mut Simulation) {
    use nannou::event::Key::*;
    match key {
        W => simulation.boid_seperation_factor += 0.1,
        Q => simulation.boid_seperation_factor -= 0.1,
        S => simulation.boid_cohesion_factor += 0.1,
        A => simulation.boid_cohesion_factor -= 0.1,
        X => simulation.boid_alignment_factor += 0.05,
        Z => simulation.boid_alignment_factor -= 0.05,
        _ => {}
    }
    dbg!(simulation.boid_seperation_factor);
    dbg!(simulation.boid_cohesion_factor);
    dbg!(simulation.boid_alignment_factor);
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        // Keyboard events
        KeyPressed(_key) => handle_key_press(_key, &mut _model.simulation),
        KeyReleased(_key) => {}
        _ => {}
    }
}
