use aoc::{aoc, aoc_input};
use glam::IVec2;
use std::num::ParseIntError;

static ROTATIONS: &[IVec2] = &[
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
    IVec2::new(0, -1),
];

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl TryFrom<&str> for Action {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, val) = value.split_at(1);
        let val = val.parse::<i32>()?;

        Ok(match dir {
            "N" => Action::North(val),
            "S" => Action::South(val),
            "E" => Action::East(val),
            "W" => Action::West(val),
            "L" => Action::Left(val),
            "R" => Action::Right(val),
            "F" => Action::Forward(val),
            _ => unimplemented!(),
        })
    }
}

impl Action {
    fn to_ivec2(&self) -> IVec2 {
        match *self {
            Action::North(val) => IVec2::new(0, val),
            Action::South(val) => IVec2::new(0, -val),
            Action::East(val) => IVec2::new(val, 0),
            Action::West(val) => IVec2::new(-val, 0),
            Action::Left(val) => {
                let rot = (val / 90) % 4;
                ROTATIONS[rot as usize]
            }
            Action::Right(val) => {
                let rot = (4 - (val / 90)) % 4;
                ROTATIONS[rot as usize]
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Ship {
    pos: IVec2,
    dir: IVec2,
    waypoint: Option<IVec2>,
}

impl Ship {
    fn new() -> Self {
        Self {
            pos: IVec2::new(0, 0),
            dir: IVec2::new(1, 0),
            waypoint: None,
        }
    }

    fn with_waypoint() -> Self {
        Self {
            waypoint: Some(IVec2::new(10, 1)),
            ..Self::new()
        }
    }

    fn dist(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs()
    }

    fn navigate(&mut self, action: &Action) {
        if let Some(waypoint) = self.waypoint {
            match action {
                Action::North(_) | Action::South(_) | Action::East(_) | Action::West(_) => {
                    self.waypoint = Some(waypoint + action.to_ivec2())
                }
                Action::Left(_) | Action::Right(_) => {
                    self.waypoint = Some(waypoint.rotate(action.to_ivec2()))
                }
                Action::Forward(val) => self.pos += *val * waypoint,
            }
        } else {
            match action {
                Action::North(_) | Action::South(_) | Action::East(_) | Action::West(_) => {
                    self.pos += action.to_ivec2()
                }
                Action::Left(_) | Action::Right(_) => self.dir = self.dir.rotate(action.to_ivec2()),
                Action::Forward(val) => self.pos += self.dir * *val,
            }
        }
    }
}

fn parse(data: &str) -> Vec<Action> {
    data.trim().lines().flat_map(Action::try_from).collect()
}

#[aoc(2020, 12)]
pub fn main() {
    let data = aoc_input!(2020, 12).unwrap();
    let actions = parse(&data);

    // Part I
    let mut ship = Ship::new();
    actions.iter().for_each(|action| ship.navigate(action));
    println!("{}", ship.dist());

    // Part II
    let mut ship = Ship::with_waypoint();
    actions.iter().for_each(|action| ship.navigate(action));
    println!("{}", ship.dist());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        let actions = parse(EXAMPLE);
        let mut ship = Ship::new();
        actions.iter().for_each(|action| ship.navigate(action));
        assert_eq!(ship.dist(), 25);
    }

    #[test]
    fn test_part2() {
        let actions = parse(EXAMPLE);
        let mut ship = Ship::with_waypoint();
        actions.iter().for_each(|action| ship.navigate(action));
        assert_eq!(ship.dist(), 286);
    }
}
