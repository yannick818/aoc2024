#[derive(Clone, Copy)]
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

    fn is_correct(&self, rules: &[Rule]) -> bool {
        let mut previous = Vec::new();
        let mut upcoming = self.0.clone();
        for &page in self.0.iter() {
            upcoming.remove(0);
            if !Self::is_valid_state(&previous, &upcoming, page, rules) {
                return false;
            }
            previous.push(page);
        }
        true
    }

    fn is_valid_state(prev: &[usize], queue: &[usize], page: usize, rules: &[Rule]) -> bool {
        let mut req_prev = Vec::new();
        let mut req_after = Vec::new();
        for rule in rules.iter() {
            if rule.x == page {
                req_after.push(rule.y);
            }
            if rule.y == page {
                req_prev.push(rule.x);
            }
        }
        for upcom in queue.iter() {
            if req_prev.contains(upcom) {
                return false;
            }
        }
        for prev in prev.iter() {
            if req_after.contains(prev) {
                return false;
            }
        }
        true
    }

    fn order(&self, rules: &[Rule]) -> Update {
        let relevant_rules: Vec<Rule> = rules
            .iter()
            .filter(|Rule { x, y }| self.0.contains(x) && self.0.contains(y))
            .cloned()
            .collect();

        let state = State::new(&relevant_rules, self.0.clone());
        state.try_rules().unwrap()
    }
}

struct State<'a> {
    rules: &'a [Rule],
    update: Vec<usize>,
    queue: Vec<usize>,
}

impl<'a> State<'a> {
    fn new(rules: &'a [Rule], pages: Vec<usize>) -> Self {
        Self {
            rules,
            update: Vec::new(),
            queue: pages,
        }
    }

    fn try_rules(self) -> Option<Update> {
        let mut update = self.update.clone();
        update.append(&mut self.queue.clone());
        let update = Update(update);
        if update.is_correct(self.rules) {
            return Some(update);
        }
        if self.queue.is_empty() {
            return None;
        }
        let mut new_states = Vec::new();
        for (idx, &page) in self.queue.iter().enumerate() {
            let mut queue = self.queue.clone();
            queue.remove(idx);
            let mut update = self.update.clone();
            update.push(page);

            if Update::is_valid_state(&update, &queue, page, self.rules) {
                new_states.push(State {
                    rules: self.rules,
                    update,
                    queue,
                });
            }
        }
        for state in new_states.into_iter() {
            if let Some(update) = state.try_rules() {
                return Some(update);
            }
        }
        None
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
        for update in &self.updates {
            if update.is_correct(&self.rules) {
                sum += update.middle();
            }
        }
        sum
    }

    fn count_corrected(&self) -> usize {
        let mut sum = 0;
        for update in &self.updates {
            if !update.is_correct(&self.rules) {
                sum += update.order(&self.rules).middle();
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> usize {
    let man = Manual::parse(input);
    man.count_middle_update()
}

pub fn part_two(input: &str) -> usize {
    let man = Manual::parse(input);
    man.count_corrected()
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

        assert_eq!(143, part_one(input));
        assert_eq!(123, part_two(input));
    }
}
