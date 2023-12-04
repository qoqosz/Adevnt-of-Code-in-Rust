//! Based on: https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2015/day22.rs
use aoc::heap::MinHeap;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    boss_hp: i16,
    player_hp: i16,
    player_mana: i16,
    shield_effect: u8,
    poison_effect: u8,
    recharge_effect: u8,
}

impl State {
    fn new(boss_hp: i16) -> Self {
        Self {
            boss_hp,
            player_hp: 50,
            player_mana: 500,
            shield_effect: 0,
            poison_effect: 0,
            recharge_effect: 0,
        }
    }

    fn apply_spell_effects(&mut self) -> Self {
        if self.shield_effect > 0 {
            self.shield_effect -= 1;
        }
        if self.poison_effect > 0 {
            self.poison_effect -= 1;
            self.boss_hp -= 3;
        }
        if self.recharge_effect > 0 {
            self.recharge_effect -= 1;
            self.player_mana += 101;
        }
        *self
    }

    fn boss_turn(&mut self, attack: i16) -> Self {
        self.player_hp -= match self.shield_effect > 0 {
            true => (attack - 7).max(1),
            false => attack,
        };
        *self
    }

    fn is_win(&self) -> bool {
        self.boss_hp <= 0
    }

    fn is_alive(&self) -> bool {
        self.player_hp > 0 && self.player_mana >= 53
    }

    fn lose_hp(&mut self) -> Self {
        self.player_hp -= 1;
        *self
    }

    fn cast(&self, spell: &Spell) -> Option<Self> {
        spell.apply(self)
    }
}

enum Spell {
    MagicMissile(i16, i16),
    Drain(i16, i16, i16),
    Shield(i16, u8),
    Poison(i16, u8),
    Recharge(i16, u8),
}

impl Spell {
    fn apply(&self, state: &State) -> Option<State> {
        match *self {
            Spell::MagicMissile(mana_cost, damage) => state.magic_missile(mana_cost, damage),
            Spell::Drain(mana_cost, damage, heal) => state.drain(mana_cost, damage, heal),
            Spell::Shield(mana_cost, shield_effect) => state.shield(mana_cost, shield_effect),
            Spell::Poison(mana_cost, poison_effect) => state.poison(mana_cost, poison_effect),
            Spell::Recharge(mana_cost, recharge_effect) => {
                state.recharge(mana_cost, recharge_effect)
            }
        }
    }
}

trait MagicMissile: Sized {
    fn magic_missile(&self, mana_cost: i16, damage: i16) -> Option<Self>;
}

impl MagicMissile for State {
    fn magic_missile(&self, mana_cost: i16, damage: i16) -> Option<Self> {
        if self.player_mana < mana_cost {
            return None;
        }
        Some(Self {
            boss_hp: self.boss_hp - damage,
            player_mana: self.player_mana - mana_cost,
            ..*self
        })
    }
}

trait Drain: Sized {
    fn drain(&self, mana_cost: i16, damage: i16, heal: i16) -> Option<Self>;
}

impl Drain for State {
    fn drain(&self, mana_cost: i16, damage: i16, heal: i16) -> Option<Self> {
        if self.player_mana < mana_cost {
            return None;
        }
        Some(Self {
            boss_hp: self.boss_hp - damage,
            player_hp: self.player_hp + heal,
            player_mana: self.player_mana - mana_cost,
            ..*self
        })
    }
}

trait Shield: Sized {
    fn shield(&self, mana_cost: i16, shield_effect: u8) -> Option<Self>;
}

impl Shield for State {
    fn shield(&self, mana_cost: i16, shield_effect: u8) -> Option<Self> {
        if self.player_mana < mana_cost || self.shield_effect != 0 {
            return None;
        }
        Some(Self {
            player_mana: self.player_mana - mana_cost,
            shield_effect,
            ..*self
        })
    }
}

trait Poison: Sized {
    fn poison(&self, mana_cost: i16, poison_effect: u8) -> Option<Self>;
}

impl Poison for State {
    fn poison(&self, mana_cost: i16, poison_effect: u8) -> Option<Self> {
        if self.player_mana < mana_cost || self.poison_effect != 0 {
            return None;
        }
        Some(Self {
            player_mana: self.player_mana - mana_cost,
            poison_effect,
            ..*self
        })
    }
}

trait Recharge: Sized {
    fn recharge(&self, mana_cost: i16, recharge_effect: u8) -> Option<Self>;
}

impl Recharge for State {
    fn recharge(&self, mana_cost: i16, recharge_effect: u8) -> Option<Self> {
        if self.player_mana < mana_cost || self.recharge_effect != 0 {
            return None;
        }
        Some(Self {
            player_mana: self.player_mana - mana_cost,
            recharge_effect,
            ..*self
        })
    }
}

fn play(boss_hp: i16, damage: i16, hard_mode: bool) -> i16 {
    let start = State::new(boss_hp);
    let mut queue = MinHeap::from([(0, start)]);
    let mut cache = FxHashSet::from_iter([start]);

    while let Some((spent, mut state)) = queue.pop() {
        if state.apply_spell_effects().is_win() {
            return spent;
        }

        if hard_mode && !state.lose_hp().is_alive() {
            continue;
        }

        for spell in &SPELLS {
            if let Some(mut next) = state.cast(spell) {
                let cost = state.player_mana - next.player_mana;

                if next.apply_spell_effects().is_win() {
                    return spent + cost;
                }
                if next.boss_turn(damage).is_alive() && cache.insert(next) {
                    queue.push(spent + cost, next);
                }
            }
        }
    }

    unreachable!()
}

static SPELLS: [Spell; 5] = [
    Spell::MagicMissile(53, 4),
    Spell::Drain(73, 2, 2),
    Spell::Shield(113, 6),
    Spell::Poison(173, 6),
    Spell::Recharge(229, 5),
];

pub fn main() {
    let (boss_hp, damage): (i16, i16) = (58, 9);

    // Part I
    let mana = play(boss_hp, damage, false);
    println!("{mana}");

    // Part II
    let mana = play(boss_hp, damage, true);
    println!("{mana}");
}
