use crate::sequence::Sequence;
use crate::turn::Turn;
use cgmath::{prelude::*, vec2};
use retain_mut::RetainMut;

#[derive(PartialEq)]
enum FlipFlop {
    READY,
    SET,
    GONE,
}

pub struct Scene {
    pub worms: Vec<Sequence>,
    grow: bool,
    shrink: bool,
    turn_left: bool,
    turn_right: bool,
    split: FlipFlop,
}

impl Scene {
    pub fn new() -> Scene {
        let mut sequence = Sequence::new(
            vec2(-1.0, -0.5),
            vec2(2., 1.).normalize(),
            Turn::Straight,
            0.05,
            0.4,
        );
        // sequence.head_forward(0.4, Turn::Straight);
        // sequence.head_forward(0.3, Turn::Right { radius: 0.3 });
        // sequence.head_forward(0.6, Turn::Left { radius: 0.3 });
        sequence.head_forward(0.2, Turn::Straight);

        Scene {
            worms: vec![sequence],
            grow: false,
            shrink: false,
            turn_left: false,
            turn_right: false,
            split: FlipFlop::READY,
        }
    }

    pub fn set_grow(&mut self, value: bool) {
        self.grow = value;
    }

    pub fn set_shrink(&mut self, value: bool) {
        self.shrink = value;
    }

    pub fn set_turn_left(&mut self, value: bool) {
        self.set_split(value);
        self.turn_left = value;
    }

    pub fn set_turn_right(&mut self, value: bool) {
        self.set_split(value);
        self.turn_right = value;
    }

    pub fn set_split(&mut self, value: bool) {
        if value {
            if self.split == FlipFlop::READY {
                self.split = FlipFlop::SET;
            }
        } else {
            self.split = FlipFlop::READY;
        }
    }

    pub fn update(&mut self) {
        log::trace!(
            "update: grow={} shrink={} left={} right={}",
            self.grow,
            self.shrink,
            self.turn_left,
            self.turn_right
        );

        if self.split == FlipFlop::SET {
            self.split = FlipFlop::GONE;
            let mut new_worms = vec![];
            for worm in &mut self.worms {
                if worm.is_dying {
                    continue;
                }

                worm.is_dying = true;
                new_worms.push(Sequence::new_at(worm, worm.half_width / 2.0, -1.0));
                new_worms.push(Sequence::new_at(worm, worm.half_width / 2.0, 1.0));
            }
            self.worms.append(&mut new_worms);
        }

        const SPEED: f32 = 0.02;
        const RADIUS: f32 = 0.3;

        let mut turn = match (self.turn_left, self.turn_right) {
            (true, false) => Turn::Left { radius: RADIUS },
            (false, true) => Turn::Right { radius: RADIUS },
            _ => Turn::Straight,
        };

        if !self.shrink {
            for worm in &mut self.worms {
                if !worm.is_dying {
                    worm.head_forward(SPEED, turn);
                    turn = turn.mirror();
                }
            }
        }
        if !self.grow {
            self.worms.retain_mut(|worm| {
                let mut stride = SPEED;
                if worm.grow > stride {
                    worm.grow -= stride;
                    return true;
                }
                stride -= worm.grow;
                worm.grow = 0.0;
                let remaining = &mut worm.tail_forward(stride);
                *remaining == 0.0
            });
        }
    }
}
