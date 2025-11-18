use std::fmt::Display;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, Default)]
pub struct GameResources {
    pub wood: u32,
    pub stone: u32,
    pub gold: u32,
}

impl Display for GameResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wood: {}, Stone: {}, Gold: {}",
            self.wood, self.stone, self.gold
        )
    }
}
