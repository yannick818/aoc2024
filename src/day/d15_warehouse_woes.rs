use std::fmt::{Display, Write};

use crate::util::{
    position::{Direction, Position},
    vec2d::Vec2D,
};

enum Field {
    Empty,
    Wall,
    Box,
}

struct Map {
    fields: Vec2D<Field>,
    movements: Vec<Direction>,
    robot: Position,
}

impl Map {
    fn parse(input: &str) -> Self {
        let (map, moves) = input.split_once("\n\n").unwrap();
        let mut rows = Vec::new();
        let mut robot = None;
        for (idx_row, line) in map.lines().enumerate() {
            let mut row = Vec::new();
            for (idx_col, c) in line.chars().enumerate() {
                let field = match c {
                    '#' => Field::Wall,
                    '.' => Field::Empty,
                    'O' => Field::Box,
                    '@' => {
                        robot = Some(Position(idx_row, idx_col));
                        Field::Empty
                    }
                    _ => unreachable!(),
                };
                row.push(field);
            }
            rows.push(row);
        }
        let mut movements = Vec::new();
        for mov in moves.lines() {
            for c in mov.chars() {
                let dir = match c {
                    '^' => Direction::Up,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    _ => unreachable!(),
                };
                movements.push(dir);
            }
        }
        Self {
            fields: Vec2D(rows),
            movements,
            robot: robot.unwrap(),
        }
    }

    fn make_moves(&mut self) {
        for &dir in self.movements.iter() {
            let next_robot = self.robot.next(dir).unwrap();
            use Field::*;
            match self.fields.get(next_robot).unwrap() {
                Wall => {}
                Empty => {
                    self.robot = next_robot;
                }
                Box => {
                    let mut next = next_robot;
                    loop {
                        next = next.next(dir).unwrap();
                        let field = self.fields.get_mut(next).unwrap();
                        match field {
                            Wall => break,
                            Empty => {
                                *field = Box;
                                *self.fields.get_mut(next_robot).unwrap() = Empty;
                                self.robot = next_robot;
                                break;
                            }
                            Box => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut map = Map::parse(input);
    println!("{}", map.fields);
    map.make_moves();
    println!("{}", map.fields);
    let mut sum = 0;
    for pos in map.fields.iter_pos() {
        if let Field::Box = map.fields.get(pos).unwrap() {
            sum += 100 * pos.0 + pos.1;
        }
    }
    sum
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Field::Empty => '.',
            Field::Box => 'O',
            Field::Wall => '#',
        };
        f.write_char(c)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_small() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(2028, part_one(input));
    }

    #[test]
    fn test_example_big() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(10092, part_one(input));
    }
}
