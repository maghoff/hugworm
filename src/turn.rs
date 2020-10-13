#[derive(PartialEq, Clone, Copy)]
pub enum Turn {
    Left { radius: f32 },
    Straight,
    Right { radius: f32 },
}

