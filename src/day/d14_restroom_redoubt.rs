use std::fmt::Write;

struct Robot {
    pos: (isize, isize),
    speed: (isize, isize),
}

struct Space {
    wide: isize,
    tall: isize,
    robots: Vec<Robot>,
}

impl Space {
    fn parse(input: &str, wide: isize, tall: isize) -> Self {
        let mut robots = Vec::new();
        for line in input.lines() {
            let (pos, velo) = line.split_once(' ').unwrap();
            let (x, y) = pos[2..].split_once(',').unwrap();
            let (vx, vy) = velo[2..].split_once(',').unwrap();
            let robot = Robot {
                pos: (x.parse().unwrap(), y.parse().unwrap()),
                speed: (vx.parse().unwrap(), vy.parse().unwrap()),
            };
            robots.push(robot);
        }
        Self { wide, tall, robots}
    }

    fn move_robots(&mut self, seconds: isize) {
        for robot in self.robots.iter_mut() {
            let x = robot.pos.0 + seconds * robot.speed.0;
            let y = robot.pos.1 + seconds * robot.speed.1;
            robot.pos.0 = match x % (self.wide) {
                x if x.is_negative() => x + self.wide,
                x => x,
            };
            robot.pos.1 = match y % (self.tall) {
                y if y.is_negative() => y + self.tall,
                y => y,
            }
        }
    }

    fn safety_factor(&self) -> usize {
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        let half_wide = (self.wide - 1) / 2;
        let half_tall = (self.tall - 1) / 2;
        for robot in self.robots.iter() {
            match robot.pos {
                (x, y) if x == half_wide || y == half_tall => {}
                (x, y) if x < half_wide && y < half_tall => q1 += 1,
                (x, y) if x < half_wide => q2 += 1,
                (x, y) if y < half_tall => q3 += 1,
                (x, y) => q4 += 1,
            }
        }
        q1 * q2 * q3 * q4
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut space = vec![vec![0; self.wide as usize]; self.tall as usize];
        for robot in self.robots.iter() {
            let tile = &mut space[robot.pos.1 as usize][robot.pos.0 as usize];
            *tile += 1;
        }
        for line in space.into_iter() {
            for num in line {
                f.write_str(&num.to_string())?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

pub fn part_one(input: &str) -> usize {
    let mut space = Space::parse(input, 101, 103);
    space.move_robots(100);
    space.safety_factor()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let mut space = Space::parse(input, 11, 7);
        println!("{}", space);
        space.move_robots(100);
        println!("{}", space);
        assert_eq!(12, space.safety_factor());
    }

    #[test]
    fn test_example_small() {
        let input = "p=2,4 v=2,-3";
        let mut space = Space::parse(input, 11, 7);
        println!("{}", space);
        space.move_robots(4);
        println!("{}", space);
        //for i in 0..=5 {
        //    println!("{i}:\n{}", space);
        //    space.move_robots(1);
        //}
    }

    #[test]
    fn test_step() {
        let robot = Robot {
            pos: (0,0),
            speed: (-1, -1),
        };
        let mut space = Space {
            wide: 7,
            tall: 5,
            robots: vec![robot],
        };
        println!("{}", space);
        space.move_robots(1);
        println!("{}", space);
    }

    #[test]
    fn test_mod() {
        assert_eq!(8, -12 % 10 + 10)
    }
}
