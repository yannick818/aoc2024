use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Stone(usize);

type Cache = HashMap<(Stone, usize), usize>;

impl Stone {
    fn blink(mut self) -> Vec<Stone> {
        if self.0 == 0 {
            self.0 = 1;
            return vec![self];
        }
        let mut number = self.0.to_string();
        let digit_cnt = number.len();
        if number.len() % 2 == 0 {
            let right = number.split_off(digit_cnt / 2);
            self.0 = number.parse().unwrap();
            return vec![self, Stone(right.parse().unwrap())];
        }
        self.0 *= 2024;
        vec![self]
    }

    fn count(self, blinks: usize, cache: &mut Cache) -> usize {
        if blinks == 0 {
            return 1;
        }
        if let Some(count) = cache.get(&(self, blinks)) {
            return *count;
        }
        let mut sum = 0;
        for stone in self.blink() {
            sum += stone.count(blinks - 1, cache);
        }
        cache.insert((self, blinks), sum);
        sum
    }
}

struct StoneLine {
    stones: Vec<Stone>,
    cache: Cache,
}

impl StoneLine {
    fn parse(input: &str) -> Self {
        let mut stones = Vec::new();
        for number in input.split_whitespace() {
            stones.push(Stone(number.parse().unwrap()));
        }
        Self {
            stones,
            cache: HashMap::new(),
        }
    }

    fn blink(&mut self, cnt: usize) -> usize {
        let mut sum = 0;
        for stone in self.stones.iter() {
            sum += stone.count(cnt, &mut self.cache);
        }
        sum
    }
}

pub fn part_one(input: &str) -> usize {
    let mut stone_line = StoneLine::parse(input);
    stone_line.blink(25)
}

pub fn part_two(input: &str) -> usize {
    let mut stone_line = StoneLine::parse(input);
    stone_line.blink(75)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "125 17";
        assert_eq!(55312, part_one(input));
    }
}
