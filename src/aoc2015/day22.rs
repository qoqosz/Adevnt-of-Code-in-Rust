use itertools::Itertools;
/// Based on: https://eddmann.com/posts/advent-of-code-2015-day-22-wizard-simulator-20xx/
///
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone)]
struct Spell<'a> {
    name: &'a str,
    cost: i32,
    damage: i32,
    armor: i32,
    heal: i32,
    mana: i32,
    timer: i32,
}

lazy_static! {
    static ref SPELLS: &'static [Spell<'static>] = &[
        Spell {
            name: "Magic Missile",
            cost: 53,
            damage: 4,
            armor: 0,
            heal: 0,
            mana: 0,
            timer: 1
        },
        Spell {
            name: "Drain",
            cost: 73,
            damage: 2,
            armor: 0,
            heal: 2,
            mana: 0,
            timer: 1
        },
        Spell {
            name: "Shield",
            cost: 113,
            damage: 0,
            armor: 7,
            heal: 0,
            mana: 0,
            timer: 6
        },
        Spell {
            name: "Poison",
            cost: 173,
            damage: 3,
            armor: 0,
            heal: 0,
            mana: 0,
            timer: 6
        },
        Spell {
            name: "Recharge",
            cost: 229,
            damage: 0,
            armor: 0,
            heal: 0,
            mana: 101,
            timer: 5
        }
    ];
}

#[derive(Debug, Clone)]
struct BattleState {
    player_hp: i32,
    player_mana: i32,
    player_armor: i32,
    boss_hp: i32,
    boss_damage: i32,
    mana_spent: i32,
    active_effects: Vec<Spell<'static>>,
}

impl BattleState {
    fn is_done(&self) -> bool {
        self.player_hp <= 0 || self.boss_hp <= 0
    }

    fn get_available_spells(&self) -> Vec<Spell<'static>> {
        if self.is_done() {
            return vec![];
        }
        SPELLS
            .iter()
            .filter(|spell| {
                let active = self
                    .active_effects
                    .iter()
                    .find(|effect| spell.name == effect.name);

                let is_inact = match active {
                    Some(act) => act.timer == 1,
                    _ => true,
                };
                spell.cost <= self.player_mana && is_inact
            })
            .copied()
            .collect_vec()
    }
}

struct BattleTransition {
    state: BattleState,
}

impl BattleTransition {
    fn enact_effects(&self) -> BattleState {
        if self.state.is_done() {
            return self.state.clone();
        }

        let mut state = self.state.clone();
        state.player_mana += state.active_effects.iter().map(|x| x.mana).sum::<i32>();
        state.player_armor += state.active_effects.iter().map(|x| x.armor).sum::<i32>();
        state.boss_hp -= state.active_effects.iter().map(|x| x.damage).sum::<i32>();
        state
            .active_effects
            .iter_mut()
            .for_each(|eff| eff.timer -= 1);
        state.active_effects = state
            .active_effects
            .into_iter()
            .filter(|eff| eff.timer > 0)
            .collect::<Vec<_>>();
        state
    }

    fn player_turn(&self, spell: &'static Spell) -> BattleState {
        if self.state.is_done() {
            return self.state.clone();
        }

        let is_spell_effect = spell.timer > 1;

        let mut state = self.state.clone();
        state.player_hp += if is_spell_effect { 0 } else { spell.heal };
        state.player_mana -= spell.cost;
        state.boss_hp -= if is_spell_effect { 0 } else { spell.damage };
        state.mana_spent += spell.cost;

        if is_spell_effect {
            state.active_effects.push(*spell);
        }
        state
    }

    fn boss_turn(&self) -> BattleState {
        if self.state.is_done() {
            return self.state.clone();
        }
        let mut state = self.state.clone();
        state.player_hp -= std::cmp::max(state.boss_damage - state.player_armor, 1);
        state
    }
}

pub fn main() {}
