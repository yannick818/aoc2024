struct Equation {
    result: usize,
    args: Vec<usize>,
}

impl Equation {
    fn parse(input: &str) -> Vec<Equation> {
        let mut equations = Vec::new();
        for line in input.lines() {
            let (result, args) = line.split_once(": ").unwrap();
            let result = result.parse().unwrap();
            let args = args
                .split_whitespace()
                .map(|arg| arg.parse().unwrap())
                .collect();
            equations.push(Equation { result, args });
        }
        equations
    }

    fn possible_true(&self, concat: bool) -> bool {
        if self.args.len() == 1 {
            return self.result == self.args[0];
        }
        let mut args = self.args.clone();
        let arg = args.pop().unwrap();
        let args = args;

        if self.result % arg == 0 {
            let result = self.result / arg;
            let eq = Equation {
                result,
                args: args.clone(),
            };
            if eq.possible_true(concat) {
                return true;
            }
        }
        if concat {
            let mut result_str = self.result.to_string();
            let arg_str = arg.to_string();
            if result_str.ends_with(&arg_str) {
                for _ in 0..arg_str.chars().count() {
                    result_str.pop().unwrap();
                }
                if let Ok(result) = result_str.parse() {
                    let eq = Equation {
                        result,
                        args: args.clone(),
                    };
                    if eq.possible_true(concat) {
                        return true;
                    }
                }
            }
        }
        if let Some(result) = self.result.checked_sub(arg) {
            let eq = Equation { result, args };
            if eq.possible_true(concat) {
                return true;
            }
        }
        false
    }
}

pub fn part_one(input: &str) -> usize {
    let equations = Equation::parse(input);
    equations
        .into_iter()
        .filter(|eq| eq.possible_true(false))
        .map(|eq| eq.result)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let equations = Equation::parse(input);
    equations
        .into_iter()
        .filter(|eq| eq.possible_true(true))
        .map(|eq| eq.result)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, part_one(input));
        assert_eq!(11387, part_two(input));
    }

    #[test]
    fn test_concat() {
        assert_eq!(192, part_two("192: 17 8 14"))
    }
}
