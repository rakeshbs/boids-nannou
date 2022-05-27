use crate::boid::*;
use crate::quadtree::*;
use nannou::{geom::*, rand};
use std::ops::Div;
use std::ops::{Add, Sub};

pub struct Simulation {
    boid_count: i32,
    mouse_position: Vec2,
    pub boids: Vec<Boid>,
    bounds: Rectangle,
    pub boid_cohesion_factor: f32,
    pub boid_seperation_factor: f32,
    pub boid_alignment_factor: f32,
}

impl Simulation {
    pub fn new(boid_count: i32, bounds: Rectangle) -> Self {
        let mut sim = Simulation {
            boid_count,
            boids: Vec::new(),
            bounds,
            mouse_position: vec2(0.0, 0.0),
            boid_seperation_factor: BOID_SEPERATION_FACTOR,
            boid_cohesion_factor: BOID_COHESION_FACTOR,
            boid_alignment_factor: BOID_ALIGNMENT_FACTOR,
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
                radius: BOID_RADIUS,
                acceleration: vec2(0.0, 0.0),
                max_speed: BOID_MAX_VELOCITY,
                max_force: BOID_MAX_FORCE,
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

    pub fn navigate(&mut self) {
        let mut quadtree: QuadTree<Boid> = QuadTree::new(self.bounds);
        for boid in self.boids.iter() {
            quadtree.insert(boid);
        }

        let forces: Vec<Vec2> = self
            .boids
            .iter()
            .map(|boid| {
                let found = quadtree.query(Self::get_vehicle_rect(&boid));

                let mut seperation = Vec2::new(0.0, 0.0);
                let mut count_seperation = 0;
                let mut cohesion = Vec2::new(0.0, 0.0);
                let mut count_cohesion = 0;
                let mut alignment = Vec2::new(0.0, 0.0);
                let mut count_alignment = 0;
                for other in &found {
                    if other.index != boid.index {
                        let dist = other.position.distance(boid.position);
                        if dist <= BOID_AVOID_RADIUS {
                            seperation += boid.position.sub(other.position);
                            count_seperation += 1;
                        }
                        if dist <= BOID_FOLLOW_RADIUS {
                            cohesion += other.position;
                            alignment += other.velocity;
                            count_cohesion += 1;
                            count_alignment += 1;
                        }
                    }
                }
                let mut net = vec2(0.0, 0.0);
                if count_seperation > 0 {
                    seperation = seperation.div(count_seperation as f32).normalize();
                    net += seperation * self.boid_seperation_factor;
                }
                if count_cohesion > 0 {
                    cohesion = cohesion.div(count_cohesion as f32);
                    cohesion = cohesion.sub(boid.position).normalize();
                    alignment = alignment.div(count_alignment as f32).normalize();
                    alignment *= BOID_MAX_VELOCITY;
                    net += (alignment - boid.velocity) * self.boid_alignment_factor;
                    net += cohesion * self.boid_cohesion_factor;
                }
                net
            })
            .collect();

        let mut count = 0;
        for boid in &mut (self).boids {
            boid.apply_force(forces[count]);
            count += 1;
        }
    }

    pub fn update(&mut self, mouse_position: Vec2) {
        self.mouse_position = mouse_position;
        //self.steer(self.mouse_position);
        self.navigate();
        for boid in &mut self.boids {
            boid.update(self.bounds)
        }
    }

    pub fn draw(&self, draw: &nannou::prelude::Draw) {
        let positions: Vec<Vec3> = self
            .boids
            .iter()
            .map(|b| vec3(b.position.x, b.position.y, 0.0))
            .collect();

        //positions.iter().for_each(|p| {
        //draw.ellipse()
        //.xyz(*p)
        //.radius(BOID_RADIUS)
        //.color(nannou::color::DARKSLATEBLUE);
        //});
        draw.point_mode().mesh().points(positions);
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
