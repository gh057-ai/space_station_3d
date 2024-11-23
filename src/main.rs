use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Space Station 3D")
        .build();

    // Enable mouse cursor lock for smoother camera rotation
    rl.disable_cursor();

    // Configure camera - start inside the room
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 1.5, 0.0), // position (eye level)
        Vector3::new(1.0, 1.5, 0.0), // looking towards window
        Vector3::new(0.0, 1.0, 0.0), // up
        75.0,                        // wider FOV for better indoor view
    );

    // Set target FPS
    rl.set_target_fps(60);

    // Movement speed
    let move_speed = 0.1;
    let look_speed = 0.003;
    let mut yaw = 0.0f32;  // Tracks total horizontal rotation

    while !rl.window_should_close() && !rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        // Mouse look
        let mouse_delta = rl.get_mouse_delta();
        yaw += mouse_delta.x * look_speed;

        // Calculate look direction (use raw yaw for continuous rotation)
        let look_dir = Vector3::new(yaw.cos(), 0.0, yaw.sin());
        camera.target = Vector3::new(
            camera.position.x + look_dir.x,
            camera.position.y,
            camera.position.z + look_dir.z,
        );

        // Basic movement
        if rl.is_key_down(KeyboardKey::KEY_W) {
            camera.position.x += look_dir.x * move_speed;
            camera.position.z += look_dir.z * move_speed;
            camera.target.x += look_dir.x * move_speed;
            camera.target.z += look_dir.z * move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            camera.position.x -= look_dir.x * move_speed;
            camera.position.z -= look_dir.z * move_speed;
            camera.target.x -= look_dir.x * move_speed;
            camera.target.z -= look_dir.z * move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            let right = Vector3::new(-look_dir.z, 0.0, look_dir.x);
            camera.position.x -= right.x * move_speed;
            camera.position.z -= right.z * move_speed;
            camera.target.x -= right.x * move_speed;
            camera.target.z -= right.z * move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            let right = Vector3::new(-look_dir.z, 0.0, look_dir.x);
            camera.position.x += right.x * move_speed;
            camera.position.z += right.z * move_speed;
            camera.target.x += right.x * move_speed;
            camera.target.z += right.z * move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            camera.position.y -= move_speed;
            camera.target.y -= move_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            camera.position.y += move_speed;
            camera.target.y += move_speed;
        }

        // Allow TAB key to toggle cursor lock
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            if rl.is_cursor_hidden() {
                rl.enable_cursor();
            } else {
                rl.disable_cursor();
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // 3D drawing
        {
            let mut d = d.begin_mode3D(camera);
            
            // Draw floor
            d.draw_plane(
                Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(6.0, 6.0),
                Color::GRAY,
            );
            
            // Draw ceiling
            d.draw_plane(
                Vector3::new(0.0, 3.0, 0.0),
                Vector2::new(6.0, 6.0),
                Color::GRAY,
            );

            // Draw walls (excluding window wall)
            // Back wall
            d.draw_cube(Vector3::new(0.0, 1.5, -3.0), 6.0, 3.0, 0.2, Color::LIGHTGRAY);
            // Left wall
            d.draw_cube(Vector3::new(-3.0, 1.5, 0.0), 0.2, 3.0, 6.0, Color::LIGHTGRAY);
            // Right wall
            d.draw_cube(Vector3::new(3.0, 1.5, 0.0), 0.2, 3.0, 6.0, Color::LIGHTGRAY);

            // Front wall with window
            // Bottom part
            d.draw_cube(Vector3::new(0.0, 0.5, 3.0), 6.0, 1.0, 0.2, Color::LIGHTGRAY);
            // Top part
            d.draw_cube(Vector3::new(0.0, 2.5, 3.0), 6.0, 1.0, 0.2, Color::LIGHTGRAY);
            // Left part
            d.draw_cube(Vector3::new(-2.0, 1.5, 3.0), 2.0, 1.0, 0.2, Color::LIGHTGRAY);
            // Right part
            d.draw_cube(Vector3::new(2.0, 1.5, 3.0), 2.0, 1.0, 0.2, Color::LIGHTGRAY);
            
            // Window (semi-transparent)
            d.draw_cube(Vector3::new(0.0, 1.5, 3.0), 2.0, 1.0, 0.1, Color::new(100, 149, 237, 100));
            d.draw_cube_wires(Vector3::new(0.0, 1.5, 3.0), 2.0, 1.0, 0.1, Color::DARKBLUE);

            // Draw some "stars" outside
            for z in 4..20 {
                let star_color = Color::new(255, 255, 255, (255 - z * 10) as u8);
                d.draw_sphere(Vector3::new(-2.0, 1.5, z as f32), 0.05, star_color);
                d.draw_sphere(Vector3::new(0.0, 2.0, z as f32), 0.05, star_color);
                d.draw_sphere(Vector3::new(2.0, 1.0, z as f32), 0.05, star_color);
            }
        }

        // Draw UI
        d.draw_fps(10, 10);
        d.draw_text(
            "Controls: WASD to move, QE for up/down, Mouse to look, TAB to toggle mouse, ESC to exit",
            10,
            30,
            20,
            Color::WHITE,
        );
    }
}
