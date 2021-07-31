use crate::sequence::Sequence;
use crate::turn::Turn;
use cgmath::{prelude::*, vec2};

const KEY_LEFT: u32 = 37;
const KEY_UP: u32 = 38;
const KEY_RIGHT: u32 = 39;
const KEY_DOWN: u32 = 40;

pub struct Scene {
    pub worm: Sequence,
    press_up: bool,
    press_down: bool,
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
            press_up: false,
            press_down: false,
            press_left: false,
            press_right: false,
        }
    }

    pub fn key_event(&mut self, code: u32, depressed: bool) -> bool {
        let handled = match code {
            KEY_UP => {
                self.press_up = depressed;
                true
            }
            KEY_DOWN => {
                self.press_down = depressed;
                true
            }
            KEY_LEFT => {
                self.press_left = depressed;
                true
            }
            KEY_RIGHT => {
                self.press_right = depressed;
                true
            }
            _ => false,
        };

        handled
    }

    pub fn update(&mut self) {
        log::trace!(
            "update: up={} down={} left={} right={}",
            self.press_up,
            self.press_down,
            self.press_left,
            self.press_right
        );

        const SPEED: f32 = 0.02;
        const RADIUS: f32 = 0.3;

        let turn = match (self.press_left, self.press_right) {
            (true, false) => Turn::Left { radius: RADIUS },
            (false, true) => Turn::Right { radius: RADIUS },
            _ => Turn::Straight,
        };

        if !self.press_down {
            self.worm.head_forward(SPEED, turn);
        }
        if !self.press_up {
            self.worm.tail_forward(SPEED);
        }
    }
}
