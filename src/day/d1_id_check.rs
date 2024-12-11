struct IdList {
    list_a: Vec<usize>,
    list_b: Vec<usize>,
}

impl IdList {
    fn parse(input: &str) -> Self {
        let mut list_a = Vec::new();
        let mut list_b = Vec::new();
        for line in input.lines() {
            let mut iter = line
                .split_whitespace()
                .map(|number| number.parse().unwrap());
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            list_a.push(a);
            list_b.push(b);
        }
        IdList { list_a, list_b }
    }

    fn distance(&mut self) -> usize {
        self.list_a.sort();
        self.list_b.sort();

        let mut sum = 0;
        for (&a, &b) in self.list_a.iter().zip(self.list_b.iter()) {
            let diff = a.abs_diff(b);
            sum += diff;
        }
        sum
    }

    fn similarity(&mut self) -> usize {
        let mut sum = 0;
        for value in self.list_a.iter() {
            let count = self.list_b.iter().filter(|v| value.eq(v)).count();
            sum += value * count;
        }
        sum
    }
}

pub fn cal_distance(input: &str) -> usize {
    let mut id_list = IdList::parse(input);
    id_list.distance()
}

pub fn cal_similarity(input: &str) -> usize {
    let mut id_list = IdList::parse(input);
    id_list.similarity()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(cal_distance(input), 11, "1.1 failed");
        assert_eq!(cal_similarity(input), 31, "1.2 failed");
    }
}
