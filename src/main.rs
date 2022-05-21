use std::ops::{Add, Sub};
mod quadtree;
mod vehicle;
use crate::quadtree::*;
use vehicle::Vehicle;

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

struct Model {
    window_bounds: Rect,
    mouse_position: Vec2,
    vehicles: Vec<Vehicle>,
}

impl Model {
    pub fn update(&mut self, app: &App) {
        self.mouse_position = app.mouse.position();
        let others: Vec<Vec2> = self.vehicles.iter().map(|v| v.position).collect();
        for v in &mut self.vehicles {
            v.steer(self.mouse_position);
            v.avoid(&others);
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
    for i in 1..100 {
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
            max_speed: 4.0,
            max_force: 0.5,
            index: i,
        });
    }
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.update(_app);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let w_rect = app.window_rect();
    let half_width = w_rect.w() / 2.0;
    let half_height = w_rect.h() / 2.0;
    let mut quadtree: QuadTree<Vehicle> = QuadTree::new(Rectangle::new(
        -half_width,
        -half_height,
        app.window_rect().w(),
        app.window_rect().h(),
    ));
    for v in &_model.vehicles {
        quadtree.insert(&v);
        draw.ellipse().radius(4.0).xy(v.position).color(STEELBLUE);
    }
    quadtree.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
