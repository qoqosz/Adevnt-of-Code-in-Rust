use aoc::{aoc, aoc_input};
use glam::IVec4;
use itertools::Itertools;
use regex_lite::Regex;
use rustc_hash::FxHashSet;
use std::{collections::VecDeque, sync::LazyLock};

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

fn bfs(state0: &[i32; 9], blueprint: &[i32; 7], max_time: i32) -> i32 {
    let [_, oro, cro, obro, obrc, gro, grob] = *blueprint;
    let mut queue = VecDeque::from_iter([*state0]);
    let mut visited = FxHashSet::default();
    let mut max_geodes = 0;

    while let Some(s) = queue.pop_front() {
        if !visited.insert(s) {
            continue;
        }

        let [o, c, ob, g, or, cr, obr, gr, t] = s;

        if g > max_geodes {
            max_geodes = g;
        }

        if t == max_time {
            continue;
        }

        // build ore robot
        if o >= oro && o <= 3 * oro {
            queue.push_back([
                o - oro + or,
                c + cr,
                ob + obr,
                g + gr,
                or + 1,
                cr,
                obr,
                gr,
                t + 1,
            ]);
        }
        // build clay robot
        if o >= cro && o <= 3 * cro {
            queue.push_back([
                o - cro + or,
                c + cr,
                ob + obr,
                g + gr,
                or,
                cr + 1,
                obr,
                gr,
                t + 1,
            ]);
        }
        // build obsidian robot
        if o >= obro && c >= obrc && o <= 3 * obro && c <= 3 * obrc {
            queue.push_back([
                o - obro + or,
                c - obrc + cr,
                ob + obr,
                g + gr,
                or,
                cr,
                obr + 1,
                gr,
                t + 1,
            ]);
        }
        // build geode robot
        if o >= gro && ob >= grob && o <= 3 * gro && ob <= 3 * grob {
            queue.push_back([
                o - gro + or,
                c + cr,
                ob - grob + obr,
                g + gr,
                or,
                cr,
                obr,
                gr + 1,
                t + 1,
            ]);
        }
        // don't build robots
        queue.push_back([o + or, c + cr, ob + obr, g + gr, or, cr, obr, gr, t + 1]);
    }

    max_geodes
}

fn parse(data: &str) -> Vec<[i32; 7]> {
    data.trim()
        .lines()
        .flat_map(|line| {
            RE.find_iter(line)
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<_>>()
                .try_into()
        })
        .collect()
}

type Blueprint = [[IVec4; 2]; 5];

fn convert(data: [i32; 7]) -> Blueprint {
    let [_, a, b, c, d, e, f] = data;
    [
        [IVec4::new(0, f, 0, e), IVec4::X],
        [IVec4::new(0, 0, d, c), IVec4::Y],
        [IVec4::new(0, 0, 0, b), IVec4::Z],
        [IVec4::new(0, 0, 0, a), IVec4::W],
        [IVec4::ZERO, IVec4::ZERO],
    ]
}

// reworked approach
fn search(blueprint: &Blueprint, max_time: i32) -> i32
where
{
    let resources = IVec4::ZERO;
    let robots = IVec4::W;
    let mut queue = vec![[resources, robots]];

    for _ in 0..max_time {
        let mut tmp = vec![];

        for &[resources, robots] in &queue {
            for &[cost, outcome] in blueprint {
                if resources.cmpge(cost).all() {
                    tmp.push([resources - cost + robots, robots + outcome]);
                }
            }
        }
        tmp.sort_unstable_by_key(|[x, y]| std::cmp::Reverse((*x + *y).to_array()));
        let n = std::cmp::min(tmp.len(), 6000);
        queue = tmp[..n].to_vec();
    }

    queue[0][0][0]
}

#[aoc(2022, 19)]
pub fn main() {
    let data = aoc_input!(2022, 19).unwrap();
    let blueprints = parse(&data);

    // Part I
    let state0 = [0, 0, 0, 0, 1, 0, 0, 0, 0];
    let result: i32 = blueprints
        .iter()
        .map(|blueprint| bfs(&state0, &blueprint, 24) * blueprint[0])
        .sum();
    println!("{result}");

    // Part II
    let result: i32 = blueprints[..3]
        .iter()
        .map(|blueprint| {
            let blueprint = convert(*blueprint);
            search(&blueprint, 32)
        })
        .product1()
        .unwrap();
    println!("{result}");
}
