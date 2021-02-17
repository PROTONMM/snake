use crate::static_values;
use rand::Rng;

pub struct Apple {
    pub x_pos: f64,
    pub y_pos: f64,
}

fn random_place() -> (f64,f64) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(40..static_values::SCREEN_WIDTH-40) as f64;
    let y = rng.gen_range(40..static_values::SCREEN_HEIGHT-40) as f64;
    (x,y)
}

impl Apple {
    pub fn new() -> Apple {
        let (x,y) = random_place();
        Apple {x_pos: x, y_pos: y }
    }

    pub fn update(&mut self) {
        let (x,y) = random_place();
        self.x_pos = x;
        self.y_pos = y;
    }
}
