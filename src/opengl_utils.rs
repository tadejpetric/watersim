use glow::NativeProgram;
use glow::*;

use crate::camera;

pub unsafe fn create_context() -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    let window = video
        .window("Watersim", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
    let event_loop = sdl.event_pump().unwrap();
    gl.enable(glow::CULL_FACE);
    gl.cull_face(glow::BACK);
    (gl, window, event_loop, gl_context)
}

pub unsafe fn attach_shaders(
    gl: &glow::Context,
    shader_sources: &[(String, u32)],
    program: NativeProgram,
) {
    let mut shaders = Vec::with_capacity(shader_sources.len());
    for (shader_source, shader_type) in shader_sources.iter() {
        let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");
        gl.shader_source(shader, &shader_source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }
        gl.attach_shader(program, shader);
        shaders.push(shader);
    }
    // Link the program and cleanup the shaders
    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }

    for shader in shaders {
        gl.detach_shader(program, shader);
        gl.delete_shader(shader);
    }
}

pub fn process_keyboard_events(camera: &mut camera::Camera, events_loop: &mut sdl2::EventPump) -> bool {
    let mut running = true;
    for event in events_loop.poll_iter() {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => {
                if let Some(keycode) = keycode {
                    camera.camera_change(keycode);
                }
            }
            sdl2::event::Event::Quit { .. } => running = false,
            _ => {}
        }
    }
    running
}
