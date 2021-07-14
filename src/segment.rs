use crate::turn::Turn;
use cgmath::{vec2, Vector2};

const WIDTH: f32 = 0.1;
const HALF_WIDTH: f32 = WIDTH / 2.0;

pub enum Segment {
    Line {
        start: Vector2<f32>,
        dir: Vector2<f32>,
        len: f32,
    },
    Arc {
        center: Vector2<f32>,
        r: f32,
        ang_dir: f32,
        len: f32,
        start_ang: f32,
    },
}

impl Segment {
    pub fn generate_geometry(&self, dest: &mut Vec<f32>, start_reach: f32) {
        match self {
            Segment::Line { start, dir, .. } => {
                let side = vec2(-dir.y, dir.x);
                let left = side * HALF_WIDTH;

                dest.push(start.x + left.x);
                dest.push(start.y + left.y);
                dest.push(0.0);
                dest.push(start_reach);

                dest.push(start.x - left.x);
                dest.push(start.y - left.y);
                dest.push(1.0);
                dest.push(start_reach);
            }
            Segment::Arc {
                center,
                r,
                ang_dir,
                len,
                start_ang,
            } => {
                let end_ang = start_ang + ang_dir * len / r;

                let steps = 30;
                let left_r = r - HALF_WIDTH * (*ang_dir as f32);
                let right_r = r + HALF_WIDTH * (*ang_dir as f32);
                let ang_step = (end_ang - start_ang) / steps as f32;
                let len_step = len / steps as f32;
                for step in 0..steps {
                    let ang = start_ang + ang_step * step as f32;

                    dest.push(center.x + left_r * ang.cos());
                    dest.push(center.y + left_r * ang.sin());
                    dest.push(0.0);
                    dest.push(start_reach + len_step * step as f32);

                    dest.push(center.x + right_r * ang.cos());
                    dest.push(center.y + right_r * ang.sin());
                    dest.push(1.0);
                    dest.push(start_reach + len_step * step as f32);
                }
            }
        }
    }

    pub fn len(&self) -> f32 {
        match self {
            Segment::Line { len, .. } => *len,
            Segment::Arc { len, .. } => *len,
        }
    }

    // return the position and direction of the ending
    pub fn ending(&self) -> (Vector2<f32>, Vector2<f32>) {
        match self {
            Segment::Line { start, dir, len } => {
                let end = start + dir * *len;

                (end, *dir)
            }
            Segment::Arc {
                center,
                r,
                ang_dir,
                len,
                start_ang,
            } => {
                let end_ang = start_ang + ang_dir * len / r;
                let end_norm = vec2(end_ang.cos(), end_ang.sin());
                let end = center + *r * end_norm;
                let end_dir = *ang_dir * vec2(-end_norm.y, end_norm.x);

                (end, end_dir)
            }
        }
    }

    pub fn head_forward(&mut self, add_len: f32) {
        match self {
            Segment::Line { len, .. } => {
                *len += add_len;
            }
            Segment::Arc { len, .. } => {
                *len += add_len;
            }
        }
    }

    pub fn tail_forward(&mut self, sub_len: f32) -> Option<f32> {
        match self {
            Segment::Line {
                start, dir, len, ..
            } => {
                if sub_len >= *len {
                    Some(sub_len - *len)
                } else {
                    *start += *dir * sub_len;
                    *len -= sub_len;
                    None
                }
            }
            Segment::Arc {
                r,
                ang_dir,
                len,
                start_ang,
                ..
            } => {
                if sub_len >= *len {
                    Some(sub_len - *len)
                } else {
                    *start_ang += *ang_dir * sub_len / *r;
                    *len -= sub_len;
                    None
                }
            }
        }
    }

    pub fn turn(&self) -> Turn {
        match *self {
            Segment::Line { .. } => Turn::Straight,
            Segment::Arc { r, ang_dir, .. } => {
                if ang_dir < 0.0 {
                    Turn::Right { radius: r }
                } else {
                    Turn::Left { radius: r }
                }
            }
        }
    }
}
