use crate::quadtree::*;
use nannou::geom::Vec2;
use std::ops::Add;

pub const BOID_BOUNDS_SIZE: f32 = 10.0;
pub const BOID_RADIUS: f32 = 3.0;
pub const BOID_MAX_VELOCITY: f32 = 3.0;
pub const BOID_MAX_FORCE: f32 = 1.0;
pub const BOID_AVOID_RADIUS: f32 = 3.0;
pub const BOID_FOLLOW_RADIUS: f32 = 10.0;

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
    pub radius: f32,
    pub index: i32,
}

impl HasLocation for Boid {
    fn get_location(&self) -> Vec2 {
        self.position
    }
}

impl Boid {
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration = self
            .acceleration
            .add(force)
            .clamp_length_max(self.max_force);
    }

    pub fn update(&mut self, bounds: Rectangle) {
        self.velocity = self.velocity.add(self.acceleration);
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.position = self.position.add(self.velocity);
        self.acceleration = Vec2::new(0.0, 0.0);
        self.loop_bounds(bounds);
    }

    pub fn reflect_bounds(&mut self, bounds: Rectangle) {
        if self.position.x + self.radius < bounds.x {
            self.velocity = Vec2::new(-self.velocity.x, self.velocity.y);
        } else if self.position.x - self.radius > bounds.x + bounds.width {
            self.velocity = Vec2::new(-self.velocity.x, self.velocity.y);
        }
        if self.position.y + self.radius < bounds.y {
            self.velocity = Vec2::new(self.velocity.x, -self.velocity.y);
        } else if self.position.y - self.radius > bounds.y + bounds.height {
            self.velocity = Vec2::new(self.velocity.x, -self.velocity.y);
        }
    }

    pub fn loop_bounds(&mut self, bounds: Rectangle) {
        if self.position.x + self.radius < bounds.x {
            self.position.x = bounds.x + bounds.width + self.radius
        } else if self.position.x - self.radius > bounds.x + bounds.width {
            self.position.x = bounds.x - self.radius
        }
        if self.position.y + self.radius < bounds.y {
            self.position.y = bounds.y + bounds.height + self.radius
        } else if self.position.y - self.radius > bounds.y + bounds.height {
            self.position.y = bounds.y - self.radius
        }
    }
}
