use crate::tools::game::Game;
use crate::tools::memory_mappings;
use crate::tools::noclip::NoClip;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::io::{self, Write};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// per-tick delta. tweak to taste.
const WASD_SPEED: f32 = 1.0;

pub fn boot() {
    let game = match Game::attach() {
        Some(g) => Arc::new(g),
        None => {
            println!(
                "process '{}' not found. is the game running?",
                memory_mappings::GOW_PROC_NAME
            );
            wait_for_enter();
            return;
        }
    };
    println!("Attached. GoWR.exe base = 0x{:X}", game.base);

    // yes, i used GPT to generate this text
    // sue me
    println!(
        "\n================== Controls =======================\n\
         [G]      -> toggle no-clip\n\
         [W]      -> move forward (camera-relative)\n\
         [S]      -> move backward\n\
         [A]      -> strafe left\n\
         [D]      -> strafe right\n\
         [LSHIFT] -> ascend\n\
         [LCTRL]  -> descend\n\
         [L]      -> lock gravity (freeze vertical accel)\n\
         [U]      -> unlock gravity\n\
         [X]      -> exit\n\
         ===================================================\n"
    );

    let noclip = Arc::new(NoClip::new(game.clone()));

    // watchdog: kill the tool when the game exits.
    thread::spawn(|| {
        loop {
            if !Game::is_still_alive() {
                println!(
                    "process '{}' exited. shutting down.",
                    memory_mappings::GOW_PROC_NAME
                );
                std::process::exit(0);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // movement / input thread.
    thread::spawn({
        let noclip = noclip.clone();
        move || movement_loop(noclip)
    });

    // main just parks; watchdog exits the process on game death, and X
    // in the movement loop exits on hotkey.
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn movement_loop(noclip: Arc<NoClip>) {
    let device_state = DeviceState::new();
    let mut g_was_down = false;
    let mut wasd_was_down = false;

    loop {
        let keys = device_state.get_keys();

        if keys.contains(&Keycode::X) {
            println!("[X] exiting.");
            std::process::exit(0);
        }

        // edge-triggered toggle on G
        if keys.contains(&Keycode::G) {
            if !g_was_down {
                let now_active = noclip.toggle();
                println!("no-clip {}", if now_active { "ON" } else { "OFF" });
                g_was_down = true;
            }
        } else {
            g_was_down = false;
        }

        if noclip.is_active() {
            let mut intent_x = 0.0;
            let mut intent_z = 0.0;
            if keys.contains(&Keycode::W) {
                intent_z += 0.05;
            }
            if keys.contains(&Keycode::S) {
                intent_z -= 0.05;
            }
            if keys.contains(&Keycode::D) {
                intent_x += 0.05;
            }
            if keys.contains(&Keycode::A) {
                intent_x -= 0.05;
            }

            let wasd_down = intent_x != 0.0 || intent_z != 0.0;
            if wasd_down {
                if !wasd_was_down {
                    if let Some(yaw) = noclip.debug_camera_yaw() {
                        println!(
                            "[wasd] yaw={:+.4} rad ({:+.1} deg) ix={:+.1} iz={:+.1}",
                            yaw,
                            yaw.to_degrees(),
                            intent_x,
                            intent_z,
                        );
                    }
                }
                let _ = noclip.move_wasd(intent_x, intent_z, WASD_SPEED);
            }
            wasd_was_down = wasd_down;

            if keys.contains(&Keycode::L) {
                noclip.set_gravity_locked(true);
            }
            if keys.contains(&Keycode::U) {
                noclip.set_gravity_locked(false);
            }
            if keys.contains(&Keycode::LShift) {
                let _ = noclip.ascend(0.5);
            }
            if keys.contains(&Keycode::LControl) {
                let _ = noclip.descend(0.5);
            }

            let _ = noclip.maintain();
        }

        thread::sleep(Duration::from_millis(5));
    }
}

fn wait_for_enter() {
    print!("press ENTER to exit...");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}
