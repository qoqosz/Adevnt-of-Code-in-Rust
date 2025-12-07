use aoc::{aoc, aoc_input};

#[derive(Default)]
struct Dir<'a> {
    name: &'a str,
    parent: Option<usize>,
    files: usize,
    dirs: Vec<usize>,
}

#[derive(Default)]
struct Disk<'a> {
    dirs: Vec<Dir<'a>>,
}

impl<'a> Disk<'a> {
    fn create_dir(&mut self, parent: usize, name: &'a str) {
        self.dirs.push(Dir {
            name,
            parent: Some(parent),
            files: 0,
            dirs: vec![],
        });
        let id = self.dirs.len() - 1;
        self.dirs[parent].dirs.push(id);
    }

    fn size(&self, i: usize, cache: &mut [Option<usize>]) -> usize {
        if let Some(sz) = cache[i] {
            return sz;
        }
        let sz = self.dirs[i].files
            + self.dirs[i]
                .dirs
                .iter()
                .map(|j| self.size(*j, cache))
                .sum::<usize>();
        cache[i] = Some(sz);
        sz
    }
}

fn parse(data: &str) -> Disk<'_> {
    let mut disk = Disk::default();
    let root = Dir::default();
    disk.dirs.push(root);
    let mut cwd = 0;

    for cmd in data.trim().lines().skip(1) {
        if cmd.starts_with("$ cd") {
            let name = &cmd[5..];

            if name == ".." {
                cwd = disk.dirs[cwd].parent.unwrap();
            } else {
                cwd = *disk.dirs[cwd]
                    .dirs
                    .iter()
                    .find(|&&i| disk.dirs[i].name == name)
                    .unwrap();
            }
        } else if cmd.starts_with("$ ls") {
            continue;
        } else if cmd.starts_with("dir") {
            disk.create_dir(cwd, &cmd[4..]);
        } else {
            let (size, _) = cmd.split_once(' ').unwrap();
            disk.dirs[cwd].files += size.parse::<usize>().unwrap();
        }
    }

    disk
}

#[aoc(2022, 7)]
pub fn main() {
    let data = aoc_input!(2022, 7).unwrap();
    let disk = parse(&data);

    // Part I
    let mut cache = [None; 256];
    let total_size = (0..disk.dirs.len())
        .map(|i| disk.size(i, &mut cache))
        .filter(|&size| size <= 100_000)
        .sum::<usize>();

    println!("{}", total_size);

    // Part II
    let total_disk_space = 70_000_000;
    let required_space = 30_000_000;
    let free_space = total_disk_space - cache[0].unwrap();
    let to_be_freed = required_space - free_space;
    let smallest_dir_size = cache
        .iter()
        .flatten()
        .filter(|&&x| x >= to_be_freed)
        .min()
        .unwrap();

    println!("{}", smallest_dir_size);
}
