use platformer_rust::player;
use std::time::Instant;

use raylib::prelude::*;

fn main() {
    let mut last_frame_time = Instant::now();
    let mut animation_elapsed_time = 0.as_f32();

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let player = player::Player::new(&mut rl, &thread);

    let mut sprite_index = 0;
    let mut frame_counter = 0;
    // number of frames showb by second
    let frame_speed = 8;

    rl.set_target_fps(60);

    // Detect window close button or ESC key
    while !rl.window_should_close() {
        // delta_time: TODO some recommend using ticks instead
        let current_frame_time = Instant::now();
        let delta_duration = current_frame_time - last_frame_time;
        // delta time in game engines are in seconds
        let delta_time = delta_duration.as_secs_f32();
        last_frame_time = current_frame_time;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        animation_elapsed_time = animation_elapsed_time + delta_time;
        println!("{}", animation_elapsed_time);

        frame_counter = frame_counter + 1;

        if frame_counter > (60 / frame_speed) {
            frame_counter = 0;
            sprite_index = sprite_index + 1;
            if sprite_index > 9 {
                sprite_index = 0;
            }
        }

        let frame_rec = Rectangle {
            x: sprite_index.as_f32() * player.width,
            y: 0.,
            width: player.width,
            height: player.height,
        };

        let frame_rec_right = Rectangle {
            x: sprite_index.as_f32() * player.width,
            y: 0.,
            // negative width to flip the sprite
            width: -player.width,
            height: player.height,
        };

        let player_pos = Vector2 { x: 120., y: 120. };
        let player_pos2 = Vector2 { x: 120., y: 240. };

        d.draw_texture_rec(&player.texture, frame_rec, player_pos, Color::WHITE);
        d.draw_texture_rec(&player.texture, frame_rec_right, player_pos2, Color::WHITE);
    }
}
