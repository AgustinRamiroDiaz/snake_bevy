use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Coordinate(pub Vec2);

impl<T> From<(T, T)> for Coordinate
where
    T: Into<f32>,
{
    fn from(value: (T, T)) -> Self {
        Self(Vec2::new(value.0.into(), value.1.into()))
    }
}

impl std::hash::Hash for Coordinate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.x.to_bits().hash(state);
        self.0.y.to_bits().hash(state);
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

impl Eq for Coordinate {}
