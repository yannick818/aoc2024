
enum BlinkResult {
    Modified,
    Split(Stone),
}

struct Stone(usize);

impl Stone {
    fn blink(&mut self) -> BlinkResult {
        use BlinkResult::*;
        if self.0 == 0 {
            self.0 = 1;
            return Modified;
        }
        let mut number = self.0.to_string();
        let digit_cnt = number.len();
        if number.len() % 2 == 0 {
            let right = number.split_off(digit_cnt / 2);
            self.0 = number.parse().unwrap();
            return Split(Stone(right.parse().unwrap()));
        }
        self.0 *= 2024;
        Modified
    }
}

struct StoneLine {
    stones: Vec<Stone>,
}

impl StoneLine {
    fn parse(input: &str) -> Self {
        let mut stones = Vec::new();
        for number in input.split_whitespace() {
            stones.push(Stone(number.parse().unwrap()));
        }
        Self { stones }
    }

    fn blink(&mut self) {
        let mut idx_start = 0;
        'outer: loop {
            #[allow(clippy::mut_range_bound)]
            // we don't want to edit current iteration with idx_start
            for idx in idx_start..self.stones.len() {
                let stone = self.stones.get_mut(idx).unwrap();
                if let BlinkResult::Split(new_stone) = stone.blink() {
                    // invalidates iteration
                    self.stones.insert(idx + 1, new_stone);
                    idx_start = idx + 2;
                    continue 'outer;
                }
            }
            break;
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut stone_line = StoneLine::parse(input);
    for _blink in 0..25 {
        stone_line.blink();
    }
    stone_line.stones.len()
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
