#[allow(unused_imports)]
use crate::prelude::*;

impl Direction {
    pub const fn axis(self) -> Axis {
        Axis::of(self)
    }

    /// True for the directions that increase the coordinate.
    pub const fn is_forward(self) -> bool {
        matches!(self, Direction::Right | Direction::Down)
    }

    pub const fn opposite(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}
