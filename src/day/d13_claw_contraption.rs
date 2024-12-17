struct ClawMachine {
    x: usize,
    y: usize,
    x_a: usize,
    x_b: usize,
    y_a: usize,
    y_b: usize,
}

impl ClawMachine {
    /*
    x = xa * a + xb * b
    y = ya * a + yb * b

    a = (x - xb * b) / xa
    b = (xa*y-ya*x)/(yb*xa-ya*xb)
    */
    fn solve(&self) -> Option<(usize, usize)> {
        let divisor = (self.y_b * self.x_a) as isize - (self.y_a * self.x_b) as isize;
        match divisor {
            0 => {
                unreachable!("both buttons move in same direction")
            }
            divisor => {
                let dividend = (self.x_a * self.y) as isize - (self.y_a * self.x) as isize;
                if dividend % divisor != 0 {
                    return None;
                }
                let b = dividend / divisor;
                if b < 0 {
                    return None;
                }
                let b = b as usize;
                let div = match self.x.checked_sub(self.x_b * b) {
                    Some(div) => div,
                    None => return None,
                };
                if div % self.x_a != 0 {
                    return None;
                }
                let a = div / self.x_a;
                Some((a, b))
            }
        }
    }
}

fn parse(input: &str, add: usize) -> Vec<ClawMachine> {
    let mut machines = Vec::new();
    for machine in input.split("\n\n") {
        let mut iter = machine.lines();
        const BUTTON: usize = 12;
        const PRIZE: usize = 9;
        let a = &iter.next().unwrap()[BUTTON..];
        let b = &iter.next().unwrap()[BUTTON..];
        let p = &iter.next().unwrap()[PRIZE..];
        let (x_a, y_a) = a.split_once(", ").unwrap();
        let y_a = &y_a[2..];
        let (x_b, y_b) = b.split_once(", ").unwrap();
        let y_b = &y_b[2..];
        let (x, y) = p.split_once(", ").unwrap();
        let y = &y[2..];
        let machine = ClawMachine {
            x: x.parse::<usize>().unwrap() + add,
            y: y.parse::<usize>().unwrap() + add,
            y_a: y_a.parse().unwrap(),
            y_b: y_b.parse().unwrap(),
            x_a: x_a.parse().unwrap(),
            x_b: x_b.parse().unwrap(),
        };
        machines.push(machine);
    }
    machines
}

pub fn part_one(input: &str) -> usize {
    let machines = parse(input, 0);
    let mut prize = 0;
    for machine in machines {
        if let Some((a, b)) = machine.solve() {
            prize += 3 * a + b;
        }
    }
    prize
}

pub fn part_two(input: &str) -> usize {
    let machines = parse(input, 10000000000000);
    let mut prize = 0;
    for machine in machines {
        if let Some((a, b)) = machine.solve() {
            prize += 3 * a + b;
        }
    }
    prize
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(480, part_one(input));
    }
}
