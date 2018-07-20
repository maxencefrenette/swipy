pub fn mean(sample: &Vec<f32>) -> f32 {
    sample.iter().sum::<f32>() / (sample.len() as f32)
}

pub fn standard_dev(sample: &Vec<f32>, x_0: f32) -> f32 {
    let n = sample.len() as f32;
    let variance = mean(&sample.iter().map(|x| square(x - x_0)).collect()) / n;
    let sd = (variance / n - 1.).sqrt();

    sd
}

fn square(x: f32) -> f32 {
    x * x
}
