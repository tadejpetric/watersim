use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub shader_dir: PathBuf,
    pub grid_size: u32,
    pub scale: f32,
    pub speed: f32,
    pub num_params: u32,
}

pub fn read_config(config_fn: &str) -> Config {
    use std::fs::read_to_string;

    let mut shader_dir: Option<PathBuf> = None;
    let mut grid_size: Option<u32> = None;
    let mut scale: Option<f32> = None;
    let mut speed: Option<f32> = None;
    let mut num_params: Option<u32> = None;

    for line in read_to_string(config_fn).unwrap().lines() {
        let mut parts = line.split_whitespace();
        let property = parts.next().unwrap();
        let value = parts.next().unwrap();
        std::debug_assert_eq!(parts.next(), None);

        match property {
            "shader_dir" => {
                std::debug_assert!(shader_dir.is_none(), "shader_dir provided twice in the config");
                shader_dir = Some(PathBuf::from(value));
            }
            "grid_size" => {
                std::debug_assert!(grid_size.is_none(), "grid_size provided twice in the config");
                grid_size = Some(value.parse().unwrap());
            }
            "scale" => {
                std::debug_assert!(scale.is_none(), "scale provided twice in the config");
                scale = Some(value.parse().unwrap());
            }
            "speed" => {
                std::debug_assert!(speed.is_none(), "speed provided twice in the config");
                speed = Some(value.parse().unwrap());
            }
            "num_params" => {
                std::debug_assert!(num_params.is_none(), "num_params provided twice in the config");
                num_params = Some(value.parse().unwrap());
            }
            _ => panic!("Unknown property: {}", property),
        }
    }
    Config {
        shader_dir: shader_dir.expect("shader_dir not provided in config"),
        grid_size: grid_size.expect("grid_size not provided in config"),
        scale: scale.expect("scale not provided in config"),
        speed: speed.expect("speed not provided in config"),
        num_params: num_params.expect("num_params not provided in config"),
    }
}
