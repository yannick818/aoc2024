use std::fmt::{Display, Write};

use super::position::Position;

#[derive(Debug, Clone)]
pub struct Vec2D<T>(pub Vec<Vec<T>>);

impl<T> Vec2D<T> {
    pub fn get(&self, pos: Position) -> Option<&T> {
        self.0.get(pos.0).and_then(|vec| vec.get(pos.1))
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.0.get_mut(pos.0).and_then(|vec| vec.get_mut(pos.1))
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = Position> {
        let rows = self.0.len();
        let cols = self.0[0].len();
        (0..rows).flat_map(move |row| (0..cols).map(move |col| Position(row, col)))
    }
}

impl<T: Display> Display for Vec2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for item in row.iter() {
                f.write_fmt(format_args!("{}", item))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
