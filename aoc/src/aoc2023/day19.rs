use aoc::{aoc, aoc_input, interval::Interval};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

type Workflows<'a> = FxHashMap<&'a str, Vec<&'a str>>;
type Rating = FxHashMap<char, i32>; // TODO: tuple instead?

fn parse(data: &str) -> (Workflows, Vec<Rating>) {
    let (workflows, ratings) = data.split_once("\n\n").unwrap();
    (
        workflows.lines().map(|line| parse_workflow(line)).collect(),
        ratings.lines().map(parse_rating).collect(),
    )
}

// parse single workflow
//   e.g.: px{a<2006:qkq,m>2090:A,rfg}
fn parse_workflow(w: &str) -> (&str, Vec<&str>) {
    let (name, rules) = w.split_once('{').unwrap();
    (
        name,
        rules
            .trim_end_matches('}')
            .split(',')
            .collect::<Vec<&str>>(),
    )
}

// parse single line
//  e.g.: {x=1679,m=44,a=2067,s=496}
fn parse_rating(r: &str) -> FxHashMap<char, i32> {
    r.trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|elem| {
            let (key, val) = elem.split_once('=').unwrap();
            let key = key.chars().next().unwrap();
            let val = val.parse::<i32>().unwrap();
            (key, val)
        })
        .collect::<FxHashMap<char, i32>>()
}

enum Outcome<'a> {
    Accepted,
    Rejected,
    Next(&'a str),
}

fn eval_single<'a>(rating: &'a Rating, logic: &Vec<&'a str>) -> Outcome<'a> {
    for test in logic {
        if test.contains(':') {
            let (check, id) = test.split_once(':').unwrap();

            if check.contains('<') {
                let (var, val) = check.split_once('<').unwrap();
                let var = var.chars().next().unwrap();
                let val = val.parse::<i32>().unwrap();

                if *rating.get(&var).unwrap() < val {
                    return match id {
                        "A" => Outcome::Accepted,
                        "R" => Outcome::Rejected,
                        id => Outcome::Next(id),
                    };
                }
                continue;
            } else if check.contains('>') {
                let (var, val) = check.split_once('>').unwrap();
                let var = var.chars().next().unwrap();
                let val = val.parse::<i32>().unwrap();

                if *rating.get(&var).unwrap() > val {
                    return match id {
                        "A" => Outcome::Accepted,
                        "R" => Outcome::Rejected,
                        id => Outcome::Next(id),
                    };
                }
                continue;
            }
            unreachable!()
        } else {
            return match *test {
                "A" => Outcome::Accepted,
                "R" => Outcome::Rejected,
                id => Outcome::Next(id),
            };
        }
    }
    unreachable!()
}

fn evaluate(rating: &Rating, workflows: &Workflows) -> bool {
    let mut key = "in";

    loop {
        let logic = workflows.get(&key).unwrap();
        let evald = eval_single(rating, logic);

        match evald {
            Outcome::Accepted => return true,
            Outcome::Rejected => return false,
            Outcome::Next(id) => key = id,
        }
    }
}

fn evaluate_range(
    rating_: &FxHashMap<char, Interval>,
    workflows: &Workflows,
) -> Vec<FxHashMap<char, Interval>> {
    let mut acc = vec![];
    let mut queue = VecDeque::new();
    queue.push_back((rating_.clone(), "in"));

    while let Some((mut rating, key)) = queue.pop_front() {
        // loop
        let logic = workflows.get(&key).unwrap();

        // eval
        for test in logic {
            // if statement
            if test.contains(':') {
                let (check, id) = test.split_once(':').unwrap();

                // a<2006:qkq
                if check.contains('<') {
                    let (var, val) = check.split_once('<').unwrap();
                    let var = var.chars().next().unwrap();
                    let val = val.parse::<i32>().unwrap();

                    let range = *rating.get(&var).unwrap();
                    let left_range: Result<Interval, ()> =
                        (range.start, std::cmp::min(range.end, val - 1)).try_into();
                    let right_range: Result<Interval, ()> =
                        (std::cmp::max(range.start, val), range.end).try_into();

                    if let Ok(left) = left_range {
                        let mut rating_true = rating.clone();
                        rating_true.entry(var).and_modify(|e| *e = left);
                        queue.push_back((rating_true, id));
                    };

                    rating
                        .entry(var)
                        .and_modify(|e| *e = right_range.ok().unwrap_or(Interval::new(0, 0)));

                    continue;
                } else if check.contains('>') {
                    // todo
                    let (var, val) = check.split_once('>').unwrap();
                    let var = var.chars().next().unwrap();
                    let val = val.parse::<i32>().unwrap();

                    let range = *rating.get(&var).unwrap();
                    let left_range: Result<Interval, ()> =
                        (range.start, std::cmp::min(range.end, val)).try_into();
                    let right_range: Result<Interval, ()> =
                        (std::cmp::max(range.start, val + 1), range.end).try_into();

                    if let Ok(right) = right_range {
                        let mut rating_true = rating.clone();
                        rating_true.entry(var).and_modify(|e| *e = right);
                        queue.push_back((rating_true, id));
                    };

                    rating
                        .entry(var)
                        .and_modify(|e| *e = left_range.ok().unwrap_or(Interval::new(0, 0)));

                    continue;
                }
                unreachable!()
            }
            // R or A?
            else {
                match *test {
                    "A" => acc.push(rating.clone()),
                    "R" => {}
                    id => queue.push_back((rating.clone(), id)),
                }
            }
        }

        // match
    }

    // println!("{:?}", acc);
    acc
}

#[aoc(2023, 19)]
pub fn main() {
    let data = aoc_input!(2023, 19).unwrap();
    let (mut workflows, ratings) = parse(&data);

    // Part I
    let res: i32 = ratings
        .iter()
        .filter(|rating| evaluate(rating, &workflows))
        .map(|rating| rating.values().sum::<i32>())
        .sum();
    println!("{res}");

    // Part II
    let ratings = FxHashMap::from_iter([
        ('x', Interval::new(1, 4000)),
        ('m', Interval::new(1, 4000)),
        ('a', Interval::new(1, 4000)),
        ('s', Interval::new(1, 4000)),
    ]);

    workflows.insert("A", vec!["A"]);
    workflows.insert("R", vec!["R"]);

    let accepted = evaluate_range(&ratings, &workflows);
    let n_comb: usize = accepted
        .iter()
        .map(|d| d.values().map(|v| v.len() as usize + 1).product::<usize>())
        .sum();

    println!("{n_comb}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        let (workflows, ratings) = parse(EXAMPLE);
        println!("{:?}", workflows);
        println!("{:?}", ratings);

        let mut s = 0;

        for rating in ratings {
            if evaluate(&rating, &workflows) {
                s += rating.values().sum::<i32>();
            }
        }

        assert_eq!(s, 19114);
    }

    #[test]
    fn test_part2() {
        let (mut workflows, _) = parse(EXAMPLE);
        let ratings = FxHashMap::from_iter([
            ('x', Interval::new(1, 4000)),
            ('m', Interval::new(1, 4000)),
            ('a', Interval::new(1, 4000)),
            ('s', Interval::new(1, 4000)),
        ]);

        workflows.insert("A", vec!["A"]);
        workflows.insert("R", vec!["R"]);

        let accepted = evaluate_range(&ratings, &workflows);
        let n_comb: usize = accepted
            .iter()
            .map(|d| d.values().map(|v| v.len() as usize + 1).product::<usize>())
            .sum();

        assert_eq!(n_comb, 167409079868000);
    }
}
