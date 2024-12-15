use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Block {
    typ: BlockType,
    len: usize,
}

#[derive(Clone, Copy, Debug)]
enum BlockType {
    Free,
    File(usize),
}

struct Disk(Vec<BlockType>);

struct BlockDisk {
    disk: Vec<Block>,
    files: usize,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let mut disk = Vec::new();
        let mut is_block = true;
        let mut file_id = 0;
        for c in input.chars() {
            let number = c.to_digit(10).unwrap();
            for _i in 0..number {
                if is_block {
                    disk.push(BlockType::File(file_id));
                } else {
                    disk.push(BlockType::Free);
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
            if let BlockType::Free = block {
                idx_start = idx;
                break;
            }
        }
        loop {
            let last = self.0.pop().unwrap();
            if let BlockType::Free = last {
                continue;
            }
            let mut found_free = false;
            for idx in idx_start..self.0.len() {
                let block = &mut self.0[idx];
                if let BlockType::Free = block {
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

impl BlockDisk {
    fn parse(input: &str) -> Self {
        let mut disk = Vec::new();
        let mut is_block = true;
        let mut file_id = 0;
        for c in input.chars() {
            let len = c.to_digit(10).unwrap() as usize;
            if len > 0 {
                let typ = match is_block {
                    true => BlockType::File(file_id),
                    false => BlockType::Free,
                };
                let block = Block { typ, len };
                disk.push(block);
                if is_block {
                    file_id += 1;
                }
            }
            is_block ^= true;
        }
        BlockDisk {
            disk,
            files: file_id,
        }
    }

    fn defragment(&mut self) {
        for id in (0..self.files).rev() {
            //println!("Move {id}");
            //println!("{}", self);
            let (idx_from, &block_from) = self
                .disk
                .iter()
                .enumerate()
                .find(|(_i, block)| matches!(block.typ, BlockType::File(i) if id == i))
                .unwrap();
            for idx in 0..idx_from {
                let block_dest = &mut self.disk[idx];
                match (block_dest.typ, block_dest.len) {
                    (BlockType::Free, space) if space == block_from.len => {
                        *block_dest = block_from;
                        self.disk[idx_from].typ = BlockType::Free;
                        break;
                    }
                    (BlockType::Free, space) if space > block_from.len => {
                        *block_dest = block_from;
                        self.disk[idx_from].typ = BlockType::Free;
                        let free_block = Block {
                            typ: BlockType::Free,
                            len: space - block_from.len,
                        };
                        self.disk.insert(idx + 1, free_block);
                        break;
                    }
                    (_, _) => {}
                }
            }
        }
    }
}

impl Display for BlockDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.disk.iter() {
            f.write_fmt(format_args!("{:?}\n", block))?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> usize {
    let mut disk = Disk::parse(input);
    disk.defragment();
    let mut checksum = 0;
    for (pos, block) in disk.0.iter().enumerate() {
        if let BlockType::File(id) = block {
            checksum += pos * id;
        }
    }
    checksum
}

pub fn part_two(input: &str) -> usize {
    let mut disk = BlockDisk::parse(input);
    disk.defragment();
    let mut checksum = 0;
    let mut pos = 0;
    for block in disk.disk.iter() {
        for _ in 0..block.len {
            if let BlockType::File(id) = block.typ {
                checksum += pos * id;
            }
            pos += 1;
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
        assert_eq!(2858, part_two(input));
    }
}
