#[derive(Debug)]
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
        6.544019, 27.110014, 43.59264, -38.143482, 35.725357, 42.71728, -52.97087, 111.30266,
        59.907913, 58.364803, -43.458874, 104.321205, 94.55572, -32.904766, -16.015085, -13.032555,
    ],
    inner_pos_bonus: [
        -4.0064635, -3.377913, -18.217653, 27.451897, 51.7641, -17.680748, 0.11126831, -2.205288,
        -49.16594, 49.51228, -118.21852, 13.331266, -43.74143, -98.10443, 35.06091, 48.832836,
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
