#[derive(Debug)]
pub struct Config {
    pub shader_dir: String,
    pub grid_size: u32,
    pub scale: f32,
    pub speed: f32,
}

pub fn read_config(config_fn: &str) -> Config {
    use std::fs::read_to_string;

    let mut shader_dir: Option<String> = None;
    let mut grid_size: Option<u32> = None;
    let mut scale: Option<f32> = None;
    let mut speed: Option<f32> = None;

    for line in read_to_string(config_fn).unwrap().lines() {
        let mut parts = line.split_whitespace();
        let property = parts.next().unwrap();
        let value = parts.next().unwrap();
        std::debug_assert_eq!(parts.next(), None);

        match property {
            "shader_dir" => {
                std::debug_assert!(shader_dir.is_none());
                shader_dir = Some(value.to_string());
            }
            "grid_size" => {
                std::debug_assert!(grid_size.is_none());
                grid_size = Some(value.parse().unwrap());
            }
            "scale" => {
                std::debug_assert!(scale.is_none());
                scale = Some(value.parse().unwrap());
            }
            "speed" => {
                std::debug_assert!(speed.is_none());
                speed = Some(value.parse().unwrap());
            }
            _ => panic!("Unknown property: {}", property),
        }
    }
    Config {
        shader_dir: shader_dir.unwrap(),
        grid_size: grid_size.unwrap(),
        scale: scale.unwrap(),
        speed: speed.unwrap(),
    }
}
