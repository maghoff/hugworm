use crate::sequence::Sequence;
use crate::turn::Turn;
use cgmath::{prelude::*, vec2};

const KEY_LEFT : u32 = 37;
const KEY_RIGHT : u32 = 39;

pub struct Scene {
    pub worm: Sequence,
    press_left: bool,
    press_right: bool,
}

impl Scene {
    pub fn new() -> Scene {
        let mut sequence = Sequence::new(vec2(-0.7, 0.), vec2(2., 1.).normalize(), Turn::Straight);
        sequence.head_forward(0.4, Turn::Straight);
        sequence.head_forward(0.3, Turn::Right { radius: 0.3 });
        sequence.head_forward(0.6, Turn::Left { radius: 0.3 });
        sequence.head_forward(0.2, Turn::Straight);

        Scene {
            worm: sequence,
            press_left: false,
            press_right: false,
        }
    }

    pub fn key_event(&mut self, code: u32, depressed: bool) -> bool {
        let handled = match code {
            KEY_LEFT => { self.press_left = depressed; true }
            KEY_RIGHT => { self.press_right = depressed; true }
            _ => false,
        };

        handled
    }

    pub fn update(&mut self) {
        const SPEED: f32 = 0.02;
        const RADIUS: f32 = 0.3;

        let turn = match (self.press_left, self.press_right) {
            (true, false) => Turn::Left { radius: RADIUS },
            (false, true) => Turn::Right { radius: RADIUS },
            _ => Turn::Straight,
        };

        self.worm.head_forward(SPEED, turn);
        self.worm.tail_forward(SPEED);
    }
}
