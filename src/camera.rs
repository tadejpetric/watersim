pub fn camera_change(keycode: Option<sdl2::keyboard::Keycode>) -> (glm::Vec3, glm::Vec3) {
    let mut position_update = glm::Vec3::new(0.0, 0.0, 0.0);
    let mut direction_update = glm::Vec3::new(0.0, 0.0, 0.0);

    match keycode {
        // This should probably manage a larger state machine, so it can handle
        // exiting the program and changing other parameters. Fine for now.
        //Some(sdl2::keyboard::Keycode::Escape) => running = false,
        Some(sdl2::keyboard::Keycode::A) => {
            position_update.x += 1.0;
        }
        Some(sdl2::keyboard::Keycode::D) => {
            position_update.x -= 1.0;
        }
        Some(sdl2::keyboard::Keycode::S) => {
            position_update.y += 1.0;
        }
        Some(sdl2::keyboard::Keycode::W) => {
            position_update.y -= 1.0;
        }
        Some(sdl2::keyboard::Keycode::Q) => {
            position_update.z += 1.0;
        }
        Some(sdl2::keyboard::Keycode::E) => {
            position_update.z -= 1.0;
        }
        Some(sdl2::keyboard::Keycode::J) => {
            direction_update.x += 1.0;
        }
        Some(sdl2::keyboard::Keycode::L) => {
            direction_update.x -= 1.0;
        }
        Some(sdl2::keyboard::Keycode::I) => {
            direction_update.y += 1.0;
        }
        Some(sdl2::keyboard::Keycode::K) => {
            direction_update.y -= 1.0;
        }
        Some(sdl2::keyboard::Keycode::U) => {
            direction_update.z += 1.0;
        }
        Some(sdl2::keyboard::Keycode::O) => {
            direction_update.z -= 1.0;
        }
        _ => {}
    }
    (position_update, direction_update)
}
