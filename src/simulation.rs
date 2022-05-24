use crate::boid::*;
use crate::quadtree::*;
use nannou::{geom::*, rand};
use std::ops::{Add, Sub};

const BOID_BOUNDS_SIZE: f32 = 30.0;

pub struct Simulation {
    boid_count: i32,
    pub boids: Vec<Boid>,
    bounds: Rectangle,
}

impl Simulation {
    pub fn new(boid_count: i32, bounds: Rectangle) -> Self {
        let mut sim = Simulation {
            boid_count,
            boids: Vec::new(),
            bounds,
        };

        let half_width = sim.bounds.width / 2.0;
        let half_height = sim.bounds.height / 2.0;

        let rng = rand::thread_rng();
        for i in 1..boid_count {
            let rx = rand::random_range::<f32>(-half_width, half_width);
            let ry = rand::random_range::<f32>(-half_height, half_height);
            let pt = Vec2::new(rx, ry);
            let vx = rand::random_range::<f32>(-2.0, 2.0);
            let vy = rand::random_range::<f32>(-2.0, 2.0);
            let v = Vec2::new(vx, vy);
            sim.boids.push(Boid {
                position: pt,
                velocity: v,
                radius: 3.0,
                acceleration: vec2(0.0, 0.0),
                max_speed: 5.0,
                max_force: 0.5,
                index: i,
            });
        }
        return sim;
    }

    fn steer(&mut self, point: Vec2) {
        for boid in &mut self.boids {
            let mut steer = boid.velocity.sub(boid.position.sub(point));
            steer = steer.clamp_length_max(boid.max_force);
            boid.apply_force(steer);
        }
    }

    pub fn update(&mut self) {
        self.steer(vec2(0.0, 0.0));
        self.avoid();
        for boid in &mut self.boids {
            boid.update(&self.bounds)
        }
    }

    pub fn avoid(&mut self) {
        let mut quadtree = QuadTree::new(self.bounds);
        for (i, boid) in self.boids.iter().enumerate() {
            quadtree.insert(boid.position, i);
        }

        let forces: &Vec<Vec2> = &self
            .boids
            .iter()
            .map(|boid| {
                let mut found: Vec<(Vec2, usize)> = Vec::new();
                found = quadtree.query(Self::get_vehicle_rect(&boid), found);

                let mut min_dist = 10000000.0;
                let mut min_position = vec2(0.0, 0.0);
                for (_, i) in found {
                    let other = &self.boids[i];
                    let d = boid.position.distance(other.position);
                    if min_dist > d && d > 0.00001 && d < boid.radius * 10.0 {
                        min_dist = d;
                        min_position = other.position;
                    }
                }
                let mut avoid = Vec2::new(0.0, 0.0);
                if min_dist < 10000000.0 {
                    avoid = boid.velocity.add(boid.position.sub(min_position));
                    avoid = 5.0 * avoid.clamp_length_max(boid.max_force);
                }
                avoid
            })
            .collect();

        let mut count = 0;
        for boid in &mut (self).boids {
            boid.apply_force(forces[count]);
            count += 1;
        }
    }

    fn get_vehicle_rect(boid: &Boid) -> Rectangle {
        Rectangle::new(
            boid.position.x - BOID_BOUNDS_SIZE / 2.0,
            boid.position.y - BOID_BOUNDS_SIZE / 2.0,
            BOID_BOUNDS_SIZE,
            BOID_BOUNDS_SIZE,
        )
    }
}
