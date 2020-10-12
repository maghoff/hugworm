use crate::sequence::{Sequence, Turn};
use cgmath::{prelude::*, vec2};

const RADIUS: f32 = 0.3;
const KEY_LEFT : u32 = 37;
const KEY_RIGHT : u32 = 39;

pub struct Scene {
    pub worm: Sequence,
    press_left: bool,
    press_right: bool,
    next_turn: Turn,
    last_turn: Turn,
}

impl Scene {
    pub fn new() -> Scene {
        let mut sequence = Sequence::new(vec2(-0.7, 0.), vec2(2., 1.).normalize(), Turn::Straight);
        sequence.head_forward(0.4);
        sequence.turn_to(Turn::Right { radius: 0.3 });
        sequence.head_forward(0.3);
        sequence.turn_to(Turn::Left { radius: 0.3 });
        sequence.head_forward(0.6);
        sequence.turn_to(Turn::Straight);
        sequence.head_forward(0.2);
        let last_turn = Turn::Left { radius: 0.3 };
        sequence.turn_to(last_turn);

        Scene {
            worm: sequence,
            press_left: false,
            press_right: false,
            last_turn,
            next_turn: last_turn,
        }
    }

    pub fn key_event(&mut self, code: u32, depressed: bool) -> bool {
        const LEFT : Turn = Turn::Left { radius: RADIUS };
        const RIGHT : Turn = Turn::Right { radius: RADIUS };
        const STRAIGHT : Turn = Turn::Straight;

        let handled = match code {
            KEY_LEFT => {
                self.press_left = depressed;

                self.next_turn = match depressed {
                    true => LEFT,
                    false if self.press_right => RIGHT,
                    _ => STRAIGHT,
                };

                true
            }
            KEY_RIGHT => {
                self.press_right = depressed;

                self.next_turn = match depressed {
                    true => RIGHT,
                    false if self.press_left => LEFT,
                    _ => STRAIGHT,
                };

                true
            }
            _ => false,
        };

        handled
    }

    pub fn update(&mut self) {
        const SPEED: f32 = 0.02;

        if self.next_turn != self.last_turn {
            self.worm.turn_to(self.next_turn);
            self.last_turn = self.next_turn;
        }

        self.worm.head_forward(SPEED);
        self.worm.tail_forward(SPEED);
    }
}
