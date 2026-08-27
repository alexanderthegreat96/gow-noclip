use crate::tools::game::Game;
use crate::tools::memory_mappings;
use std::sync::Arc;

pub struct Player {
    game: Arc<Game>,
    pos: usize,
    pos_mirror: usize,
    accel: usize,
    body_yaw: usize,
}

impl Player {
    pub fn attach(game: Arc<Game>) -> Option<Self> {
        let entity = game.entity_ptr()?;
        Some(Self {
            pos: entity + memory_mappings::entity::POS_X_OFFSET,
            pos_mirror: entity + memory_mappings::entity::POS_MIRROR_OFFSET,
            accel: entity + memory_mappings::entity::ACCEL_OFFSET,
            body_yaw: entity + memory_mappings::entity::BODY_YAW_OFFSET,
            game,
        })
    }

    #[allow(dead_code)]
    pub fn position(&self) -> Option<[f32; 3]> {
        self.game.read::<[f32; 3]>(self.pos)
    }

    pub fn nudge(&self, d: [f32; 3]) -> Option<()> {
        let p = self.game.read::<[f32; 3]>(self.pos)?;
        let m = self.game.read::<[f32; 3]>(self.pos_mirror)?;
        let np = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];
        let nm = [m[0] + d[0], m[1] + d[1], m[2] + d[2]];
        self.game.write(self.pos, &np)?;
        self.game.write(self.pos_mirror, &nm)?;
        Some(())
    }

    pub fn height(&self) -> Option<f32> {
        self.game.read::<f32>(self.pos + 4)
    }

    pub fn set_height(&self, y: f32) -> Option<()> {
        self.game.write::<f32>(self.pos + 4, &y)
    }

    pub fn adjust_height(&self, dy: f32) -> Option<f32> {
        let y = self.height()?;
        let new_y = y + dy;
        self.set_height(new_y)?;
        Some(new_y)
    }

    pub fn acceleration(&self) -> Option<f32> {
        self.game.read::<f32>(self.accel)
    }

    pub fn set_acceleration(&self, v: f32) -> Option<()> {
        self.game.write::<f32>(self.accel, &v)
    }

    #[allow(dead_code)]
    pub fn body_yaw(&self) -> Option<f32> {
        self.game.read::<f32>(self.body_yaw)
    }
}
