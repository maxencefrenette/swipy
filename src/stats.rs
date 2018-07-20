pub fn average(sample: &Vec<f64>) -> f64 {
    sample.iter().sum::<f64>() / (sample.len() as f64)
}
