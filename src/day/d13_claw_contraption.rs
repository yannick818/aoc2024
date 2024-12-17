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
    */
    fn solve(&self) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        for b in 0..=100 {
            let sub = match self.x.checked_sub(self.x_b * b) {
                Some(sub) => sub,
                None => continue,
            };
            if sub % self.x_a != 0 {
                continue;
            }
            let a = sub / self.x_a;
            if a > 100 {
                continue;
            }
            if self.y_a * a + self.y_b * b == self.y {
                results.push((a, b));
            }
        }
        results
    }
}

fn parse(input: &str) -> Vec<ClawMachine> {
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
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
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
    let machines = parse(input);
    let mut prize = 0;
    for machine in machines {
        let mut results: Vec<_> = machine
            .solve()
            .into_iter()
            .map(|(a, b)| (a, b, 3 * a + b))
            .collect();
        results.sort_by_key(|(_,_,prize)| *prize);
        if let Some(cheapest) = results.first() {
            prize += cheapest.2;
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
