use aoc::{aoc, aoc_input};
use glam::IVec2;
use regex_lite::Regex;
use rustc_hash::FxHashMap;

static DIRECTIONS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

fn parse(data: &str) -> (FxHashMap<IVec2, char>, Vec<&str>) {
    let cube = data
        .lines()
        .take(202)
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '.' || *ch == '#')
                .map(move |(j, ch)| (IVec2::new(i as i32 + 1, j as i32 + 1), ch))
        })
        .collect();
    let moves_list = data.trim().lines().last().unwrap();
    let re = Regex::new(r"R|L|\d+").unwrap();
    let moves = re.find_iter(moves_list).map(|m| m.as_str()).collect();

    (cube, moves)
}

//        4-> 1->
//      5         2
//      |         |
//      v         v
//             3->
//      6     3
//      |     |
//      v     v
//  6->
// ^          ^
// |          |
// 5          2
//        7->
// 4    7
// |    |
// v    v
//  1->
fn solve(
    cube: &FxHashMap<IVec2, char>,
    moves: &[&str],
    wrappings: &FxHashMap<IVec2, (IVec2, IVec2)>,
) -> i32 {
    let mut idx_d: i32 = 0;
    let mut pos = IVec2::new(1, 51);

    for m in moves {
        match *m {
            "R" => idx_d = (idx_d + 1).rem_euclid(4),
            "L" => idx_d = (idx_d - 1).rem_euclid(4),
            _ => {
                let d = m.parse::<usize>().unwrap();

                for _ in 0..d {
                    let mut dd = DIRECTIONS[idx_d as usize];
                    let mut dest = pos + dd;

                    if !cube.contains_key(&dest) {
                        (dest, dd) = *wrappings.get(&dest).unwrap();
                    }
                    if cube[&dest] == '.' {
                        pos = dest;
                        idx_d = DIRECTIONS.iter().position(|&x| x == dd).unwrap() as i32;
                    }
                }
            }
        }
    }

    1000 * pos.x + 4 * pos.y + idx_d
}

macro_rules! wrap {
    ($cont:expr, $x:expr, $y:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        $cont.insert(IVec2::new($x, $y), (IVec2::new($a, $b), IVec2::new($c, $d)))
    };
}

#[aoc(2022, 22)]
pub fn main() {
    let data = aoc_input!(2022, 22).unwrap();
    let (cube, moves) = parse(&data);

    // Part I
    let mut wrappings = FxHashMap::default();

    for i in 1..=50 {
        wrap!(wrappings, 0, 100 + i, 50, 100 + i, -1, 0);
        wrap!(wrappings, 51, 100 + i, 1, 100 + i, 1, 0);

        wrap!(wrappings, i, 151, i, 51, 0, 1);
        wrap!(wrappings, i, 50, i, 150, 0, -1);

        wrap!(wrappings, 0, 50 + i, 150, 50 + i, -1, 0);
        wrap!(wrappings, 151, 50 + i, 1, 50 + i, 1, 0);

        wrap!(wrappings, 50 + i, 101, 50 + i, 51, 0, 1);
        wrap!(wrappings, 50 + i, 50, 50 + i, 100, 0, -1);

        wrap!(wrappings, 100 + i, 101, 100 + i, 1, 0, 1);
        wrap!(wrappings, 100 + i, 0, 100 + i, 100, 0, -1);

        wrap!(wrappings, 150 + i, 51, 150 + i, 1, 0, 1);
        wrap!(wrappings, 150 + i, 0, 150 + i, 50, 0, -1);

        wrap!(wrappings, 100, i, 200, i, -1, 0);
        wrap!(wrappings, 201, i, 101, i, 1, 0);
    }

    println!("{}", solve(&cube, &moves, &wrappings));

    // Part II
    let mut wrappings = FxHashMap::default();

    for i in 1..=50 {
        wrap!(wrappings, 0, 100 + i, 200, i, -1, 0);
        wrap!(wrappings, 201, i, 1, 100 + i, 1, 0);

        wrap!(wrappings, i, 151, 151 - i, 100, 0, -1);
        wrap!(wrappings, 151 - i, 101, i, 150, 0, -1);

        wrap!(wrappings, 51, 100 + i, 50 + i, 100, 0, -1);
        wrap!(wrappings, 50 + i, 101, 50, 100 + i, -1, 0);

        wrap!(wrappings, 0, 50 + i, 150 + i, 1, 0, 1);
        wrap!(wrappings, 150 + i, 0, 1, 50 + i, 1, 0);

        wrap!(wrappings, i, 50, 151 - i, 1, 0, 1);
        wrap!(wrappings, 151 - i, 0, i, 51, 0, 1);

        wrap!(wrappings, 50 + i, 50, 101, i, 1, 0);
        wrap!(wrappings, 100, i, 50 + i, 51, 0, 1);

        wrap!(wrappings, 151, 50 + i, 150 + i, 50, 0, -1);
        wrap!(wrappings, 150 + i, 51, 150, 50 + i, -1, 0);
    }

    println!("{}", solve(&cube, &moves, &wrappings));
}
