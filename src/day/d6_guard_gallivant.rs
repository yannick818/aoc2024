use std::{
    collections::HashSet,
    fmt::{Display, Write},
    thread,
};

use Direction::*;
use FieldType::*;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone)]
struct Field {
    typ: FieldType,
    visited: bool,
    walked: HashSet<Direction>,
}

impl Field {
    fn new(typ: FieldType) -> Self {
        Self {
            typ,
            visited: false,
            walked: Default::default(),
        }
    }
}

#[derive(Clone, Copy)]
enum FieldType {
    Obstacle,
    Free,
}

#[derive(Clone)]
struct Map(Vec<Vec<Field>>);

impl Map {
    fn get(&self, pos: Position) -> Option<&Field> {
        self.0.get(pos.0).and_then(|vec| vec.get(pos.1))
    }

    fn get_mut(&mut self, pos: Position) -> Option<&mut Field> {
        self.0.get_mut(pos.0).and_then(|vec| vec.get_mut(pos.1))
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            let line: String = row
                .iter()
                .map(|field| match (field.typ, field.visited) {
                    (Obstacle, _) => '#',
                    (Free, true) => 'X',
                    (Free, false) => '.',
                })
                .collect();
            f.write_str(&line)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone)]
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
                    '.' => Field::new(Free),
                    '#' => Field::new(Obstacle),
                    '^' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Up);
                        Field::new(Free)
                    }
                    '>' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Right);
                        Field::new(Free)
                    }
                    'v' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Down);
                        Field::new(Free)
                    }
                    '<' => {
                        start = Some(Position(row_idx, col_idx));
                        dir = Some(Left);
                        Field::new(Free)
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

    fn walk(&mut self) -> End {
        loop {
            //println!("{:?}", self.map);
            let field = self.map.get_mut(self.position).unwrap();
            assert!(matches!(field.typ, Free));
            field.visited = true;
            let is_new = field.walked.insert(self.direction);
            if !is_new {
                return End::Loop;
            }
            self.position = loop {
                let next_pos = match self.position.next(self.direction) {
                    Some(pos) => pos,
                    None => {
                        return End::Exit;
                    }
                };
                match self.map.get(next_pos).map(|f| f.typ) {
                    None => {
                        return End::Exit;
                    }
                    Some(Obstacle) => {
                        self.direction.rotate_90();
                    }
                    Some(Free) => break next_pos,
                };
            };
        }
    }
}

#[derive(Debug)]
enum End {
    Loop,
    Exit,
}

pub fn part_one(input: &str) -> usize {
    let mut guard = Guard::parse(input);
    let end = guard.walk();
    assert!(matches!(end, End::Exit));
    guard
        .map
        .0
        .iter()
        .flatten()
        .filter(|field| field.visited)
        .count()
}

//TODO is there a faster way? this takes 5s in release and 60s in debug
pub fn part_two(input: &str) -> usize {
    let mut threads = Vec::new();
    let original = Guard::parse(input);
    for (idx_row, row) in original.map.0.iter().enumerate() {
        for (idx_col, field) in row.iter().enumerate() {
            if original.position == Position(idx_row, idx_col) {
                continue;
            }
            if matches!(field.typ, Obstacle) {
                continue;
            }
            let mut guard = original.clone();
            guard.map.0[idx_row][idx_col].typ = Obstacle;
            let handle = thread::spawn(move || guard.walk());
            threads.push(handle);
        }
    }
    threads
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .filter(|end| matches!(end, End::Loop))
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
        assert_eq!(41, part_one(input));
        assert_eq!(6, part_two(input));
    }
}
