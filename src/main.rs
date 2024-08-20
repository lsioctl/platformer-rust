use platformer_rust::player;
use std::time::Instant;

use raylib::prelude::*;

fn main() {
    let mut last_frame_time = Instant::now();

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let mut player = player::Player::new(&mut rl, &thread);

    rl.set_target_fps(60);

    // Detect window close button or ESC key
    while !rl.window_should_close() {
        // delta_time: TODO some recommend using ticks instead
        let current_frame_time = Instant::now();
        let delta_duration = current_frame_time - last_frame_time;
        // delta time in game engines are in seconds
        let delta_time = delta_duration.as_secs_f32();
        last_frame_time = current_frame_time;

        player.update(&rl, delta_time);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        player.draw(&mut d);
    }
}
