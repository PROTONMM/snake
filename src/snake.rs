use crate::static_values;

pub struct Tail {
    pub x_pos: f64,
    pub y_pos: f64,
}

pub struct Snake {
    pub x_pos: f64,
    pub y_pos: f64,
    pub angle: f64,
    pub step: i32,
    pub tail: Vec<Tail>,
}

impl Snake {
    pub fn new(x:f64, y:f64, angle:f64) -> Snake {
        Snake {x_pos: x, y_pos: y, angle: angle, step: 0, tail: Vec::new() }
    }
    pub fn add_tail(&mut self, count: i32) {
        let x;
        let y;
        if self.tail.len() == 0 {
            x = self.x_pos;
            y = self.y_pos;
        }else {
            x = self.tail[self.tail.len()-1].x_pos;
            y = self.tail[self.tail.len()-1].y_pos;
        }
        for _ in 0..count {
            let t = Tail { x_pos: x, y_pos: y };
            self.tail.push(t);
        }
    }

    pub fn next_step(&mut self) {
        // increment step
        self.step += 1;
        // step modulo
        if self.step % static_values::STEP_TAIL == 0 {
            for i in (1..self.tail.len()).rev() {
                self.tail[i].y_pos = self.tail[i - 1].y_pos;
                self.tail[i].x_pos = self.tail[i - 1].x_pos;
            }
            self.tail[0].x_pos = self.x_pos;
            self.tail[0].y_pos = self.y_pos;
        }
        let pi_angle = self.angle /180.0 * std::f64::consts::PI;
        let step_x = pi_angle.cos()*static_values::STEP_LEN;
        let step_y = pi_angle.sin()*static_values::STEP_LEN;
        self.y_pos += step_y;
        self.x_pos += step_x;
    }
}