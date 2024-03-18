// https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs
// Modified to only use SDL2 and glow

use glow::*;
extern crate nalgebra_glm as glm;
mod camera;
mod config;
mod shader_utils;

unsafe fn mat4_to_array(mat: glm::Mat4) -> [f32; 16] {
    std::mem::transmute::<[[f32; 4]; 4], [f32; 16]>(mat.into())
}

unsafe fn create_vertex_buffer(
    gl: &glow::Context,
    grid_size: u32,
    scale: f32,
) -> (NativeBuffer, NativeVertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec2s.
    // It represents a square grid of vertices, used as baseline for water.
    let mut plane_vertices: Vec<f32> = Vec::with_capacity((grid_size * grid_size * 12) as usize);
    for i in 0..grid_size {
        for j in 0..grid_size {
            // Calculate the coordinates of the top-left corner of the current square
            let x0 = j as f32 * scale;
            let y0 = i as f32 * scale;
            // Calculate other corners based on scale
            let x1 = x0 + scale;
            let y1 = y0 + scale;

            // First triangle
            plane_vertices.extend_from_slice(&[x0, y0, x1, y0, x0, y1]);
            // Second triangle
            plane_vertices.extend_from_slice(&[x0, y1, x1, y0, x1, y1]);
        }
    }
    let plane_vertices_u8: &[u8] = core::slice::from_raw_parts(
        plane_vertices.as_ptr() as *const u8,
        plane_vertices.len() * core::mem::size_of::<f32>(),
    );

    // Construct the buffer and upload the data.
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, plane_vertices_u8, glow::STATIC_DRAW);

    // Construct the vertex array to describe the format of the input buffer.
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

    (vbo, vao)
}

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
                .window("Hello triangle!", 960, 1050)
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

        let program = gl.create_program().expect("Cannot create program");

        // Attach shaders to the program
        let shader_sources = [
            shader_utils::read_shader(config.shader_dir.join("shader.vert"), glow::VERTEX_SHADER),
            shader_utils::read_shader(config.shader_dir.join("shader.frag"), glow::FRAGMENT_SHADER),
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

        let mut camera = camera::create_camera(
            glm::Vec3::new(0.0, 0.0, 1.0),
            glm::Vec3::new(0.1, 0.0, -0.7),
        );

        
        gl.enable(glow::CULL_FACE);
        gl.cull_face(glow::BACK);
        // Run the program.
        gl.use_program(Some(program));
        
        let mut time: f32 = 0.0;
        let time_location = gl.get_uniform_location(program, "time");
        // See also `uniform_n_i32`, `uniform_n_u32`, `uniform_matrix_4_f32_slice` etc.
        gl.uniform_1_f32(time_location.as_ref(), time);
        
        let camera_location = gl.get_uniform_location(program, "camera");
        let camera_matrix = camera.look_at();
        gl.uniform_matrix_4_f32_slice(
            camera_location.as_ref(),
            false,
            &mat4_to_array(camera_matrix),
        );
        
        let perspective = mat4_to_array(glm::perspective(1.0, 1.0, 0.01, 10.0));
        let perspective_location = gl.get_uniform_location(program, "perspective");
        gl.uniform_matrix_4_f32_slice(perspective_location.as_ref(), false, &perspective);

        let (vbo, vao) = create_vertex_buffer(&gl, config.grid_size, config.scale);

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        {
            let mut running = true;
            while running {
                let timer = std::time::Instant::now();
                {
                    for event in events_loop.poll_iter() {
                        match event {
                            sdl2::event::Event::KeyDown { keycode, .. } => {
                                println!("Key down: {:?}", keycode);
                                if let Some(keycode) = keycode {
                                    camera.camera_change(keycode);
                                }

                                println!("Camera {:?}", camera);
                                gl.uniform_matrix_4_f32_slice(
                                    camera_location.as_ref(),
                                    false,
                                    &mat4_to_array(camera.look_at()),
                                );
                            }
                            sdl2::event::Event::Quit { .. } => running = false,
                            _ => {}
                        }
                    }
                }

                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.draw_arrays(
                    glow::TRIANGLES,
                    0,
                    12 * (config.grid_size * config.grid_size) as i32,
                );
                window.gl_swap_window();
                time += 0.01;
                gl.uniform_1_f32(time_location.as_ref(), time);
                if !running {
                    gl.delete_program(program);
                    gl.delete_vertex_array(vao);
                    gl.delete_buffer(vbo);
                }

                //println!("Frame time: {} ms", timer.elapsed().as_millis());
            }
        }
    }
}
