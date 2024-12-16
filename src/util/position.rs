use Direction::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn next(&self, dir: Direction) -> Option<Position> {
        match dir {
            Up => self.0.checked_sub(1).map(|row| Position(row, self.1)),
            Down => Some(Position(self.0 + 1, self.1)),
            Right => Some(Position(self.0, self.1 + 1)),
            Left => self.1.checked_sub(1).map(|col| Position(self.0, col)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn rotate_90(&mut self) {
        *self = match &self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        [Up, Left, Right, Down].into_iter()
    }
}
