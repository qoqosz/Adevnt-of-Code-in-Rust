use aoc::aoc;
use itertools::Itertools;

// (cost, damage, defense)
type Item = (i32, i32, i32);
// Rules:
// 1 weapon
// 0 or 1 armor
// 0-2 rings
static WEAPONS: &[Item] = &[
    (8, 4, 0),  // dagger
    (10, 5, 0), // shortsword
    (25, 6, 0), // warhammer
    (40, 7, 0), // longsword
    (74, 8, 0), // greataxe
];
static ARMORS: &[Item] = &[
    (0, 0, 0),   // no armor
    (13, 0, 1),  // leather
    (31, 0, 2),  // chainmail
    (53, 0, 3),  // splintmail
    (75, 0, 4),  // bandedmail
    (102, 0, 5), // platemail
];
static RINGS: &[Item] = &[
    (0, 0, 0),   // no ring
    (0, 0, 0),   // no ring
    (25, 1, 0),  // damage +1
    (50, 2, 0),  // damage +2
    (100, 3, 0), // damage +3
    (20, 0, 1),  // defense +1
    (40, 0, 2),  // defense +2
    (80, 0, 3),  // defense +3
];

fn is_win(mut player: Item, mut boss: Item) -> bool {
    let mut i = 0;
    //    let (mut player, mut boss, mut i) = (player, boss, 0);

    while boss.0 > 0 {
        boss.0 -= std::cmp::max(1, player.1 - boss.2);
        (player, boss, i) = (boss, player, i + 1);
    }

    i % 2 == 0
}

fn search(player: Item, boss: Item) -> (i32, i32) {
    let mut min_cost = i32::MAX;
    let mut max_cost = 0;

    for weapon in WEAPONS {
        for armor in ARMORS {
            for rings in RINGS.iter().combinations(2) {
                let (ring1, ring2) = rings.into_iter().collect_tuple().unwrap();
                let equipment = (
                    weapon.0 + armor.0 + ring1.0 + ring2.0,
                    weapon.1 + armor.1 + ring1.1 + ring2.1,
                    weapon.2 + armor.2 + ring1.2 + ring2.2,
                );

                if is_win((player.0, equipment.1, equipment.2), boss) {
                    min_cost = std::cmp::min(equipment.0, min_cost);
                } else {
                    max_cost = std::cmp::max(equipment.0, max_cost);
                }
            }
        }
    }

    (min_cost, max_cost)
}

#[aoc(2015, 12)]
pub fn main() {
    let boss = (103, 9, 2);
    let player = (100, 0, 0);

    let (min_cost, max_cost) = search(player, boss);

    // Part I
    println!("{}", min_cost);

    // Part II
    println!("{}", max_cost);
}
