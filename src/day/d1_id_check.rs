struct IdList {
    list_a: Vec<usize>,
    list_b: Vec<usize>,
}

impl IdList {
    fn parse(input: String) -> Self {
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
}

pub fn cal_distance(input: String) -> usize {
    let mut id_list = IdList::parse(input);
    id_list.distance()
}

#[cfg(test)]
mod tests {
    use crate::d1_id_check::cal_distance;

    #[test]
    fn test_check_id() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(cal_distance(input.into()), 11);
    }
}
