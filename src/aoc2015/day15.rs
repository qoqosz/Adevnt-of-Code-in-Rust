use aoc::aoc_input;
use fxhash::{FxBuildHasher, FxHashSet};
use itertools::iproduct;
use regex::Regex;
use std::cmp::max;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Default)]
struct Property {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Property {
    fn new(capacity: i64, durability: i64, flavor: i64, texture: i64, calories: i64) -> Self {
        Property {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }

    fn score(&self) -> i64 {
        {
            max(self.capacity, 0)
                * max(self.durability, 0)
                * max(self.flavor, 0)
                * max(self.texture, 0)
        }
    }
}

impl From<&str> for Property {
    fn from(value: &str) -> Self {
        let re: Regex = Regex::new(r"-*\d+").unwrap();
        let nums: Vec<_> = re
            .find_iter(value)
            .filter_map(|d| d.as_str().parse::<i64>().ok())
            .collect();

        Property::new(nums[0], nums[1], nums[2], nums[3], nums[4])
    }
}

impl std::ops::Mul<i64> for Property {
    type Output = Property;

    fn mul(self, rhs: i64) -> Self::Output {
        Property {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl std::ops::Add<Property> for Property {
    type Output = Property;

    fn add(self, rhs: Property) -> Self::Output {
        Property {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

fn search(ingredients: &Vec<Property>, calories: Option<i64>) -> i64 {
    let n = ingredients.len();
    let mut max_score: i64 = 0;

    let mut start = vec![0i64; n];
    start[n - 1] = 100;

    let mut queue: VecDeque<Vec<i64>> = VecDeque::with_capacity(n * n);
    queue.push_front(start);

    let mut visited: FxHashSet<Vec<i64>> =
        HashSet::with_capacity_and_hasher(n * n * n * n, FxBuildHasher::default());

    while let Some(coeff) = queue.pop_front() {
        if visited.contains(&coeff) {
            continue;
        }

        visited.insert(coeff.clone());

        let property: Property = ingredients
            .iter()
            .zip(coeff.iter())
            .fold(Property::default(), |acc, (&p, &c)| acc + p * c);

        match calories {
            Some(ref cals) if *cals != property.calories => {}
            _ => {
                max_score = std::cmp::max(max_score, property.score());
            }
        }

        for (i, j) in iproduct!(0..n, 0..n) {
            if i == j {
                continue;
            }
            if coeff[i] != 0 {
                let mut c = coeff.clone();
                c[i] -= 1;
                c[j] += 1;
                queue.push_back(c);
            }
        }
    }

    max_score
}

fn main() {
    let data = aoc_input!(2015, 15).unwrap();
    let mut ingredients = vec![];

    for line in data.split('\n').filter(|x| !x.is_empty()) {
        ingredients.push(Property::from(line));
    }

    // Part I
    println!("{:?}", search(&ingredients, None));

    // Part II
    println!("{:?}", search(&ingredients, Some(500)));
}
