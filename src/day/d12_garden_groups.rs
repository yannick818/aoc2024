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

    fn price(&self) -> usize {
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
}

pub fn part_one(input: &str) -> usize {
    let mut fields = Fields::parse(input);
    fields.group();
    //println!("{}", &fields.fields);
    fields.price()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
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
    }
}
