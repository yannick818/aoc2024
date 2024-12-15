#[derive(Clone, Copy)]
enum Block {
    Free,
    File(usize),
}

struct Disk(Vec<Block>);

impl Disk {
    fn parse(input: &str) -> Self {
        let mut disk = Vec::new();
        let mut is_block = true;
        let mut file_id = 0;
        for c in input.chars() {
            let number = c.to_digit(10).unwrap();
            for _i in 0..number {
                if is_block {
                    disk.push(Block::File(file_id));
                } else {
                    disk.push(Block::Free);
                }
            }
            if is_block {
                file_id += 1;
            }
            is_block ^= true;
        }
        Disk(disk)
    }

    fn defragment(&mut self) {
        let mut idx_start = 0;
        for (idx, block) in self.0.iter().enumerate() {
            if let Block::Free = block {
                idx_start = idx;
                break;
            }
        }
        loop {
            let last = self.0.pop().unwrap();
            if let Block::Free = last {
                continue;
            }
            let mut found_free = false;
            for idx in idx_start..self.0.len() {
                let block = &mut self.0[idx];
                if let Block::Free = block {
                    *block = last;
                    found_free = true;
                    idx_start = idx + 1;
                    break;
                }
            }
            if !found_free {
                self.0.push(last);
                break;
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut disk = Disk::parse(input);
    disk.defragment();
    let mut checksum = 0;
    for (pos, block) in disk.0.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += pos * id;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        assert_eq!(1928, part_one(input));
    }
}
