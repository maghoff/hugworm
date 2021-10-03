use crate::segment::Segment;
use crate::turn::Turn;
use cgmath::{prelude::*, vec2, Vector2};
use std::collections::VecDeque;

pub struct Sequence {
    segments: VecDeque<Segment>,
    pub half_width: f32,
    pub is_dying: bool,
    pub grow: f32,
}

// create an arc with a starting point and normalized direction vector
fn arc(start: Vector2<f32>, dir: Vector2<f32>, r: f32, len: f32, clockwise: bool) -> Segment {
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
    pub fn new(
        pos: Vector2<f32>,
        dir: Vector2<f32>,
        turn: Turn,
        half_width: f32,
        grow: f32,
    ) -> Sequence {
        let mut sequence = Sequence {
            segments: Vec::new().into(),
            half_width,
            is_dying: false,
            grow,
        };
        sequence.new_segment(pos, dir, turn);
        sequence
    }

    pub fn new_at(other: &Sequence, half_width: f32, translate: f32) -> Sequence {
        let mut sequence = Sequence {
            segments: Vec::new().into(),
            half_width,
            is_dying: false,
            grow: other.length() + other.grow,
        };
        let mut new_head = (*other.segments.back().unwrap()).clone();
        let (_, dir) = new_head.ending();
        let orth = Vector2::new(dir.y, -dir.x);
        new_head.translate(orth * half_width * translate);
        new_head.tail_forward(new_head.len() - 0.01);
        sequence.segments.push_back(new_head);
        sequence
    }

    pub fn get_turn(&self) -> Option<Turn> {
        Some(self.segments.back()?.turn())
    }

    pub fn head_forward(&mut self, len: f32, turn: Turn) {
        if turn != self.get_turn().unwrap() {
            let (pos, dir) = self.segments.back().unwrap().ending();
            self.new_segment(pos, dir, turn);
        }

        self.segments.back_mut().unwrap().head_forward(len);
    }

    pub fn tail_forward(&mut self, len: f32) -> f32 {
        if self.segments.is_empty() {
            return len;
        }

        match self.segments.front_mut().unwrap().tail_forward(len) {
            Some(remainder) => {
                self.segments.pop_front();
                self.tail_forward(remainder)
            }
            None => 0.0,
        }
    }

    fn new_segment(&mut self, pos: Vector2<f32>, dir: Vector2<f32>, turn: Turn) {
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

    pub fn length(&self) -> f32 {
        self.segments.iter().map(|seg| seg.len()).sum()
    }

    pub fn generate_geometry(&self, dest: &mut Vec<f32>) {
        if self.segments.is_empty() {
            return;
        }

        let mut reach = 0.0;

        for segment in &self.segments {
            segment.generate_geometry(dest, reach, self.half_width);
            reach += segment.len();
        }

        let tail = self.segments.back().unwrap();
        let (start, dir) = tail.ending();
        let line = crate::segment::Segment::Line {
            start,
            dir,
            len: 0.0,
        };
        line.generate_geometry(dest, reach, self.half_width);
    }
}
