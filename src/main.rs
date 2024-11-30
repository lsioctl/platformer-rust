use platformer_rust::background;
use platformer_rust::player;
use std::time::Instant;

use raylib::prelude::*;

fn main() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 480;
    let mut last_frame_time = Instant::now();

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let mut player = player::Player::new(&mut rl, &thread);
    let mut background = background::Background::new(&mut rl, &thread, player.position);

    let mut camera = Camera2D {
        // offset: Vector2 { x: 0.0, y: 0.0 },
        offset: Vector2 {
            x: SCREEN_WIDTH as f32 / 2.0,
            y: SCREEN_HEIGHT as f32 / 2.0,
        },
        rotation: 0.0,
        target: player.position,
        zoom: 1.0,
    };

    rl.set_target_fps(60);

    // Detect window close button or ESC key
    while !rl.window_should_close() {
        // delta_time: TODO some recommend using ticks instead
        let current_frame_time = Instant::now();
        let delta_duration = current_frame_time - last_frame_time;
        // delta time in game engines are in seconds
        let delta_time = delta_duration.as_secs_f32();
        last_frame_time = current_frame_time;

        // camera.target = player.position;
        player.update(&rl, delta_time);
        background.update(player.position);
        camera.target.x = player.position.x;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        background.draw(&mut d);
        // player.draw(&mut d);

        {
            let mut d = d.begin_mode2D(camera);
            player.draw(&mut d);
        }
    }
}
