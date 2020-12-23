mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
use hittable::*;
mod sphere;
use sphere::*;
mod hittable_list;
use hittable_list::*;
mod camera;
use camera::*;
mod utils;
use utils::*;

use rand::random;

// extern crate minifb;
// use minifb::{Key, Window, WindowOptions};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut hit_record = HitRecord::new();
    if world.hit(ray, 0.0, INF, &mut hit_record) {
        let norm = hit_record.normal;
        return 0.5 * Color::new(norm.x + 1.0, norm.y + 1.0, norm.z + 1.0);
    }
    let unit_dir = unit_vec3(ray.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn progress(val: u64, max: u64) {
    use std::io::{stderr, Write};
    if val == 0 {
        eprintln!("");
    }

    let val = (val * 50) / max;

    eprint!("\r");
    eprint!("[");
    for _ in 0..val {
        eprint!("#");
    }
    for _ in 0..(50 - val) {
        eprint!(" ");
    }
    eprint!("]");
    stderr().flush().unwrap()
}

fn main() {
    // pixel_render()
    ppm_render()
}

fn ppm_render() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let img_w = 500;
    let img_h = (img_w as f64 / aspect_ratio) as u64;
    let samples_per_pixel = 16;

    eprintln!("Starting render");
    println!("P3");
    println!("{}, {}", img_w, img_h);
    println!("255");

    // World

    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // Camera
    let cam = Camera::new();

    for j in (0..img_h).rev() {
        progress(img_h - j, img_h);
        for i in 0..img_w {
            let mut pixel_color = Color::default();
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (img_w - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (img_h - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            pixel_color.write(samples_per_pixel);
        }
    }
    eprintln!("");
    eprintln!("Done");
}

// fn pixel_render() {
//     let aspect_ratio = 16.0 / 9.0;
//     let img_w = 384;
//     let img_h = (img_w as f64 / aspect_ratio) as u64;

//     let mut buffer: Vec<u32> = vec![0; (img_w * img_h) as usize];

//     let mut window = Window::new(
//         "Test - ESC to exit",
//         WIDTH,
//         HEIGHT,
//         WindowOptions::default(),
//     )
//     .unwrap_or_else(|e| {
//         panic!("{}", e);
//     });

//     // Limit to max ~60 fps update rate
//     window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         for i in buffer.iter_mut() {
//             *i = 0; // write something more funny here!
//         }

//         // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
//         window
//             .update_with_buffer(&buffer, WIDTH, HEIGHT)
//             .unwrap();
//     }
// }
