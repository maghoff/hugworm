use crate::sequence::Sequence;
use crate::turn::Turn;
use cgmath::{prelude::*, vec2};

pub struct Scene {
    pub worm: Sequence,
    grow: bool,
    shrink: bool,
    turn_left: bool,
    turn_right: bool,
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
            grow: false,
            shrink: false,
            turn_left: false,
            turn_right: false,
        }
    }

    pub fn set_grow(&mut self, value: bool) {
        self.grow = value;
    }

    pub fn set_shrink(&mut self, value: bool) {
        self.shrink = value;
    }

    pub fn set_turn_left(&mut self, value: bool) {
        self.turn_left = value;
    }

    pub fn set_turn_right(&mut self, value: bool) {
        self.turn_right = value;
    }

    pub fn update(&mut self) {
        log::trace!(
            "update: grow={} shrink={} left={} right={}",
            self.grow,
            self.shrink,
            self.turn_left,
            self.turn_right
        );

        const SPEED: f32 = 0.02;
        const RADIUS: f32 = 0.3;

        let turn = match (self.turn_left, self.turn_right) {
            (true, false) => Turn::Left { radius: RADIUS },
            (false, true) => Turn::Right { radius: RADIUS },
            _ => Turn::Straight,
        };

        if !self.shrink {
            self.worm.head_forward(SPEED, turn);
        }
        if !self.grow {
            self.worm.tail_forward(SPEED);
        }
    }
}
