use std::ops::{Add, Sub};
mod quadtree;
mod vehicle;
use crate::quadtree::*;
use rand::Rng;
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

const VEHICLE_RECT_WIDTH: f32 = 10.0;

struct Model {
    window_bounds: Rect,
    mouse_position: Vec2,
    vehicles: Vec<Vehicle>,
}

impl Model {
    pub fn update(&mut self, app: &App) {
        self.mouse_position = app.mouse.position();
        let w_rect = app.window_rect();
        let half_width = w_rect.w() / 2.0;
        let half_height = w_rect.h() / 2.0;
        let mut quadtree: QuadTree<Vehicle> = QuadTree::new(Rectangle::new(
            -half_width,
            -half_height,
            w_rect.w(),
            w_rect.h(),
        ));

        for v in &mut self.vehicles {
            v.steer(self.mouse_position);
            let vehicle_rect = Model::get_vehicle_rect(v);
            let mut found: Vec<&Vehicle> = Vec::new();
            found = quadtree.query(&vehicle_rect, found);

            let others: Vec<Vec2> = found.iter().map(|v| v.position).collect();
            v.avoid(&others);
            v.update(self.window_bounds);
        }
    }

    fn get_vehicle_rect(vehicle: &Vehicle) -> Rectangle {
        Rectangle::new(
            vehicle.position.x - VEHICLE_RECT_WIDTH / 2.0,
            vehicle.position.y - VEHICLE_RECT_WIDTH / 2.0,
            VEHICLE_RECT_WIDTH,
            VEHICLE_RECT_WIDTH,
        )
    }
}

fn model(app: &App) -> Model {
    //app.set_loop_mode(LoopMode::NTimes {
    //number_of_updates: 1000,
    //});
    let _window = app.new_window().view(view).build().unwrap();
    let w_rect = app.window_rect();
    let half_width = w_rect.w() / 2.0;
    let half_height = w_rect.h() / 2.0;
    dbg!("Window Size {}{}", half_width, half_height);
    let mut model = Model {
        window_bounds: w_rect,
        vehicles: Vec::new(),
        mouse_position: Vec2::new(0.0, 0.0),
    };
    let mut rng = rand::thread_rng();
    for i in 1..200 {
        let rx = rand::random_range::<f32>(-half_width, half_width);
        let ry = rand::random_range::<f32>(-half_height, half_height);
        let pt = Vec2::new(rx, ry);
        let vx = rand::random_range::<f32>(-2.0, 2.0);
        let vy = rand::random_range::<f32>(-2.0, 2.0);
        let v = Vec2::new(vx, vy);
        let mut max_speed: f32 = rng.gen_range(0.1..4.0);
        model.vehicles.push(Vehicle {
            position: pt,
            velocity: v,
            radius: 3.0,
            acceleration: Vec2::new(0.0, 0.0),
            max_speed,
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
        w_rect.w(),
        w_rect.h(),
    ));
    let w = 100.0;
    let x = app.mouse.x;
    let y = app.mouse.y;
    let rect = Rectangle {
        x: x - w / 2.0,
        y: y - w / 2.0,
        width: w,
        height: w,
    };

    //let rect2 = Rectangle {
    //x: -100.0,
    //y: -100.0,
    //width: w,
    //height: w,
    //};
    //draw.rect()
    //.x_y(rect.x, rect.y)
    //.width(rect.width)
    //.height(rect.height)
    //.no_fill()
    //.stroke_weight(1.0)
    //.stroke_color(BLUE);

    //if rect2.point_inside_rect(Vec2::new(x, y)) {
    //draw.rect()
    //.x_y(rect2.x + w / 2.0, rect2.y + w / 2.0)
    //.width(rect2.width)
    //.height(rect2.height)
    //.no_fill()
    //.stroke_weight(1.0)
    //.stroke_color(GREEN);
    //} else {
    //draw.rect()
    //.x_y(rect2.x + w / 2.0, rect2.y + w / 2.0)
    //.width(rect2.width)
    //.height(rect2.height)
    //.no_fill()
    //.stroke_weight(1.0)
    //.stroke_color(RED);
    //}

    for v in &_model.vehicles {
        quadtree.insert(&v);
        draw.ellipse().radius(4.0).xy(v.position).color(STEELBLUE);
    }
    quadtree.draw(&draw, &rect);
    let mut found: Vec<&Vehicle> = Vec::new();
    found = quadtree.query(&rect, found);
    for v in found {
        draw.ellipse().radius(4.0).xy(v.position).color(YELLOW);
    }
    draw.rect()
        .x_y(rect.x + w / 2.0, rect.y + w / 2.0)
        .width(rect.width)
        .height(rect.height)
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(RED);

    draw.to_frame(app, &frame).unwrap();
}
