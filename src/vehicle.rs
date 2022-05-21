use nannou::prelude::*;
use std::ops::{Add, Sub};

use crate::quadtree::HasLocation;

pub struct Vehicle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
    pub radius: f32,
    pub index: i32,
}

impl HasLocation for Vehicle {
    fn get_location(&self) -> Vec2 {
        self.position
    }
}

impl Vehicle {
    pub fn steer(&mut self, point: Vec2) {
        let mut steer = self.velocity.sub(self.position.sub(point));
        steer = steer.clamp_length_max(self.max_force);
        &self.apply_force(steer);
    }

    pub fn avoid(&mut self, others: &Vec<Vec2>) {
        let mut min_dist = 10000000.0;
        let mut min_position = vec2(0.0, 0.0);
        for p in others {
            let d = self.position.distance(*p);
            if min_dist > d && d > 0.00001 && d < self.radius * 10.0 {
                min_dist = d;
                min_position = *p;
            }
        }

        if min_dist < 10000000.0 {
            let mut avoid = self.velocity.add(self.position.sub(min_position));
            avoid = 3.0 * avoid.clamp_length_max(self.max_force);
            &self.apply_force(avoid);
        }
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
