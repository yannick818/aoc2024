use crate::util::position::Position;

type Height = u8;

struct Map(Vec<Vec<Height>>);

struct Hiker<'a> {
    map: &'a Map,
    pos: Position,
    height: Height,
}

enum HikeResult<'a> {
    ReachedTop,
    Deadend,
    Split(Vec<Hiker<'a>>),
}

impl<'a> Hiker<'a> {
    fn hike(self) -> HikeResult<'a> {
        todo!()
    }
}

impl Map {
    fn parse(input: &str) -> (Self, Vec<Position>) {
        todo!()
    }
}

pub fn part_one(input: &str) -> usize {
    let (map, starts) = Map::parse(input);
    let mut sum = 0;
    for start in starts.into_iter() {
        let mut score = 0;
        let hiker = Hiker {
            map: &map,
            pos: start,
            height: 0,
        };
        let mut paths = vec![hiker];
        while let Some(path) = paths.pop() {
            use HikeResult::*;
            match path.hike() {
                ReachedTop => score+=1,
                Deadend => break,
                Split(new_paths) => {
                    for new_path in new_paths.into_iter() {
                        paths.push(new_path);
                    }
                }
            }
        }
        sum += score;
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
        assert_eq!(36, part_one(input));
    }
}
