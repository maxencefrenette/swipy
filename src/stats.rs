pub fn mean(sample: &Vec<f64>) -> f64 {
    sample.iter().sum::<f64>() / (sample.len() as f64)
}

pub fn standard_dev(sample: &Vec<f64>, x_0: f64) -> f64 {
    let n = sample.len() as f64;
    (mean(&sample.iter().map(|x| square(x - x_0)).collect()) / (n - 1.)).sqrt()
}

fn square(x: f64) -> f64 {
    x * x
}
