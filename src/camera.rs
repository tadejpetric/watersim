#[derive(Debug)]
pub struct Camera {
    // Do not manage roll.
    pub position: glm::Vec3,
    direction: glm::Vec3,
}

pub fn create_camera(position: glm::Vec3, direction: glm::Vec3) -> Camera {
    Camera {
        position: position.normalize(),
        direction,
    }
}

impl Camera {
    pub fn _update(&mut self, position_update: glm::Vec3, direction_update: glm::Vec3) {
        self.position += position_update;
        self.direction += direction_update;
    }
    pub fn look_at(&self) -> glm::Mat4 {
        let up = glm::Vec3::new(0.0, 0.0, 1.0);
        glm::look_at(&self.position, &(self.position + self.direction), &up)
    }
    pub fn camera_change(&mut self, keycode: sdl2::keyboard::Keycode) {
        let global_up = glm::Vec3::new(0.0, 0.0, 1.0);

        fn compute_up(direction: &glm::Vec3, global_up: &glm::Vec3) -> glm::Vec3 {
            glm::cross(&glm::cross(&direction, &global_up), &direction).normalize()
        }

        let position_update = match keycode {
            sdl2::keyboard::Keycode::W => self.direction,
            sdl2::keyboard::Keycode::S => -self.direction,
            sdl2::keyboard::Keycode::D => glm::cross(&self.direction, &global_up).normalize(),
            sdl2::keyboard::Keycode::A => -glm::cross(&self.direction, &global_up).normalize(),
            sdl2::keyboard::Keycode::E => compute_up(&self.direction, &global_up),
            sdl2::keyboard::Keycode::Q => -compute_up(&self.direction, &global_up),
            _ => glm::Vec3::new(0.0, 0.0, 0.0),
        };
        let position_speed = 0.05;
        self.position += position_update * position_speed;

        fn rotate_down(angle: f32, direction: &glm::Vec3) -> glm::Vec3 {
            let up = glm::Vec3::new(0.0, 0.0, 1.0);
            let right = glm::cross(&up, &direction).normalize();
            glm::rotate_vec3(&direction, angle, &right)
        }

        fn rotate_right(angle: f32, direction: &glm::Vec3) -> glm::Vec3 {
            let up = glm::Vec3::new(0.0, 0.0, 1.0);
            glm::rotate_vec3(&direction, angle, &up)
        }

        let rotation_speed = 0.03;
        self.direction = match keycode {
            sdl2::keyboard::Keycode::Up => rotate_down(rotation_speed, &self.direction),
            sdl2::keyboard::Keycode::Down => rotate_down(-rotation_speed, &self.direction),
            sdl2::keyboard::Keycode::Right => rotate_right(-rotation_speed, &self.direction),
            sdl2::keyboard::Keycode::Left => rotate_right(rotation_speed, &self.direction),
            _ => self.direction,
        };
    }
}
