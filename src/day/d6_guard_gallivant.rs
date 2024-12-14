use std::fmt::{Debug, Write};

use Direction::*;
use Field::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position(usize, usize);

impl Position {
    fn next(&self, dir: Direction) -> Option<Position> {
        match dir {
            Up => self.0.checked_sub(1).map(|row| Position(row, self.1)),
            Down => Some(Position(self.0 + 1, self.1)),
            Right => Some(Position(self.0, self.1 + 1)),
            Left => self.1.checked_sub(1).map(|col| Position(self.0, col)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn rotate_90(&mut self) {
        *self = match &self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

enum Field {
    Obstacle,
    Free { visited: bool },
}

struct Map(Vec<Vec<Field>>);

impl Map {
    fn get(&self, pos: Position) -> Option<&Field> {
        self.0.get(pos.0).and_then(|vec| vec.get(pos.1))
    }

    fn get_mut(&mut self, pos: Position) -> Option<&mut Field> {
        self.0.get_mut(pos.0).and_then(|vec| vec.get_mut(pos.1))
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            let line: String = row
                .iter()
                .map(|field| match field {
                    Obstacle => '#',
                    Free { visited: true } => 'X',
                    Free { visited: false } => '.',
                })
                .collect();
            f.write_str(&line)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}
struct Guard {
    map: Map,
    position: Position,
    direction: Direction,
}

impl Guard {
    fn parse(input: &str) -> Self {
        let mut start = None;
        let mut dir = None;
        let mut rows = Vec::new();
        for (row_idx, line) in input.lines().enumerate() {
            let row = line
                .chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '.' => Free { visited: false },
                    '#' => Obstacle,
                    '^' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Up);
                        Free { visited: true }
                    }
                    '>' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Right);
                        Free { visited: true }
                    }
                    'v' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Down);
                        Free { visited: true }
                    }
                    '<' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Left);
                        Free { visited: true }
                    }
                    _ => unreachable!(),
                })
                .collect();
            rows.push(row);
        }
        Guard {
            map: Map(rows),
            position: start.unwrap(),
            direction: dir.unwrap(),
        }
    }
}

pub fn count_positions(input: &str) -> usize {
    let Guard {
        mut map,
        mut position,
        mut direction,
    } = Guard::parse(input);
    loop {
        //println!("{:?}", &map);
        assert!(matches!(
            &map.get(position),
            Some(Field::Free { visited: _ })
        ));
        let field = map.get_mut(position).unwrap();
        *field = Field::Free { visited: true };
        let next_pos = position.next(direction);
        let next_pos = match next_pos.and_then(|pos| map.get(pos)) {
            None => break,
            Some(Obstacle) => {
                direction.rotate_90();
                position.next(direction).unwrap()
            }
            Some(Free { visited: _ }) => next_pos.unwrap(),
        };
        position = next_pos;
    }
    map.0
        .iter()
        .flatten()
        .filter(|field| matches!(field, Free { visited: true }))
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_positions() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, count_positions(input));
    }
}
