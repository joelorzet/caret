#[allow(unused_imports)]
use crate::prelude::*;

impl Axis {
    pub const fn of(dir: Direction) -> Axis {
        match dir {
            Direction::Left | Direction::Right => Axis::Horizontal,
            Direction::Up | Direction::Down => Axis::Vertical,
        }
    }
}
