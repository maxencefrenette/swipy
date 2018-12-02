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
        87.79041,
        69.05487,
        57.168068,
        29.885117,
        10.603633,
        9.633699,
        -9.814237,
        -69.98233,
        -178.96976,
        -212.71788,
        -0.077834405,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    ],
    inner_pos_bonus: [
        77.0028, 65.47649, 53.37442, 43.31748, 34.485863, 18.012466, -12.514331, -73.97131,
        -184.62035, -224.8075, -3.1455405, 0.0, 0.0, 0.0, 0.0, 0.0,
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
