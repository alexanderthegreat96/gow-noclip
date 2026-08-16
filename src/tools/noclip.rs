use crate::tools::camera::Camera;
use crate::tools::game::Game;
use crate::tools::player::Player;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct NoClip {
    game: Arc<Game>,
    session: Mutex<Option<(Player, Camera)>>,
    active: AtomicBool,
    gravity_locked: AtomicBool,
    target_height: Mutex<Option<f32>>,
}

impl NoClip {
    pub fn new(game: Arc<Game>) -> Self {
        Self {
            game,
            session: Mutex::new(None),
            active: AtomicBool::new(false),
            gravity_locked: AtomicBool::new(true),
            target_height: Mutex::new(None),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    pub fn toggle(&self) -> bool {
        let new = !self.active.load(Ordering::Relaxed);
        self.active.store(new, Ordering::Relaxed);
        *self.target_height.lock().unwrap() = None;

        if new {
            let p = Player::attach(self.game.clone());
            let c = Camera::attach(self.game.clone());
            *self.session.lock().unwrap() = match (p, c) {
                (Some(p), Some(c)) => Some((p, c)),
                _ => None,
            };
        } else {
            *self.session.lock().unwrap() = None;
        }
        new
    }

    pub fn set_gravity_locked(&self, locked: bool) {
        self.gravity_locked.store(locked, Ordering::Relaxed);
    }

    pub fn move_wasd(&self, intent_x: f32, intent_z: f32, speed: f32) -> Option<()> {
        let session = self.session.lock().unwrap();
        let (player, camera) = session.as_ref()?;

        let mag = (intent_x * intent_x + intent_z * intent_z).sqrt();
        if mag == 0.0 {
            return Some(());
        }
        let nx = intent_x / mag;
        let nz = intent_z / mag;

        let basis = camera.basis()?;
        let d = [
            (nx * basis.right[0] + nz * basis.forward[0]) * speed,
            (nx * basis.right[1] + nz * basis.forward[1]) * speed,
            (nx * basis.right[2] + nz * basis.forward[2]) * speed,
        ];
        player.nudge(d)
    }

    pub fn ascend(&self, delta: f32) -> Option<()> {
        let session = self.session.lock().unwrap();
        let player = &session.as_ref()?.0;
        let cur = player.height()?;
        let new_target = cur + delta * 3.0; // matches the old +1.5 for delta=0.5
        player.adjust_height(delta)?;
        *self.target_height.lock().unwrap() = Some(new_target);
        Some(())
    }

    pub fn descend(&self, delta: f32) -> Option<()> {
        let session = self.session.lock().unwrap();
        let player = &session.as_ref()?.0;
        let cur = player.height()?;
        let new_target = cur - delta;
        player.adjust_height(-delta)?;
        *self.target_height.lock().unwrap() = Some(new_target);
        Some(())
    }

    pub fn maintain(&self) -> Option<()> {
        let target = (*self.target_height.lock().unwrap())?;
        let session = self.session.lock().unwrap();
        let player = &session.as_ref()?.0;

        let current = player.height()?;
        if current < target {
            player.set_height(target)?;
        }
        if self.gravity_locked.load(Ordering::Relaxed) {
            let a = player.acceleration()?;
            if a.abs() > 0.01 {
                player.set_acceleration(0.0)?;
            }
        }
        Some(())
    }

    pub fn debug_camera_yaw(&self) -> Option<f32> {
        let session = self.session.lock().unwrap();
        let camera = &session.as_ref()?.1;
        camera.yaw()
    }
}
