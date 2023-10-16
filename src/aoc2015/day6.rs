use aoc::load_input;

#[derive(PartialEq, Debug)]
enum Action {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(PartialEq, Debug)]
struct Range {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Range {
    fn new(x0: usize, y0: usize, x1: usize, y1: usize) -> Self {
        assert!(x0 <= x1);
        assert!(y0 <= y1);

        Range {
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
        }
    }

    fn from_vec(v: Vec<usize>) -> Self {
        assert!(v.len() >= 4);
        Range::new(v[0], v[1], v[2], v[3])
    }

    fn points(&self) -> Vec<(usize, usize)> {
        let mut container = Vec::with_capacity((self.x1 - self.x0) * (self.y1 - self.y0));

        for i in self.x0..=self.x1 {
            for j in self.y0..=self.y1 {
                container.push((i, j))
            }
        }
        container
    }
}

struct Grid {
    size_x: usize,
    // size_y: usize,
    grid: Vec<u32>,
}

impl Grid {
    fn new(size_x: usize, size_y: usize) -> Self {
        let capacity = (size_x + 1) * (size_y + 1);
        Grid {
            size_x: size_x,
            // size_y: size_y,
            grid: vec![0; capacity],
        }
    }

    fn count_on(&self) -> usize {
        self.grid.iter().filter(|&x| *x != 0).count()
    }

    fn brightness(&self) -> usize {
        self.grid.iter().sum::<u32>() as usize
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x + self.size_x * y
    }

    fn act_single(&mut self, action: &Action, x: usize, y: usize, f: &dyn Fn(&Action, u32) -> u32) {
        let idx = self.idx(x, y);
        let state = *self.grid.get(idx).unwrap();
        let state = f(action, state);
        self.grid[idx] = state;
    }

    fn act_range(&mut self, action: &Action, range: &Range, f: &dyn Fn(&Action, u32) -> u32) {
        for (x, y) in range.points() {
            self.act_single(action, x, y, f);
        }
    }
}

fn parse_line(line: &str) -> (Action, Range) {
    let action = {
        if line.starts_with("toggle") {
            Action::Toggle
        } else if line.starts_with("turn off") {
            Action::TurnOff
        } else {
            Action::TurnOn
        }
    };

    let parts = line
        .split_whitespace()
        .filter(|x| x.chars().nth(0).unwrap().is_digit(10))
        .map(|x| x.split(','))
        .flatten()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let range = Range::from_vec(parts);

    (action, range)
}

fn main() {
    let data = load_input!("/Users/qoqosz/Documents/Coding/Rust/Advent of Code/data/2015/day6.txt");
    let commands = data
        .split('\n')
        .filter(|&x| !x.is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();

    // Part I
    let mut grid = Grid::new(1000, 1000);
    let f = |action: &Action, state: u32| -> u32 {
        match action {
            Action::Toggle => (state + 1) % 2,
            Action::TurnOff => 0,
            Action::TurnOn => 1,
        }
    };

    for (action, range) in &commands {
        grid.act_range(action, range, &f);
    }

    println!("{}", grid.count_on());

    // Part II
    let mut grid = Grid::new(1000, 1000);
    let f = |action: &Action, state: u32| -> u32 {
        match action {
            Action::Toggle => state + 2,
            Action::TurnOff => state.saturating_sub(1),
            Action::TurnOn => state + 1,
        }
    };

    for (action, range) in &commands {
        grid.act_range(action, range, &f);
    }

    println!("{}", grid.brightness());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let range = Range::new(1, 1, 2, 1);
        let points = vec![(1, 1), (2, 1)];
        assert_eq!(points, range.points());
    }

    #[test]
    fn test_case_2() {
        let line: &str = "toggle 461,550 through 564,900";
        let result = parse_line(&line);

        assert_eq!(result.0, Action::Toggle);
        assert_eq!(result.1, Range::new(461, 550, 564, 900));
    }
}
