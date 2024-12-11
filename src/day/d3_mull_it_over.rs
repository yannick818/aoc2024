pub fn multiply_numbers(input: &str) -> isize {
    let mut sum = 0;
    for input in input.split("mul(").skip(1) {
        let trimmed = match input.find(")") {
            None => continue,
            Some(i) => input.get(..i).unwrap(),
        };
        //check for invalid characters
        if let Some(_hit) = trimmed.find(|c: char| !(c.is_numeric() || c == ',')) {
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

        sum += lhs * rhs;
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
        assert_eq!(161, multiply_numbers(input))
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
