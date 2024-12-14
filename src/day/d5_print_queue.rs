use std::collections::VecDeque;

struct Rule {
    x: usize,
    y: usize,
}

struct Update(Vec<usize>);

impl Update {
    fn middle(&self) -> usize {
        assert_eq!(self.0.len() % 2, 1);
        let middle = (self.0.len() - 1) / 2;
        *self.0.get(middle).unwrap()
    }
}

struct Manual {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Manual {
    fn parse(input: &str) -> Manual {
        let (in_rules, in_updates) = input.split_once("\n\n").unwrap();
        let mut rules = Vec::new();
        for rule in in_rules.lines() {
            let (x, y) = rule.split_once('|').unwrap();
            rules.push(Rule {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }
        let mut updates = Vec::new();
        for update in in_updates.lines() {
            let mut pages = Vec::new();
            for page in update.split(',') {
                pages.push(page.parse().unwrap());
            }
            updates.push(Update(pages));
        }
        Self { rules, updates }
    }

    fn count_middle_update(&self) -> usize {
        let mut sum = 0;
        'update: for update in &self.updates {
            let mut previous = Vec::new();
            let mut upcoming: VecDeque<_> = update.0.clone().into();
            for &page in update.0.iter() {
                upcoming.pop_front();
                let mut req_prev = Vec::new();
                let mut req_after = Vec::new();
                for rule in self.rules.iter() {
                    if rule.x == page {
                        req_after.push(rule.y);
                    }
                    if rule.y == page {
                        req_prev.push(rule.x);
                    }
                }
                for upcom in upcoming.iter() {
                    if req_prev.contains(upcom) {
                        continue 'update;
                    }
                }
                for prev in previous.iter() {
                    if req_after.contains(prev) {
                        continue 'update;
                    }
                }
                previous.push(page);
            }
            sum += update.middle();
        }
        sum
    }
}

pub fn count_update(input: &str) -> usize {
    let man = Manual::parse(input);
    man.count_middle_update()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_update() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(143, count_update(input));
    }
}
