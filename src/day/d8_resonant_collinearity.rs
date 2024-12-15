use crate::util::position::*;
use crate::util::vec2d::Vec2D;
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::vec;

enum FieldType {
    Empty,
    Antenna(char),
}

struct Field {
    typ: FieldType,
    antinote: bool,
}

impl Field {
    fn new(typ: FieldType) -> Self {
        Field {
            typ,
            antinote: false,
        }
    }
}

impl Position {
    fn antidotes_pos(&self, other: Position) -> Vec<Position> {
        let mut antinodes = Vec::new();
        let diff_x = other.0 as isize - self.0 as isize;
        let diff_y = other.1 as isize - self.1 as isize;
        let anti_x = other.0 as isize + diff_x;
        let anti_y = other.1 as isize + diff_y;
        if !anti_x.is_negative() && !anti_y.is_negative() {
            antinodes.push(Position(anti_x as usize, anti_y as usize));
        }
        let anti_x = self.0 as isize - diff_x;
        let anti_y = self.1 as isize - diff_y;
        if !anti_x.is_negative() && !anti_y.is_negative() {
            antinodes.push(Position(anti_x as usize, anti_y as usize));
        }
        antinodes
    }

    fn antidotes_pos_resonant(&self, other: Position, end: Position) -> Vec<Position> {
        let in_range = |x: isize, y: isize| {
            if !x.is_negative() && !y.is_negative() && x <= end.0 as isize && y <= end.1 as isize {
                Some(Position(x as usize, y as usize))
            } else {
                None
            }
        };
        let mut antinodes = vec![*self, other];
        let diff_x = other.0 as isize - self.0 as isize;
        let diff_y = other.1 as isize - self.1 as isize;
        let mut pos = other;
        loop {
            let anti_x = pos.0 as isize + diff_x;
            let anti_y = pos.1 as isize + diff_y;
            if let Some(new_pos) = in_range(anti_x, anti_y) {
                antinodes.push(new_pos);
                pos = new_pos;
            } else {
                break;
            }
        }
        pos = *self;
        loop {
            let anti_x = pos.0 as isize - diff_x;
            let anti_y = pos.1 as isize - diff_y;
            if let Some(new_pos) = in_range(anti_x, anti_y) {
                antinodes.push(new_pos);
                pos = new_pos;
            } else {
                break;
            }
        }
        antinodes
    }
}

struct Map {
    map: Vec2D<Field>,
    antennas: HashMap<char, Vec<Position>>,
    end: Position,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut map = Vec::new();
        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
        for (idx_row, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (idx_col, c) in line.chars().enumerate() {
                let typ = match c {
                    '.' => FieldType::Empty,
                    c => FieldType::Antenna(c),
                };
                if let FieldType::Antenna(c) = typ {
                    let pos = Position(idx_row, idx_col);
                    if let Some(positions) = antennas.get_mut(&c) {
                        positions.push(pos);
                    } else {
                        let old = antennas.insert(c, vec![pos]);
                        assert!(old.is_none())
                    }
                }
                row.push(Field::new(typ));
            }
            map.push(row);
        }
        let x = map.len() - 1;
        let y = map[0].len() - 1;
        Map {
            map: Vec2D(map),
            antennas,
            end: Position(x, y),
        }
    }

    fn find_antinodes(&mut self, resonance: bool) {
        for (_typ, antenna_positions) in self.antennas.iter() {
            let mut other_antennas = antenna_positions.clone();
            while let Some(pos) = other_antennas.pop() {
                for &antenna_pos in other_antennas.iter() {
                    let antinodes = if resonance {
                        pos.antidotes_pos_resonant(antenna_pos, self.end)
                    } else {
                        pos.antidotes_pos(antenna_pos)
                    };
                    for antinode in antinodes.into_iter() {
                        if let Some(field) = self.map.get_mut(antinode) {
                            field.antinote = true;
                        }
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut map = Map::parse(input);
    map.find_antinodes(false);
    //println!("{}", map.map);
    map.map.0
        .iter()
        .flatten()
        .filter(|field| field.antinote)
        .count()
}

pub fn part_two(input: &str) -> usize {
    let mut map = Map::parse(input);
    map.find_antinodes(true);
    //println!("{}", map);
    map.map.0
        .iter()
        .flatten()
        .filter(|field| field.antinote)
        .count()
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match (&self.typ, self.antinote) {
            (FieldType::Antenna(c), _) => *c,
            (FieldType::Empty, true) => '#',
            (FieldType::Empty, false) => '.',
        };
        f.write_char(c)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(14, part_one(input));
        assert_eq!(34, part_two(input));
    }
}
