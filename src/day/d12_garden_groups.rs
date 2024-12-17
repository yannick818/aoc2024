use std::fmt::{Display, Write};

use crate::util::{
    position::{Direction, Position},
    vec2d::Vec2D,
};

struct Field {
    typ: char,
    group: Option<usize>,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.group.is_some() { self.typ } else { '.' };
        f.write_char(c)
    }
}

struct Fields {
    fields: Vec2D<Field>,
    id_gen: usize,
}

impl Fields {
    fn parse(input: &str) -> Self {
        let mut rows = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Field {
                    typ: c,
                    group: None,
                });
            }
            rows.push(row);
        }
        Fields {
            fields: Vec2D(rows),
            id_gen: 0,
        }
    }

    fn find_neighbour(&mut self, start: Position) {
        let root = self.fields.get_mut(start).unwrap();
        let typ = root.typ;
        let group = root.group.unwrap();
        let mut positions = vec![start];
        while let Some(pos) = positions.pop() {
            match self.fields.get_mut(pos) {
                Some(field) if field.typ == typ => {
                    field.group = Some(group);
                    Direction::all()
                        .flat_map(|dir| pos.next(dir))
                        .filter(|pos| {
                            self.fields
                                .get(*pos)
                                .is_some_and(|field| field.group.is_none())
                        })
                        .for_each(|pos| {
                            positions.push(pos);
                        })
                }
                _ => {}
            }
        }
    }

    fn group(&mut self) {
        for pos in self.fields.iter_pos() {
            let field = self.fields.get_mut(pos).unwrap();
            if field.group.is_none() {
                let new_id = self.id_gen;
                self.id_gen += 1;
                field.group = Some(new_id);
                self.find_neighbour(pos);
                //println!("{}", self.fields);
            }
        }
    }

    fn price_per_fence(&self) -> usize {
        let mut groups = vec![(0_usize, 0_usize); self.id_gen];
        for pos in self.fields.iter_pos() {
            let field = self.fields.get(pos).unwrap();
            let id = field.group.unwrap();
            let (area, fence) = &mut groups[id];
            *area += 1;
            let fences = 4 - Direction::all()
                .flat_map(|dir| pos.next(dir))
                .flat_map(|pos| self.fields.get(pos))
                .filter(|field| field.group.unwrap() == id)
                .count();
            *fence += fences;
        }
        let mut sum = 0;
        for (area, fence) in groups.into_iter() {
            sum += area * fence;
        }
        sum
    }

    fn price_per_side(&self) -> usize {
        use Direction::*;
        let mut groups = vec![(0_usize, 0_usize); self.id_gen];
        for (group_id, (area, sides)) in groups.iter_mut().enumerate() {
            let is_in_group = |pos: Option<Position>| {
                pos.and_then(|pos| self.fields.get(pos))
                .filter(|field| field.group.unwrap() == group_id).is_some()
            };
            // iterate over column and rows to find continuous fences
            for (idx_row, row) in self.fields.0.iter().enumerate() {
                let mut sides_row = 0;
                let mut side_top = 0;
                let mut side_bot = 0;
                for (idx_col, field) in row.iter().enumerate() {
                    if field.group.unwrap() == group_id {
                        *area += 1;
                        let pos = Position(idx_row, idx_col);
                        let pos_top = pos.next(Up);
                        if is_in_group(pos_top) {
                            sides_row += side_top;
                            side_top = 0;
                        } else {
                            side_top = 1;
                        }
                        let pos_bot = pos.next(Down);
                        if is_in_group(pos_bot) {
                            sides_row += side_bot;
                            side_bot = 0;
                        } else {
                            side_bot = 1;
                        }
                    } else {
                        sides_row += side_top + side_bot;
                        side_top = 0;
                        side_bot = 0;
                    }
                }
                sides_row += side_top + side_bot;
                *sides += sides_row;
            }
            let cols = self.fields.0[0].len();
            let rows = self.fields.0.len();
            for idx_col in 0..cols {
                let mut sides_col = 0;
                let mut side_left = 0;
                let mut side_right = 0;
                for idx_row in 0..rows {
                    let pos = Position(idx_row, idx_col);
                    let field = self.fields.get(pos).unwrap();
                    if field.group.unwrap() == group_id {
                        //area already counted in row iteration
                        let pos_left = pos.next(Left);
                        if is_in_group(pos_left) {
                            sides_col += side_left;
                            side_left = 0;
                        } else {
                            side_left = 1;
                        }
                        let pos_right = pos.next(Right);
                        if is_in_group(pos_right) {
                            sides_col += side_right;
                            side_right = 0;
                        } else {
                            side_right = 1;
                        }
                    } else {
                        sides_col += side_left + side_right;
                        side_left = 0;
                        side_right = 0;
                    }
                }
                sides_col += side_left + side_right;
                *sides += sides_col;
            }
        }
        let mut sum = 0;
        for (area, sides) in groups.into_iter() {
            sum += area * sides;
        }
        sum
    }
}

pub fn part_one(input: &str) -> usize {
    let mut fields = Fields::parse(input);
    fields.group();
    //println!("{}", &fields.fields);
    fields.price_per_fence()
}

pub fn part_two(input: &str) -> usize {
    let mut fields = Fields::parse(input);
    fields.group();
    fields.price_per_side()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_big() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(1930, part_one(input));
        assert_eq!(1206, part_two(input));
    }

    #[test]
    fn test_example_two() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(80, part_two(input));
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(236, part_two(input));
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(368, part_two(input));
    }
}
