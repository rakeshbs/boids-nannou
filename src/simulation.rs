use crate::boid::*;
use crate::spatial_hash::*;
use nannou::prelude::*;
use nannou::rand;
use rayon::prelude::*;

pub struct Simulation {
    boid_count: i32,
    mouse_position: Vec2,
    pub boids: Vec<Boid>,
    bounds: Rectangle,
    grid: SpatialHashGrid,
    pub boid_cohesion_factor: f32,
    pub boid_seperation_factor: f32,
    pub boid_alignment_factor: f32,
}

impl Simulation {
    pub fn new(boid_count: i32, bounds: Rectangle) -> Self {
        let half_width = bounds.width / 2.0;
        let half_height = bounds.height / 2.0;

        let mut boids = Vec::with_capacity(boid_count as usize);
        for i in 0..boid_count {
            let rx = rand::random_range::<f32>(-half_width, half_width);
            let ry = rand::random_range::<f32>(-half_height, half_height);
            let pt = Vec2::new(rx, ry);
            let vx = rand::random_range::<f32>(-2.0, 2.0);
            let vy = rand::random_range::<f32>(-2.0, 2.0);
            let v = Vec2::new(vx, vy);
            boids.push(Boid {
                position: pt,
                velocity: v,
                radius: BOID_RADIUS,
                acceleration: vec2(0.0, 0.0),
                max_speed: BOID_MAX_VELOCITY,
                max_force: BOID_MAX_FORCE,
                index: i,
            });
        }

        let grid = SpatialHashGrid::new(bounds, BOID_BOUNDS_SIZE);

        Simulation {
            boid_count,
            boids,
            bounds,
            grid,
            mouse_position: vec2(0.0, 0.0),
            boid_seperation_factor: BOID_SEPERATION_FACTOR,
            boid_cohesion_factor: BOID_COHESION_FACTOR,
            boid_alignment_factor: BOID_ALIGNMENT_FACTOR,
        }
    }

    fn steer(&mut self, point: Vec2) {
        for boid in &mut self.boids {
            let mut steer = boid.velocity - (boid.position - point);
            steer = steer.clamp_length_max(boid.max_force);
            boid.apply_force(steer);
        }
    }

    pub fn navigate(&mut self) {
        self.grid.clear();
        for (i, boid) in self.boids.iter().enumerate() {
            self.grid.insert(boid.position, i);
        }

        let boids = &self.boids;
        let grid = &self.grid;
        let sep_factor = self.boid_seperation_factor;
        let coh_factor = self.boid_cohesion_factor;
        let ali_factor = self.boid_alignment_factor;
        let avoid_radius_sq = BOID_AVOID_RADIUS * BOID_AVOID_RADIUS;
        let follow_radius_sq = BOID_FOLLOW_RADIUS * BOID_FOLLOW_RADIUS;

        let forces: Vec<Vec2> = (0..boids.len())
            .into_par_iter()
            .map(|i| {
                let boid = &boids[i];
                let mut separation = Vec2::new(0.0, 0.0);
                let mut count_separation = 0;
                let mut cohesion = Vec2::new(0.0, 0.0);
                let mut count_cohesion = 0;
                let mut alignment = Vec2::new(0.0, 0.0);
                let mut count_alignment = 0;

                grid.query(boid.get_perception_rect(), |other_idx| {
                    if other_idx == i {
                        return;
                    }
                    let other = &boids[other_idx];
                    let diff = boid.position - other.position;
                    let dist_sq = diff.length_squared();
                    if dist_sq <= avoid_radius_sq {
                        separation += diff;
                        count_separation += 1;
                    }
                    if dist_sq <= follow_radius_sq {
                        cohesion += other.position;
                        alignment += other.velocity;
                        count_cohesion += 1;
                        count_alignment += 1;
                    }
                });

                let mut net = vec2(0.0, 0.0);
                if count_separation > 0 {
                    separation = separation / count_separation as f32;
                    separation = separation.normalize();
                    net += separation * sep_factor;
                }
                if count_cohesion > 0 {
                    cohesion = cohesion / count_cohesion as f32;
                    cohesion = (cohesion - boid.position).normalize();
                    alignment = alignment / count_alignment as f32;
                    alignment *= BOID_MAX_VELOCITY;
                    net += (alignment - boid.velocity) * ali_factor;
                    net += cohesion * coh_factor;
                }
                net
            })
            .collect();

        self.boids.par_iter_mut().enumerate().for_each(|(i, b)| {
            b.apply_force(forces[i]);
            b.update(self.bounds);
        });
    }

    pub fn update(&mut self, mouse_position: Vec2) {
        self.mouse_position = mouse_position;
        self.navigate();
    }

    pub fn draw(&self, draw: &nannou::prelude::Draw) {
        let positions: Vec<Vec3> = self
            .boids
            .iter()
            .map(|b| vec3(b.position.x, b.position.y, 0.0))
            .collect();

        draw.point_mode().mesh().points(positions);
    }
}
