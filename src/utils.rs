pub const PI: f64 = std::f64::consts::PI;
pub const INF: f64 = std::f64::INFINITY;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

#[allow(dead_code)]
fn rad_to_deg(radians: f64) -> f64 {
    return radians * 180.0 / PI;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
