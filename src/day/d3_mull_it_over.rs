use Instruction::*;
enum Instruction {
    Do,
    Dont,
    Mul(isize),
}

impl Instruction {
    fn parse(input: &str) -> Vec<Self> {
        let mut instrutions = Vec::new();
        for (pos, _do) in input.match_indices("do()") {
            instrutions.push((pos, Do));
        }
        for (pos, _dont) in input.match_indices("don't()") {
            instrutions.push((pos, Dont));
        }
        for (pos, _mul) in input.match_indices("mul(") {
            let ltrim = &input[pos + 4..];
            let trimmed = match ltrim.find(")") {
                None => continue,
                Some(i) => ltrim.get(..i).unwrap(),
            };

            if let Some(_hit) = trimmed.find(|c: char| !(c.is_numeric() || c == ',')) {
                //invalid character
                continue;
            }

            if 1 != trimmed.chars().filter(|&c| c == ',').count() {
                continue;
            }

            let (lhs, rhs) = trimmed.split_once(',').unwrap();

            if !lhs.is_valid() || !rhs.is_valid() {
                continue;
            }

            let lhs: isize = lhs.parse().unwrap();
            let rhs: isize = rhs.parse().unwrap();

            instrutions.push((pos, Mul(lhs * rhs)));
        }

        instrutions.sort_by_key(|(pos, _)| *pos);
        instrutions.into_iter().map(|(_, instr)| instr).collect()
    }
}

pub fn multiply_numbers(input: &str) -> isize {
    let instuctions = Instruction::parse(input);
    let mut sum = 0;
    for instruction in instuctions.into_iter() {
        if let Mul(mul) = instruction {
            sum += mul;
        }
    }
    sum
}

pub fn multiply_numbers_filtered(input: &str) -> isize {
    let instuctions = Instruction::parse(input);
    let mut sum = 0;
    let mut mul_enabled = true;
    for instruction in instuctions.into_iter() {
        match (instruction, mul_enabled) {
            (Do, _) => {
                mul_enabled = true;
            }
            (Dont, _) => {
                mul_enabled = false;
            }
            (Mul(mul), true) => {
                sum += mul;
            }
            _ => {}
        }
    }
    sum
}

trait ValidNumber {
    fn is_valid(&self) -> bool;
}

impl ValidNumber for str {
    fn is_valid(&self) -> bool {
        !self.is_empty() && self.len() <= 3
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiply() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, multiply_numbers(input));
    }

    #[test]
    fn test_multiply_filter() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, multiply_numbers_filtered(input), "3.2 failed");
    }

    #[test]
    fn test_split() {
        let input = "mul(123)";
        let mut split = input.split("mul(");
        assert_eq!(Some(""), split.next());
        assert_eq!(Some("123)"), split.next());
        assert_eq!(None, split.next());
    }
}
