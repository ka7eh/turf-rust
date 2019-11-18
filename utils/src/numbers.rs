pub fn precision(n: f64, p: i8) -> f64 {
    let p = p as f64;
    let digits = 10_f64.powf(p);
    (n * digits).round() / digits
}