use std::path::PathBuf;

pub fn read_shader(shader_fn: PathBuf, shader_type: u32) -> (String, u32) {
    use std::fs::read_to_string;
    (read_to_string(shader_fn).unwrap(), shader_type)
}