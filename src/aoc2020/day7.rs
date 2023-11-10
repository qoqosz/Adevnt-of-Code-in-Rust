use aoc::aoc_input;
use hashbrown::HashMap;
use regex::Regex;

struct Bag<'a> {
    container: HashMap<&'a str, Vec<(usize, &'a str)>>,
    target_color: Option<&'a str>,
}

impl<'a> Bag<'a> {
    fn from_input(data: &'a str) -> Self {
        let re: Regex = Regex::new(r"(\d+) (.*?) bag").unwrap();

        let container: HashMap<&'a str, Vec<(usize, &'a str)>> = data
            .lines()
            .filter_map(|line| {
                let (out_color, in_color) = line.trim().split_once(" bags contain ").unwrap();
                if in_color.starts_with("no other") {
                    None
                } else {
                    let colors = re
                        .captures_iter(line)
                        .map(|d| {
                            (
                                d.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                                d.get(2).unwrap().as_str(),
                            )
                        })
                        .collect::<Vec<_>>();
                    Some((out_color, colors))
                }
            })
            .collect();

        Self {
            container,
            target_color: None,
        }
    }

    fn set_color(&mut self, color: &'a str) {
        self.target_color = Some(color);
    }

    fn contains(&self, color: &'a str) -> bool {
        if color.contains(self.target_color.unwrap()) {
            return true;
        }
        match self.container.get(color) {
            Some(inside) => inside.iter().any(|(_, c)| self.contains(c)),
            _ => false,
        }
    }

    fn count(&self) -> usize {
        self.container.keys().filter(|c| self.contains(c)).count() - 1
    }

    fn capacity(&self) -> usize {
        fn _capacity(bag: &Bag, color: &str) -> usize {
            match bag.container.get(color) {
                Some(inside) => inside.iter().map(|(n, c)| n + n * _capacity(bag, c)).sum(),
                _ => 0,
            }
        }
        _capacity(self, self.target_color.unwrap())
    }
}

fn main() {
    let data = aoc_input!(2020, 7).unwrap();
    let mut bag = Bag::from_input(&data);
    bag.set_color("shiny gold");

    // Part I
    println!("{}", bag.count());

    // Part II
    println!("{}", bag.capacity());
}
