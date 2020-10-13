use crate::segment::Segment;
use cgmath::{prelude::*, vec2, Vector2};
use std::collections::VecDeque;

#[derive(PartialEq, Clone, Copy)]
pub enum Turn {
    Left { radius: f32 },
    Straight,
    Right { radius: f32 },
}

pub struct Sequence {
    segments: VecDeque<Segment>,
}

// create an arc with a starting point and normalized direction vector
fn arc(
    start: Vector2<f32>,
    dir: Vector2<f32>,
    r: f32,
    len: f32,
    clockwise: bool,
) -> Segment {
    let normal_dir = vec2(-dir.y, dir.x);
    let ang_dir = if clockwise { -1.0 } else { 1.0 };
    let center = start + r * normal_dir * ang_dir;
    Segment::Arc {
        center,
        r,
        ang_dir,
        len,
        start_ang: vec2(1.0, 0.0).angle((-ang_dir) * normal_dir).0,
    }
}

impl Sequence {
    pub fn new(pos: Vector2<f32>, dir: Vector2<f32>, turn: Turn) -> Sequence {
        Sequence {
            segments: vec![match turn {
                Turn::Left { radius } => arc(pos, dir, radius, 0., false),
                Turn::Straight => Segment::Line {
                    start: pos,
                    dir,
                    len: 0.,
                },
                Turn::Right { radius } => arc(pos, dir, radius, 0., true),
            }]
            .into(),
        }
    }

    pub fn head_forward(&mut self, len: f32) {
        self.segments.back_mut().unwrap().head_forward(len);
    }

    pub fn tail_forward(&mut self, len: f32) {
        match self.segments.front_mut().unwrap().tail_forward(len) {
            Some(remainder) => {
                self.segments.pop_front();
                self.tail_forward(remainder);
            }
            None => (),
        }
    }

    pub fn turn_to(&mut self, turn: Turn) {
        let (pos, dir) = self.segments.back().unwrap().ending();

        self.segments.push_back(match turn {
            Turn::Left { radius } => arc(pos, dir, radius, 0., false),
            Turn::Straight => Segment::Line {
                start: pos,
                dir,
                len: 0.,
            },
            Turn::Right { radius } => arc(pos, dir, radius, 0., true),
        });
    }

    pub fn generate_geometry(&self, dest: &mut Vec<f32>) {
        let mut reach = 0.0;

        for segment in &self.segments {
            segment.generate_geometry(dest, reach);
            reach += segment.len();
        }

        let tail = self.segments.back().unwrap();
        let (start, dir) = tail.ending();
        let line = crate::segment::Segment::Line {
            start,
            dir,
            len: 0.0,
        };
        line.generate_geometry(dest, reach);
    }
}
