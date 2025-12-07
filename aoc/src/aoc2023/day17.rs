use aoc::{aoc, aoc_input, heap::MinHeap};
use rustc_hash::{FxHashMap, FxHashSet};

type Map = FxHashMap<(i32, i32), usize>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    n_blocks: i32,
}

impl State {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
            n_blocks: 1,
        }
    }

    fn straight(&self) -> Self {
        Self {
            x: self.x + self.dx,
            y: self.y + self.dy,
            dx: self.dx,
            dy: self.dy,
            n_blocks: self.n_blocks + 1,
        }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x + self.dy,
            y: self.y - self.dx,
            dx: self.dy,
            dy: -self.dx,
            n_blocks: 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x - self.dy,
            y: self.y + self.dx,
            dx: -self.dy,
            dy: self.dx,
            n_blocks: 1,
        }
    }
}

fn parse(data: &str) -> Map {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| ((x, y), ch)))
        .map(|((x, y), ch)| ((x as i32, y as i32), ch.to_digit(10).unwrap() as usize))
        .collect::<Map>()
}

fn minimize_heat_loss(map: &Map, end: (i32, i32)) -> Option<usize> {
    // start one block outside
    let start = State {
        x: -1,
        y: 0,
        dx: 1,
        dy: 0,
        n_blocks: -1,
    };
    let init_cost = *map.get(&(0, 0)).unwrap();
    let mut queue = MinHeap::from([(0, start)]);
    let mut visited = FxHashSet::default(); // from_iter([start]);

    while let Some((cost, state)) = queue.pop() {
        // println!("{} - {:?}", cost, state);
        // end condition
        if (state.x, state.y) == end {
            return Some(cost - init_cost);
        }

        // already visited
        if !visited.insert(state) {
            continue; // return Some(cost);
        }

        // move straight
        if state.n_blocks < 3 {
            let next = state.straight();
            if let Some(&cost_increase) = map.get(&(next.x, next.y)) {
                queue.push(cost + cost_increase, next);
            }
        }

        // turn left
        let next = state.left();
        if let Some(&cost_increase) = map.get(&(next.x, next.y)) {
            queue.push(cost + cost_increase, next);
        }

        // turn right
        let next = state.right();
        if let Some(&cost_increase) = map.get(&(next.x, next.y)) {
            queue.push(cost + cost_increase, next);
        }
    }

    None
}

fn minimize_ultra(map: &Map, end: (i32, i32)) -> Option<usize> {
    // start one block outside
    let start = State {
        x: -1,
        y: 0,
        dx: 1,
        dy: 0,
        n_blocks: -1,
    };
    let init_cost = *map.get(&(0, 0)).unwrap();
    let mut queue = MinHeap::from([(0, start)]);
    let mut visited = FxHashSet::default(); // from_iter([start]);

    while let Some((cost, state)) = queue.pop() {
        // println!("{} - {:?}", cost, state);
        // end condition
        if (state.x, state.y) == end && state.n_blocks >= 4 {
            return Some(cost - init_cost);
        }

        // already visited
        if !visited.insert(state) {
            continue; // return Some(cost);
        }

        // move straight
        if state.n_blocks < 10 {
            let next = state.straight();
            if let Some(&cost_increase) = map.get(&(next.x, next.y)) {
                queue.push(cost + cost_increase, next);
            }
        }

        // turn left
        if state.n_blocks >= 4 {
            let next = state.left();
            if let Some(&cost_increase) = map.get(&(next.x, next.y)) {
                queue.push(cost + cost_increase, next);
            }
        }

        // turn right
        if state.n_blocks >= 4 {
            let next = state.right();
            if let Some(&cost_increase) = map.get(&(next.x, next.y)) {
                queue.push(cost + cost_increase, next);
            }
        }
    }

    None
}

#[aoc(2023, 17)]
pub fn main() {
    let data = aoc_input!(2023, 17).unwrap();
    let map = parse(&data);
    let end = map.keys().max().unwrap();

    // Part I
    let res = minimize_heat_loss(&map, *end);
    println!("{:?}", res.unwrap());

    // Part II
    let res = minimize_ultra(&map, *end);
    println!("{:?}", res.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_state() {
        let start = State::new(-1, 0, 1, 0);
        println!("start: {:?}", start);
        let straight = start.straight();
        println!("straight: {:?}", straight);

        assert_eq!(straight.x, 0);
        assert_eq!(straight.y, 0);
        assert_eq!(straight.dx, 1);
        assert_eq!(straight.dy, 0);

        let down = straight.right();
        println!("down: {:?}", down);

        assert_eq!(down.x, 0);
        assert_eq!(down.y, 1);
        assert_eq!(down.dx, 0);
        assert_eq!(down.dy, 1);

        let end = down.left();
        println!("end: {:?}", end);

        assert_eq!(end.x, 1);
        assert_eq!(end.y, 1);
        assert_eq!(end.dx, 1);
        assert_eq!(end.dy, 0);
    }

    #[test]
    fn test_part1() {
        let map = parse(EXAMPLE);
        let end = map.keys().max().unwrap();
        let res = minimize_heat_loss(&map, *end);
        println!("{:?}", res);
        assert_eq!(res, Some(102));
    }
}
