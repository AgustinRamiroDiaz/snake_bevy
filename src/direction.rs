use bevy::prelude::Vec2;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Into<Vec2> for Direction {
    fn into(self) -> Vec2 {
        let x = match self {
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
        };

        Vec2::new(x.0 as f32, x.1 as f32)
    }
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }
}
