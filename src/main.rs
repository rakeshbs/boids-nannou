use std::ops::{Add, Sub};

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
    acceleration: Vec2,
    max_speed: f32,
    max_force: f32,
    radius: f32,
}

impl Vehicle {
    pub fn steer(&mut self, point: Vec2) {
        let mut steer = self.velocity.sub(self.position.sub(point));
        steer = steer.clamp_length_max(self.max_force);
        &self.apply_force(steer);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration = self.acceleration.add(force);
    }

    pub fn update(&mut self, bounds: Rect) {
        self.velocity = self.velocity.add(self.acceleration);
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.position = self.position.add(self.velocity);
        self.acceleration = Vec2::new(0.0, 0.0);
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
    mouse_position: Vec2,
    vehicles: Vec<Vehicle>,
}

impl Model {
    pub fn update(&mut self, app: &App) {
        self.mouse_position = app.mouse.position();
        for v in &mut self.vehicles {
            v.steer(self.mouse_position);
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
        mouse_position: app.mouse.position(),
    };
    for i in (1..10000) {
        let rx = rand::random_range::<f32>(-half_width, half_width);
        let ry = rand::random_range::<f32>(-half_height, half_height);
        let pt = Vec2::new(rx, ry);
        let vx = rand::random_range::<f32>(-2.0, 2.0);
        let vy = rand::random_range::<f32>(-2.0, 2.0);
        let v = Vec2::new(vx, vy);
        model.vehicles.push(Vehicle {
            position: pt,
            velocity: v,
            radius: 3.0,
            acceleration: Vec2::new(0.0, 0.0),
            max_speed: 10.0,
            max_force: 0.5,
        });
    }
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.update(_app);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let mut draw = app.draw().point_mode();
    draw.background().color(BLACK);
    let points: Vec<Point3> = _model
        .vehicles
        .iter()
        .map(|v| {
            let p = v.position;
            Point3::new(p.x, p.y, 0.0)
        })
        .collect();
    draw.mesh().points(points);
    draw.to_frame(app, &frame).unwrap();
}
