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
}

trait MagicMissile: Sized {
    fn magic_missile(&self) -> Option<Self>;
}

impl MagicMissile for State {
    fn magic_missile(&self) -> Option<Self> {
        if self.player_mana < 53 {
            return None;
        }
        Some(Self {
            boss_hp: self.boss_hp - 4,
            player_mana: self.player_mana - 53,
            ..*self
        })
    }
}

trait Drain: Sized {
    fn drain(&self) -> Option<Self>;
}

impl Drain for State {
    fn drain(&self) -> Option<Self> {
        if self.player_mana < 73 {
            return None;
        }
        Some(Self {
            boss_hp: self.boss_hp - 2,
            player_hp: self.player_hp + 2,
            player_mana: self.player_mana - 73,
            ..*self
        })
    }
}

trait Shield: Sized {
    fn shield(&self) -> Option<Self>;
}

impl Shield for State {
    fn shield(&self) -> Option<Self> {
        if self.player_mana < 113 || self.shield_effect != 0 {
            return None;
        }
        Some(Self {
            player_mana: self.player_mana - 113,
            shield_effect: 6,
            ..*self
        })
    }
}

trait Poison: Sized {
    fn poison(&self) -> Option<Self>;
}

impl Poison for State {
    fn poison(&self) -> Option<Self> {
        if self.player_mana < 173 || self.poison_effect != 0 {
            return None;
        }
        Some(Self {
            player_mana: self.player_mana - 173,
            poison_effect: 6,
            ..*self
        })
    }
}

trait Recharge: Sized {
    fn recharge(&self) -> Option<Self>;
}

impl Recharge for State {
    fn recharge(&self) -> Option<Self> {
        if self.player_mana < 229 || self.recharge_effect != 0 {
            return None;
        }
        Some(Self {
            player_mana: self.player_mana - 229,
            recharge_effect: 5,
            ..*self
        })
    }
}

fn play(boss_hp: i16, damage: i16, hard_mode: bool) -> i16 {
    let start = State::new(boss_hp);
    let mut queue = MinHeap::new();
    let mut cache = FxHashSet::default();

    queue.push(0, start);
    cache.insert(start);

    while let Some((spent, mut state)) = queue.pop() {
        if state.apply_spell_effects().is_win() {
            return spent;
        }

        if hard_mode {
            if state.player_hp > 1 {
                state.player_hp -= 1;
            } else {
                continue;
            }
        }

        let mut cast_spell = |effect: Option<State>| {
            if let Some(mut next) = effect {
                let cost = state.player_mana - next.player_mana;
                if next.apply_spell_effects().is_win() {
                    return Some(spent + cost);
                }
                if next.boss_turn(damage).is_alive() && cache.insert(next) {
                    queue.push(spent + cost, next);
                }
            }
            None
        };

        // Magic Missile
        if let Some(spent) = cast_spell(state.magic_missile()) {
            return spent;
        }
        // Drain
        if let Some(spent) = cast_spell(state.drain()) {
            return spent;
        }
        // Shield
        if let Some(spent) = cast_spell(state.shield()) {
            return spent;
        }
        // Poison
        if let Some(spent) = cast_spell(state.poison()) {
            return spent;
        }
        // Recharge
        if let Some(spent) = cast_spell(state.recharge()) {
            return spent;
        }
    }

    unreachable!()
}

pub fn main() {
    let (boss_hp, damage): (i16, i16) = (58, 9);

    // Part I
    let mana = play(boss_hp, damage, false);
    println!("{mana}");

    // Part II
    let mana = play(boss_hp, damage, true);
    println!("{mana}");
}
