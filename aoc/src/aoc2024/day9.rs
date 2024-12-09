use aoc::{aoc, aoc_input};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct DiskMap {
    blocks: Vec<Option<usize>>,
    file_blocks: Vec<(usize, usize)>,
    free_blocks: Vec<(usize, usize)>,
}

impl FromStr for DiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = vec![];
        let mut file_blocks = vec![];
        let mut free_blocks = vec![];

        for (i, elem) in s.trim().as_bytes().iter().enumerate() {
            let block_len = *elem as usize - 48;
            if i % 2 == 0 {
                file_blocks.push((blocks.len(), block_len));
                blocks.extend(vec![Some(i / 2); block_len]);
            } else {
                free_blocks.push((blocks.len(), block_len));
                blocks.extend(vec![None; block_len]);
            }
        }

        Ok(Self {
            blocks,
            file_blocks,
            free_blocks,
        })
    }
}

impl DiskMap {
    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, val)| i * (val.unwrap_or(0) as usize))
            .sum::<usize>()
    }

    // Part I
    fn defragment(&mut self) {
        let mut file_positions = self
            .file_blocks
            .iter()
            .rev()
            .flat_map(|(start, len)| (*start..(*start + *len)).rev());

        let (mut i_block, mut i_file) = (0, file_positions.next().unwrap());

        while i_block <= i_file {
            if self.blocks[i_block].is_none() {
                self.blocks.swap(i_block, i_file);

                match file_positions.next() {
                    Some(idx) => i_file = idx,
                    None => break,
                }
            }
            i_block += 1;
        }
    }

    // Part II
    fn move_files(&mut self) {
        for &(file_start, file_len) in self.file_blocks.iter().rev() {
            for i_free in 0..self.free_blocks.len() {
                let (free_start, free_len) = self.free_blocks[i_free];

                if free_start > file_start {
                    break;
                }

                if free_len >= file_len {
                    for i in 0..file_len {
                        self.blocks.swap(free_start + i, file_start + i);
                    }
                    if file_len < free_len {
                        self.free_blocks[i_free] = (free_start + file_len, free_len - file_len);
                    } else {
                        self.free_blocks.remove(i_free);
                    }
                    break;
                }
            }
        }
    }
}

#[aoc(2024, 9)]
pub fn main() {
    let data = aoc_input!(2024, 9).unwrap();
    let mut disk = DiskMap::from_str(&data).unwrap();

    // Part I
    let mut disk_ = disk.clone();
    disk_.defragment();
    println!("{}", disk_.checksum());

    // Part II
    disk.move_files();
    println!("{}", disk.checksum());
}
