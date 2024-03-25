// https://github.com/grovesNL/glow/blob/main/examples/hello/src/main.rs
// Modified to only use SDL2 and glow

use glow::*;
extern crate nalgebra_glm as glm;
use rand::Rng;

mod camera;
mod config;
mod opengl_utils;
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
        let (gl, window, mut events_loop, _context) = opengl_utils::create_context();

        let program = gl.create_program().expect("Cannot create program");

        // Attach shaders to the program
        let shader_sources = [
            shader_utils::read_shader(config.shader_dir.join("shader.vert"), glow::VERTEX_SHADER),
            shader_utils::read_shader(config.shader_dir.join("shader.frag"), glow::FRAGMENT_SHADER),
        ];

        opengl_utils::attach_shaders(&gl, &shader_sources, program);

        let mut camera = camera::create_camera(
            glm::Vec3::new(0.0402, -0.93128, 0.83396),
            glm::Vec3::new(-0.01503, 0.722822, -0.69087),
        );

        // Run the program.
        gl.use_program(Some(program));

        // Add uniforms.
        let mut time: f32 = 0.0;
        let time_location = gl.get_uniform_location(program, "time");
        gl.uniform_1_f32(time_location.as_ref(), time);

        let camera_matrix = camera.look_at();
        let camera_location = gl.get_uniform_location(program, "camera");
        gl.uniform_matrix_4_f32_slice(
            camera_location.as_ref(),
            false,
            &mat4_to_array(camera_matrix),
        );

        let perspective = glm::perspective(1.0, 1.0, 0.1, 1000.0);
        let perspective_location = gl.get_uniform_location(program, "perspective");
        gl.uniform_matrix_4_f32_slice(
            perspective_location.as_ref(),
            false,
            &mat4_to_array(perspective),
        );

        let (vbo, vao) = create_vertex_buffer(&gl, config.grid_size, config.scale);

        // Background colour.
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        {
            let mut running = true;
            while running {
                running = opengl_utils::process_keyboard_events(&mut camera, &mut events_loop);

                // Update uniforms.
                gl.uniform_matrix_4_f32_slice(
                    camera_location.as_ref(),
                    false,
                    &mat4_to_array(camera.look_at()),
                );

                time += config.speed;
                gl.uniform_1_f32(time_location.as_ref(), time);

                // Draw the scene.
                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.draw_arrays(
                    glow::TRIANGLES,
                    0,
                    12 * (config.grid_size * config.grid_size) as i32,
                );
                window.gl_swap_window();

                // Cleanup.
                if !running {
                    gl.delete_program(program);
                    gl.delete_vertex_array(vao);
                    gl.delete_buffer(vbo);
                }
            }
        }
    }
}
