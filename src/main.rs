mod vec3;
use vec3::*;
mod ray;
use ray::*;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * ray.dir.dot(ray.orig - *center);
    let c = (ray.orig - *center).dot(ray.orig - *center) - (radius * radius);
    let delta = b * b - 4.0 * a * c;
    delta > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0,0.0,-3.0), 1.0, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_dir = unit_vec3(ray.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_w = 384;
    let img_h = (img_w as f64 / aspect_ratio) as u64;
    eprintln!("Starting render");
    println!("P3");
    println!("{}, {}", img_w, img_h);
    println!("255");

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {:>3}", j);
        for i in 0..img_w {
            let u = i as f64 / (img_w - 1) as f64;
            let v = j as f64 / (img_h - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            ray_color(&r).write();
        }
    }
    eprintln!("");
    eprintln!("Done");
}
