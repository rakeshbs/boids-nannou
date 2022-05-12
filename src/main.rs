use std::ops::Add;

use nannou::{
    lyon::geom::{
        euclid::{Point2D, Vector2D},
        Point,
    },
    prelude::*,
    rand,
};

fn main() {
    nannou::app(model).update(update).run();
}

struct Vehicle {
    position: Vec2,
    velocity: Vec2,
}

impl Vehicle {
    pub fn new(p: Vec2, v: Vec2) -> Self {
        Vehicle {
            position: p,
            velocity: v,
        }
    }

    pub fn update(&mut self) {
        self.position = self.position.add(self.velocity);
    }
}

struct Model {
    vehicles: Vec<Vehicle>,
}

impl Model {
    pub fn update(&mut self) {
        for v in &mut self.vehicles {
            v.update();
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let w = app.window(_window).unwrap();
    let half_width = 100.0;
    let half_height = 100.0;
    dbg!("Window Size {}{}", half_width, half_height);
    let mut model = Model {
        vehicles: Vec::new(),
    };
    for i in (1..10) {
        let rx = rand::random_range::<f32>(-half_width, half_width);
        let ry = rand::random_range::<f32>(-half_height, half_height);
        let pt = Vec2::new(rx, ry);
        let vx = rand::random_range::<f32>(-2.0, 2.0);
        let vy = rand::random_range::<f32>(-2.0, 2.0);
        let v = Vec2::new(vx, vy);
        model.vehicles.push(Vehicle {
            position: pt,
            velocity: v,
        });
    }
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.update();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for v in &_model.vehicles {
        draw.ellipse()
            .xy(v.position)
            .w_h(10.0, 10.0)
            .color(STEELBLUE);
    }
    //draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
