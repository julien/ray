mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use hit::{Hit, World};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3};

use rand::Rng;
use std::io::{stderr, Write};
use std::sync::Arc;

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

// run with cargo run > image.ppm
fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 800;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 10;
    const MAX_DEPTH: u64 = 5;

    let r: f64 = (std::f64::consts::PI / 4.0).cos();
    let mut world = World::new();

    let mat_left = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let sphere_left = Sphere::new(Point3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3::new(r, 0.0, -1.0), r, mat_right);

    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    let cam = Camera::new(90.0, ASPECT_RATIO);

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let mut rng = rand::thread_rng();
    // pixels are written out in rows with pixels left to right
    // rows are written from top to bottom
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {:3}", j + 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
