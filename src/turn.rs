#[derive(PartialEq, Clone, Copy)]
pub enum Turn {
    Left { radius: f32 },
    Straight,
    Right { radius: f32 },
}

impl Turn {
    pub fn mirror(&self) -> Turn {
        match self {
            Self::Left { radius } => Self::Right { radius: *radius },
            Self::Straight => Self::Straight,
            Self::Right { radius } => Self::Left { radius: *radius },
        }
    }
}