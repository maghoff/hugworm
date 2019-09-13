use cgmath::{vec2, Vector2, InnerSpace};

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
        dir: f32,
        len: f32,
        start_ang: f32,
    },
}

impl Segment {
    pub fn generate_geometry(&self, dest: &mut Vec<f32>) {
        match self {
            Segment::Line { start, dir, len } => {
                let end = start + dir * *len;
                let side = vec2(-dir.y, dir.x);
                let hw = side * HALF_WIDTH;

                dest.push(start.x + hw.x);
                dest.push(start.y + hw.y);

                dest.push(start.x - hw.x);
                dest.push(start.y - hw.y);

                dest.push(end.x + hw.x);
                dest.push(end.y + hw.y);

                dest.push(end.x - hw.x);
                dest.push(end.y - hw.y);
            }
            Segment::Arc {
                center,
                r,
                dir,
                len,
                start_ang,
            } => {
                let end_ang = start_ang + dir * len / r;

                let steps = 30;
                let ir = r - HALF_WIDTH;
                let or = r + HALF_WIDTH;
                let ang_step = (end_ang - start_ang) / (steps - 1) as f32;
                for step in 0..steps {
                    let ang = start_ang + ang_step * step as f32;

                    dest.push(center.x + ir * ang.cos());
                    dest.push(center.y + ir * ang.sin());

                    dest.push(center.x + or * ang.cos());
                    dest.push(center.y + or * ang.sin());
                }
            }
        }
    }

    // return the position and normalized direction of the ending
    pub fn ending(&self) -> (Vector2<f32>, Vector2<f32>) {
        match self {
            Segment::Line { start, dir, len } => {
                let end = start + dir * *len;

                (end, *dir)
            }
            Segment::Arc {
                center,
                r,
                dir,
                len,
                start_ang,
            } => {
                let end_ang = start_ang + dir * len / r;
                let end_norm = vec2(end_ang.cos(), end_ang.sin());
                let end = center + *r * end_norm;
                let end_dir = *dir * vec2(-end_norm.y, end_norm.x);

                (end, end_dir)
            }
        }
    }
}

// create an arc with a starting point and normalized direction vector
pub fn arc(start: Vector2<f32>, dir: Vector2<f32>, r: f32, len: f32, clockwise: bool) -> Segment {
    let normal_dir = vec2(-dir.y, dir.x);
    let dir_sign = if clockwise { -1.0 } else { 1.0 };
    let center = start + r * normal_dir * dir_sign;
    Segment::Arc {
        center: center,
        r: r,
        dir: dir_sign,
        len: len,
        start_ang: vec2(1.0, 0.0).angle((-dir_sign) * normal_dir).0,
    }
}
