

struct Report(Vec<usize>);

struct Reports(Vec<Report>);

impl Reports {
    fn parse(input: &str) -> Self {
        let mut reports = Vec::new();
        for line in input.lines() {
            let mut levels = Vec::new();
            for level in line.split_whitespace().map(|level| level.parse().unwrap()) {
                levels.push(level);
            }
            reports.push(Report(levels));
        }
        Reports(reports)
    }

    fn count_safe(&self) -> usize {
        self.0.iter().filter(|report| report.is_safe()).count()
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut levels = self.0.iter();
        let mut last = levels.next().cloned().unwrap() as isize;
        let mut incr_state = None;
        for &level in levels {
            let diff = level as isize - last;
            if diff.abs() > 3 || diff == 0 {
                return false;
            }
            match (incr_state, diff.is_positive()) {
                (Some(prev_incr), incr) if prev_incr == incr => {},
                (None, is_incr) => {
                    incr_state = Some(is_incr);
                }
                _ => {
                    return false;
                }
            }
            last = level as isize;
        }
        true
    }
}

pub fn count_safe(input: &str) -> usize {
    let reports = Reports::parse(input);
    reports.count_safe()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(count_safe(input), 2);
    }
}

