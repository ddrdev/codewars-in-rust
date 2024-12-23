pub fn square_area_to_circle(size: f64) -> f64 {
    let r = size.sqrt() / 2f64;
    std::f64::consts::PI * r.powi(2)
}
