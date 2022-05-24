mod boid;
mod quadtree;
mod simulation;
use nannou::prelude::*;
use quadtree::Rectangle;
use simulation::Simulation;

fn main() {
    nannou::app(model).update(update).run();
}

const VEHICLE_RECT_WIDTH: f32 = 10.0;

struct Model {
    mouse_position: Vec2,
    simulation: Simulation,
}

impl Model {
    pub fn update(&mut self, app: &App) {
        self.mouse_position = app.mouse.position();
        self.simulation.update();
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let w_rect = app.window_rect();
    let bounds = Rectangle {
        x: -w_rect.w() / 2.0,
        y: -w_rect.h() / 2.0,
        width: w_rect.w(),
        height: w_rect.h(),
    };
    let mut model = Model {
        simulation: Simulation::new(100, bounds),
        mouse_position: Vec2::new(0.0, 0.0),
    };
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.update(_app);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for v in &_model.simulation.boids {
        draw.ellipse().radius(4.0).xy(v.position).color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}
