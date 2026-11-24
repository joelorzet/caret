#[allow(unused_imports)]
use crate::prelude::*;

impl Palette {
    pub fn get(&self, index: u8) -> Option<Rgb> {
        match index {
            0..=7 => Some(self.normal[index as usize]),
            8..=15 => Some(self.bright[(index - 8) as usize]),
            _ => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Rgb> + '_ {
        self.normal.iter().chain(self.bright.iter()).copied()
    }
}
