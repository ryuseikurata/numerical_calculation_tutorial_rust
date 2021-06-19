use plotters::prelude::*;
use std::f64;

const OUT_FILE_NAME: &'static str = "images/test.png";

fn main() {
    let drawing_area = BitMapBackend::new(OUT_FILE_NAME, (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(-5.0..5.0, 0.0..0.5)
        .unwrap();
    let mut norm = NormalDistribution::new(0.0, 1.0);

    println!("{:?}", norm.method(&-1.0));
    chart
        .draw_series(LineSeries::new(
            (-100..100).map(|x| (x as f64, (norm.method(&(x as f64))))),
            &BLACK,
        ))
        .unwrap();
}

/// 正規分布
struct NormalDistribution {
    /// 平均
    mu: f64,
    /// 分散
    sigma: f64,
}

impl NormalDistribution {
    fn new(mu: f64, sigma: f64) -> Self {
        Self { mu, sigma }
    }

    fn method(&mut self, x: &f64) -> f64 {
        let numerator = (-(x - self.mu).powi(2) / (2.0_f64 * self.sigma.powi(2))).exp();
        // sqrtは、累乗根を持つようになる
        // PIは、π
        let denominator = (2.0_f64 * f64::consts::PI).sqrt() * self.sigma;
        numerator / denominator
    }

    /// 与えられたx(ベクトル)に対して、正規分布に従った
    fn calc(&mut self, x: &Vec<f64>) -> Vec<f64> {
        let mut y: Vec<f64> = vec![];
        for e in x {
            y.push(self.method(e));
        }
        y
    }

    fn show_info(&mut self) {
        println!("------- Information -------");
        println!("Average: {}, Variance: {}", self.mu, self.sigma);
        println!("---------------------------");
    }
}
