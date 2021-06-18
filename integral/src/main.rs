fn main() {
    let equation = |x: f64| -> f64 { 3.0 * x.powi(2) + 2.0 };
    let value = quadrature(0.01, equation);

    println!("{:?}", value);
}

fn quadrature(x_delta: f64, equation: fn(f64) -> f64) -> f64 {
    let mut x: f64 = 0.0;
    let mut s: f64 = 0.0;
    while x <= 1.0 {
        s += equation(x) * x_delta;
        x += x_delta;
    }
    s
}
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(true, true);
    }
}
