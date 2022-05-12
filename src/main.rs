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
    radius: f32,
}

impl Vehicle {
    pub fn update(&mut self, bounds: Rect) {
        self.position = self.position.add(self.velocity);
        if self.position.x + self.radius < -(bounds.w() / 2.0) {
            self.position.x = bounds.w() / 2.0 + self.radius
        } else if self.position.x - self.radius > (bounds.w() / 2.0) {
            self.position.x = -bounds.w() / 2.0 - self.radius
        }
        if self.position.y + self.radius < -(bounds.h() / 2.0) {
            self.position.y = bounds.h() / 2.0 + self.radius
        } else if self.position.y - self.radius > (bounds.h() / 2.0) {
            self.position.y = -bounds.h() / 2.0 - self.radius
        }
    }
}

struct Model {
    window_bounds: Rect,
    vehicles: Vec<Vehicle>,
}

impl Model {
    pub fn update(&mut self) {
        for v in &mut self.vehicles {
            v.update(self.window_bounds);
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let w_rect = app.window_rect();
    let half_width = w_rect.w() / 2.0;
    let half_height = w_rect.h() / 2.0;
    dbg!("Window Size {}{}", half_width, half_height);
    let mut model = Model {
        window_bounds: w_rect,
        vehicles: Vec::new(),
    };
    for i in (1..100) {
        let rx = rand::random_range::<f32>(-half_width, half_width);
        let ry = rand::random_range::<f32>(-half_height, half_height);
        let pt = Vec2::new(rx, ry);
        let vx = rand::random_range::<f32>(-2.0, 2.0);
        let vy = rand::random_range::<f32>(-2.0, 2.0);
        let v = Vec2::new(vx, vy);
        model.vehicles.push(Vehicle {
            position: pt,
            velocity: v,
            radius: 5.0,
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
            .w_h(v.radius, v.radius)
            .color(STEELBLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}
