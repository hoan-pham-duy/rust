// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        return if self.health == 0 {
            Some(Self {
                health: 100,
                mana: {
                    if self.level >= 10 {
                        Some(100)
                    } else {
                        None
                    }
                },
                level: self.level
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        let ret_val = match self.mana{
            None => {
                // if self.health >= mana_cost {
                //     self.health -= mana_cost
                // } else {
                //     self.health = 0;
                // }
                // 0
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(curr_mana) => {
                return if curr_mana >= mana_cost {
                    self.mana = Some(curr_mana - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
        };
        return ret_val;
    }
}
