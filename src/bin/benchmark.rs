use std::time::Instant;
use nannou::prelude::*;
use nannou::rand;
use rayon::prelude::*;

use steering::boid::*;
use steering::spatial_hash::{Rectangle, SpatialHashGrid};
use steering::quadtree::QuadTree;

const NUM_BOIDS: usize = 150_000;
const NUM_ITERATIONS: usize = 30;
const WARMUP_ITERATIONS: usize = 5;

fn main() {
    let bounds = Rectangle::new(-960.0, -540.0, 1920.0, 1080.0);

    // Generate random boids
    let mut boids = Vec::with_capacity(NUM_BOIDS);
    for i in 0..NUM_BOIDS as i32 {
        let rx = rand::random_range::<f32>(-960.0, 960.0);
        let ry = rand::random_range::<f32>(-540.0, 540.0);
        let vx = rand::random_range::<f32>(-2.0, 2.0);
        let vy = rand::random_range::<f32>(-2.0, 2.0);
        boids.push(Boid {
            position: Vec2::new(rx, ry),
            velocity: Vec2::new(vx, vy),
            radius: BOID_RADIUS,
            acceleration: vec2(0.0, 0.0),
            max_speed: BOID_MAX_VELOCITY,
            max_force: BOID_MAX_FORCE,
            index: i,
        });
    }

    println!("Benchmarking with {} boids", NUM_BOIDS);
    println!("{} warmup + {} measured iterations each\n", WARMUP_ITERATIONS, NUM_ITERATIONS);

    // --- Spatial Hash ---
    let mut boids_sh = boids.clone();
    let mut grid = SpatialHashGrid::new(bounds, BOID_BOUNDS_SIZE);

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        run_spatial_hash(&mut boids_sh, &mut grid, bounds);
    }

    let start = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        run_spatial_hash(&mut boids_sh, &mut grid, bounds);
    }
    let sh_time = start.elapsed();
    let sh_avg = sh_time / NUM_ITERATIONS as u32;
    let sh_fps = 1.0 / sh_avg.as_secs_f64();

    println!("Spatial Hash:");
    println!("  Total: {:?}", sh_time);
    println!("  Avg per frame: {:?}  ({:.1} fps)", sh_avg, sh_fps);

    // --- QuadTree ---
    let mut boids_qt = boids.clone();

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        run_quadtree(&mut boids_qt, bounds);
    }

    let start = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        run_quadtree(&mut boids_qt, bounds);
    }
    let qt_time = start.elapsed();
    let qt_avg = qt_time / NUM_ITERATIONS as u32;
    let qt_fps = 1.0 / qt_avg.as_secs_f64();

    println!("\nQuadTree:");
    println!("  Total: {:?}", qt_time);
    println!("  Avg per frame: {:?}  ({:.1} fps)", qt_avg, qt_fps);

    let speedup = qt_time.as_secs_f64() / sh_time.as_secs_f64();
    println!("\nSpatial Hash is {:.2}x faster than QuadTree", speedup);
}

fn run_spatial_hash(boids: &mut Vec<Boid>, grid: &mut SpatialHashGrid, bounds: Rectangle) {
    grid.clear();
    for (i, boid) in boids.iter().enumerate() {
        grid.insert(boid.position, i);
    }

    let boids_ref = &*boids;
    let grid_ref = &*grid;
    let sep_factor = BOID_SEPERATION_FACTOR;
    let coh_factor = BOID_COHESION_FACTOR;
    let ali_factor = BOID_ALIGNMENT_FACTOR;
    let avoid_radius_sq = BOID_AVOID_RADIUS * BOID_AVOID_RADIUS;
    let follow_radius_sq = BOID_FOLLOW_RADIUS * BOID_FOLLOW_RADIUS;

    let forces: Vec<Vec2> = (0..boids_ref.len())
        .into_par_iter()
        .map(|i| {
            let boid = &boids_ref[i];
            let mut separation = Vec2::new(0.0, 0.0);
            let mut count_separation = 0;
            let mut cohesion = Vec2::new(0.0, 0.0);
            let mut count_cohesion = 0;
            let mut alignment = Vec2::new(0.0, 0.0);
            let mut count_alignment = 0;

            grid_ref.query(boid.get_perception_rect(), |other_idx| {
                if other_idx == i {
                    return;
                }
                let other = &boids_ref[other_idx];
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

    boids.par_iter_mut().enumerate().for_each(|(i, b)| {
        b.apply_force(forces[i]);
        b.update(bounds);
    });
}

fn run_quadtree(boids: &mut Vec<Boid>, bounds: Rectangle) {
    let mut quadtree: QuadTree<Boid> = QuadTree::new(bounds);
    for boid in boids.iter() {
        quadtree.insert(boid);
    }

    let sep_factor = BOID_SEPERATION_FACTOR;
    let coh_factor = BOID_COHESION_FACTOR;
    let ali_factor = BOID_ALIGNMENT_FACTOR;

    let forces: Vec<Vec2> = boids
        .par_iter()
        .map(|boid| {
            let found = quadtree.query(boid.get_perception_rect());

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
                        seperation += boid.position - other.position;
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
                let seperation = (seperation / count_seperation as f32).normalize();
                net += seperation * sep_factor;
            }
            if count_cohesion > 0 {
                let cohesion = (cohesion / count_cohesion as f32);
                let cohesion = (cohesion - boid.position).normalize();
                let alignment = (alignment / count_alignment as f32).normalize();
                let alignment = alignment * BOID_MAX_VELOCITY;
                net += (alignment - boid.velocity) * ali_factor;
                net += cohesion * coh_factor;
            }
            net
        })
        .collect();

    boids.par_iter_mut().enumerate().for_each(|(i, b)| {
        b.apply_force(forces[i]);
    });

    boids.par_iter_mut().for_each(|boid| boid.update(bounds));
}
