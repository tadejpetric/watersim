// https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs
// Modified to only use SDL2 and glow

use glow::*;
mod config;
mod shader_utils;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    std::debug_assert!(args.len() == 2);
    let config_fn = &args[1];
    let config = config::read_config(config_fn);
    println!("{:?}", config);

    unsafe {
        // Create a context from a sdl2 window
        let (gl, window, mut events_loop, _context) = {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
            let window = video
                .window("Hello triangle!", 1024, 769)
                .opengl()
                .resizable()
                .build()
                .unwrap();
            let gl_context = window.gl_create_context().unwrap();
            let gl =
                glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
            let event_loop = sdl.event_pump().unwrap();
            (gl, window, event_loop, gl_context)
        };

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        // Attach shaders to the program
        let shader_sources = [
            shader_utils::read_shader(config.shader_dir.join( "shader.vert"), glow::VERTEX_SHADER),
            shader_utils::read_shader(config.shader_dir.join( "shader.frag"), glow::FRAGMENT_SHADER),
        ];

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

        // Run the program.
        gl.use_program(Some(program));

        let mut time: f32 = 0.0;
        let uniform_location = gl.get_uniform_location(program, "time");
        // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
        gl.uniform_1_f32(uniform_location.as_ref(), time);


        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        {
            let mut running = true;
            while running {
                {
                    for event in events_loop.poll_iter() {
                        match event {
                            sdl2::event::Event::Quit { .. } => running = false,
                            _ => {}
                        }
                    }
                }

                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.draw_arrays(glow::TRIANGLES, 0, 3);
                window.gl_swap_window();
                time += 0.1;
                gl.uniform_1_f32(uniform_location.as_ref(), time);
                if !running {
                    gl.delete_program(program);
                    gl.delete_vertex_array(vertex_array);
                }
            }
        }
    }
}
