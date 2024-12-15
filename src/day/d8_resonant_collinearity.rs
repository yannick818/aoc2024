use std::collections::HashMap;
use std::fmt::{Display, Write};

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

#[derive(Clone, Copy)]
struct Position(usize, usize);

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
}

struct Map {
    map: Vec<Vec<Field>>,
    antennas: HashMap<char, Vec<Position>>,
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
        Map { map, antennas }
    }

    fn find_antinodes(&mut self) {
        for (_typ, antenna_positions) in self.antennas.iter() {
            let mut other_antennas = antenna_positions.clone();
            while let Some(pos) = other_antennas.pop() {
                for &antenna_pos in other_antennas.iter() {
                    let antinodes = pos.antidotes_pos(antenna_pos);
                    for antinode in antinodes.into_iter() {
                        if let Some(field) = self
                            .map
                            .get_mut(antinode.0)
                            .and_then(|vec| vec.get_mut(antinode.1))
                        {
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
    map.find_antinodes();
    println!("{}", map);
    map.map
        .iter()
        .flatten()
        .filter(|field| field.antinote)
        .count()
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            let line: String = row
                .iter()
                .map(|field| match (&field.typ, field.antinote) {
                    (FieldType::Antenna(c), _) => *c,
                    (FieldType::Empty, true) => '#',
                    (FieldType::Empty, false) => '.',
                })
                .collect();
            f.write_str(&line)?;
            f.write_char('\n')?;
        }
        Ok(())
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
    }
}
