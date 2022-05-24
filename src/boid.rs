use crate::quadtree::Rectangle;
use nannou::geom::Vec2;
use std::ops::Add;

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
    pub radius: f32,
    pub index: i32,
}

impl Boid {
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration = self.acceleration.add(force);
    }

    pub fn update(&mut self, bounds: &Rectangle) {
        self.velocity = self.velocity.add(self.acceleration);
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.position = self.position.add(self.velocity);
        self.acceleration = Vec2::new(0.0, 0.0);
        if self.position.x + self.radius < -(bounds.width / 2.0) {
            self.position.x = bounds.width / 2.0 + self.radius
        } else if self.position.x - self.radius > (bounds.width / 2.0) {
            self.position.x = -bounds.width / 2.0 - self.radius
        }
        if self.position.y + self.radius < -(bounds.height / 2.0) {
            self.position.y = bounds.height / 2.0 + self.radius
        } else if self.position.y - self.radius > (bounds.height / 2.0) {
            self.position.y = -bounds.height / 2.0 - self.radius
        }
    }
}
