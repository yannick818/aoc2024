use std::collections::HashSet;

use crate::util::{
    position::{Direction, Position},
    vec2d::Vec2D,
};

type Height = u8;

struct Hiker<'a> {
    map: &'a Vec2D<Height>,
    pos: Position,
    height: Height,
}

enum HikeStep<'a> {
    ReachedTop(Position),
    Split(Vec<Hiker<'a>>),
}

impl<'a> Hiker<'a> {
    fn hike(self) -> HikeStep<'a> {
        if *self.map.get(self.pos).unwrap() == 9 {
            return HikeStep::ReachedTop(self.pos);
        }
        use Direction::*;
        let next_height = self.height + 1;
        let paths = [Up, Down, Left, Right]
            .into_iter()
            .flat_map(|dir| self.pos.next(dir))
            .filter(|&pos| match self.map.get(pos) {
                None => false,
                Some(&heigh) => heigh == next_height,
            })
            .map(|pos| Hiker {
                map: self.map,
                pos,
                height: next_height,
            })
            .collect();
        HikeStep::Split(paths)
    }
}

struct Map {
    map: Vec2D<Height>,
    starts: Vec<Position>,
}

struct HikeResult {
    reached_tops: usize,
    unique_ways: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut starts = Vec::new();
        let mut map = Vec::new();
        for (idx_row, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (idx_col, c) in line.chars().enumerate() {
                let height = c.to_digit(10).unwrap() as Height;
                row.push(height);
                if height == 0 {
                    starts.push(Position(idx_row, idx_col));
                }
            }
            map.push(row);
        }
        Map {
            map: Vec2D(map),
            starts,
        }
    }

    fn hike(&self, start: Position) -> HikeResult {
        let mut reached_top = HashSet::new();
        let mut unique_ways = 0;
        let hiker = Hiker {
            map: &self.map,
            pos: start,
            height: 0,
        };
        let mut paths = vec![hiker];
        while let Some(path) = paths.pop() {
            use HikeStep::*;
            match path.hike() {
                ReachedTop(top) => {
                    reached_top.insert(top);
                    unique_ways += 1;
                }
                Split(new_paths) => {
                    for new_path in new_paths.into_iter() {
                        paths.push(new_path);
                    }
                }
            }
        }
        HikeResult {
            reached_tops: reached_top.len(),
            unique_ways,
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let map = Map::parse(input);
    let mut sum = 0;
    for start in map.starts.iter() {
        let score = map.hike(*start).reached_tops;
        sum += score;
    }
    sum
}

pub fn part_two(input: &str) -> usize {
    let map = Map::parse(input);
    let mut sum = 0;
    for start in map.starts.iter() {
        let rating = map.hike(*start).unique_ways;
        sum += rating;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(5, {
            let map = Map::parse(input);
            map.hike(map.starts[0]).reached_tops
        });
        assert_eq!(36, part_one(input));
        assert_eq!(81, part_two(input));
    }
}
