#[derive(Debug, Clone)]
pub struct Config {
    pub outer_pos_bonus: [f32; 16],
    pub inner_pos_bonus: [f32; 16],
}

pub const DEFAULT_CONFIG: Config = Config {
    outer_pos_bonus: [0.; 16],
    inner_pos_bonus: [0.; 16],
};

pub const OPTIMIZED_CONFIG: Config = Config {
    outer_pos_bonus: [
        187.51797, 146.52249, 118.49475, 57.08678, 19.882929, 33.229523, -6.8986917, -130.95418,
        -349.45972, -388.40686, -0.4167655, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
    inner_pos_bonus: [
        162.26865, 136.7995, 109.440506, 94.70556, 73.923134, 41.334373, -11.293593, -140.11522,
        -363.43854, -411.76, -5.027099, 0.0, 0.0, 0.0, 0.0, 0.0,
    ],
};

impl Config {
    pub fn dimensions() -> usize {
        32
    }

    pub fn from_vec(params: Vec<f32>) -> Config {
        Config {
            inner_pos_bonus: slice_to_arr(&params[0..16]),
            outer_pos_bonus: slice_to_arr(&params[16..32]),
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut vec = Vec::<f32>::with_capacity(Config::dimensions());
        vec.append(&mut self.inner_pos_bonus.to_vec());
        vec.append(&mut self.outer_pos_bonus.to_vec());

        vec
    }
}

fn slice_to_arr(slice: &[f32]) -> [f32; 16] {
    let mut arr = [0.; 16];

    for i in 0..16 {
        arr[i] = slice[i];
    }

    arr
}
