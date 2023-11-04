use lazy_static::lazy_static;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Spell {
    hp: i32,
    mana: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    recharge: i32,
    timer: i32,
}

type Player = Spell;

impl Spell {
    fn cast(&mut self) -> Option<Self> {
        self.timer -= 1;
        match self.timer {
            0 => None,
            _ => Some(*self),
        }
    }
}

lazy_static! {
    static ref SPELLS: &'static [Spell] = &[
        Spell {
            hp: 0,
            mana: 53,
            damage: 4,
            heal: 0,
            armor: 0,
            recharge: 0,
            timer: 1
        },
        Spell {
            hp: 0,
            mana: 73,
            damage: 2,
            heal: 2,
            armor: 0,
            recharge: 0,
            timer: 1
        },
        Spell {
            hp: 0,
            mana: 113,
            damage: 0,
            heal: 0,
            armor: 7,
            recharge: 0,
            timer: 6
        },
        Spell {
            hp: 0,
            mana: 173,
            damage: 3,
            heal: 0,
            armor: 0,
            recharge: 0,
            timer: 6
        },
        Spell {
            hp: 0,
            mana: 229,
            damage: 0,
            heal: 0,
            armor: 0,
            recharge: 101,
            timer: 5
        }
    ];
}

fn search(player: &Player, boss: &Player) -> usize {
    let mut min_mana = usize::MAX;
    let mut queue = VecDeque::new();
    min_mana
}

fn main() {
    let mut player = Player {
        hp: 50,
        mana: 500,
        damage: 0,
        heal: 0,
        armor: 0,
        recharge: 0,
        timer: 0,
    };
    let mut boss = Player {
        hp: 58,
        mana: 0,
        damage: 9,
        heal: 0,
        armor: 0,
        recharge: 0,
        timer: 0,
    };
}
