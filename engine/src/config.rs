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
        14.472204, 14.519839, 14.416439, 14.508921, 15.4399185, 18.434225, 18.233442, 16.547981,
        45.424736, 55.613377, 64.75347, -27.860935, 20.596258, 50.147663, 72.93939, -12.07913,
    ],
    inner_pos_bonus: [
        14.455665, 14.60203, 14.593038, 14.517358, 14.499062, 14.5365715, 14.504547, 14.540269,
        14.561566, 14.504628, 14.6381, 14.442113, 14.441541, 14.5403, 14.545102, 14.191628,
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
