use aoc::aoc_input;

#[derive(Debug, Default)]
struct Compounds {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Compounds {
    fn to_vec(&self) -> Vec<Option<usize>> {
        vec![
            self.children,    // 0
            self.cats,        // 1
            self.samoyeds,    // 2
            self.pomeranians, // 3
            self.akitas,      // 4
            self.vizslas,     // 5
            self.goldfish,    // 6
            self.trees,       // 7
            self.cars,        // 8
            self.perfumes,    // 9
        ]
    }

    fn contains(&self, other: &Compounds) -> bool {
        for (x, y) in self.to_vec().iter().zip(other.to_vec().iter()) {
            if x.is_some() && y.is_some() && x.unwrap() != y.unwrap() {
                return false;
            }
        }
        true
    }

    fn contains_adj(&self, other: &Compounds) -> bool {
        for (i, (x, y)) in self.to_vec().iter().zip(other.to_vec().iter()).enumerate() {
            // Cats and trees
            if i == 1 || i == 7 {
                if x.is_some() && y.is_some() && x.unwrap() >= y.unwrap() {
                    return false;
                }
            }
            // pomeranians and goldfish
            else if i == 3 || i == 6 {
                if x.is_some() && y.is_some() && x.unwrap() <= y.unwrap() {
                    return false;
                }
            } else if x.is_some() && y.is_some() && x.unwrap() != y.unwrap() {
                return false;
            }
        }
        true
    }
}

impl<'a> From<&Vec<(&'a str, usize)>> for Compounds {
    fn from(parts: &Vec<(&'a str, usize)>) -> Self {
        let mut comps = Compounds::default();

        for (comp, qty) in parts {
            match *comp {
                "children" => comps.children = Some(*qty),
                "cats" => comps.cats = Some(*qty),
                "samoyeds" => comps.samoyeds = Some(*qty),
                "pomeranians" => comps.pomeranians = Some(*qty),
                "akitas" => comps.akitas = Some(*qty),
                "vizslas" => comps.vizslas = Some(*qty),
                "goldfish" => comps.goldfish = Some(*qty),
                "trees" => comps.trees = Some(*qty),
                "cars" => comps.cars = Some(*qty),
                "perfumes" => comps.perfumes = Some(*qty),
                _ => {}
            }
        }

        comps
    }
}

fn parse_line(line: &str) -> Vec<(&str, usize)> {
    let mut parts = line.split(',').collect::<Vec<_>>();
    let part0 = parts.remove(0).split(':').collect::<Vec<_>>();
    let _sue = part0[0];
    let cmp = part0[1].trim();
    let qty = part0[2].trim().parse::<usize>().unwrap();
    let mut parts = parts
        .iter()
        .map(|p| {
            let cmps = p.split(':').collect::<Vec<_>>();
            let typ = cmps[0].trim();
            let qty = cmps[1].trim().parse::<usize>().unwrap();
            (typ, qty)
        })
        .collect::<Vec<(&str, usize)>>();
    parts.push((cmp, qty));
    parts
}

fn main() {
    let data = aoc_input!(2015, 16).unwrap();
    let sues: Vec<Compounds> = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| Compounds::from(&parse_line(l)))
        .collect::<Vec<_>>();
    let message = Compounds {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    // Part I
    let i = sues
        .iter()
        .enumerate()
        .find_map(|(i, sue)| match message.contains(sue) {
            true => Some(i + 1),
            _ => None,
        })
        .unwrap();
    println!("{}", i);

    // Part II
    for (i, sue) in sues.iter().enumerate() {
        if message.contains_adj(sue) {
            println!("{}", i + 1);
        }
    }
}
