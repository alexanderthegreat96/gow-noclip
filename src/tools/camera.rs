use crate::tools::game::Game;
use crate::tools::memory_mappings;
use std::sync::Arc;

/// Camera's world-space basis in a script-friendly convention:
/// `forward` is the direction the camera is looking (already negated).
pub struct Basis {
    pub right: [f32; 3],
    pub forward: [f32; 3],
}

pub struct Camera {
    game: Arc<Game>,
    yaw: usize,
    pitch: usize,
    right_vec: usize,
    back_vec: usize,
}

impl Camera {
    pub fn attach(game: Arc<Game>) -> Option<Self> {
        Some(Self {
            yaw: game.resolve(memory_mappings::camera::YAW)?,
            pitch: game.resolve(memory_mappings::camera::PITCH)?,
            right_vec: game.resolve(memory_mappings::camera::LEFT_VEC)?,
            back_vec: game.resolve(memory_mappings::camera::FORWARD_VEC)?,
            game,
        })
    }

    pub fn yaw(&self) -> Option<f32> {
        self.game.read::<f32>(self.yaw)
    }

    #[allow(dead_code)]
    pub fn pitch(&self) -> Option<f32> {
        self.game.read::<f32>(self.pitch)
    }

    pub fn basis(&self) -> Option<Basis> {
        let right = self.game.read::<[f32; 3]>(self.right_vec)?;
        let back = self.game.read::<[f32; 3]>(self.back_vec)?;
        Some(Basis {
            right,
            forward: [-back[0], -back[1], -back[2]],
        })
    }
}
