mod vec3;
use vec3::*;

fn main() {
    let img_w = 256;
    let img_h = 256;
    eprintln!("Starting render");
    println!("P3");
    println!("{}, {}", img_w, img_h);
    println!("255");
    for j in (0..img_h).rev() {
        eprint!("\rScanlines remaining: {:>3}", j);
        for i in 0..img_w {
            let color = Color {
                r: i as f64 / (img_w as f64 - 1.0),
                g: 0.25,
                b: j as f64 / (img_h as f64 - 1.0),
            };
            color.write();
        }
    }
    eprintln!("");
    eprintln!("Done");
}
