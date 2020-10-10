use crate::sequence::{Sequence, Turn};
use cgmath::{prelude::*, vec2};

pub struct Scene {
    pub worm: Sequence,
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
        sequence.turn_to(Turn::Left { radius: 0.3 });

        Scene { worm: sequence }
    }

    pub fn update(&mut self) {
        const SPEED: f32 = 0.02;
        self.worm.head_forward(SPEED);
        self.worm.tail_forward(SPEED);
    }
}
