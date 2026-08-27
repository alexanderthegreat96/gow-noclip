use crate::tools::game::Game;
use crate::tools::memory_mappings;
use std::sync::Arc;

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
        // Use the sig-resolved (or fallback) camera struct pointer.
        let cam = game.camera_ptr()?;
        Some(Self {
            yaw: cam + memory_mappings::camera::YAW_OFFSET,
            pitch: cam + memory_mappings::camera::PITCH_OFFSET,
            right_vec: cam + memory_mappings::camera::LEFT_VEC_OFFSET,
            back_vec: cam + memory_mappings::camera::FORWARD_VEC_OFFSET,
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
