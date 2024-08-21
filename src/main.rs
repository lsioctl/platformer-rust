use platformer_rust::player;
use std::time::Instant;

use raylib::prelude::*;

fn main() {
    let mut last_frame_time = Instant::now();

    let (mut rl, thread) = raylib::init().size(800, 480).title("Hello, World").build();

    let mut player = player::Player::new(&mut rl, &thread);

    rl.set_target_fps(60);

    let background = rl
        .load_texture(&thread, "./ressources/Backgrounds/Layer0_0.png")
        .unwrap();

    let background2 = rl
        .load_texture(&thread, "./ressources/Backgrounds/Layer1_0.png")
        .unwrap();

    let mut scrolling_back = 0.as_f32();
    let mut scrolling_fore = 0.as_f32();

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

        scrolling_back = scrolling_back - 0.1;
        scrolling_fore = scrolling_fore - 0.5;

        if scrolling_back <= -background.width.as_f32() {
            scrolling_back = 0.as_f32();
        };
        if scrolling_fore <= -background2.width.as_f32() {
            scrolling_fore = 0.as_f32();
        }

        d.clear_background(Color::WHITE);

        d.draw_texture_ex(
            &background,
            Vector2 {
                x: scrolling_back,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );
        d.draw_texture_ex(
            &background,
            Vector2 {
                x: background.width.as_f32() + scrolling_back,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );

        d.draw_texture_ex(
            &background2,
            Vector2 {
                x: scrolling_fore,
                y: 0.,
            },
            0.,
            1.0,
            Color::WHITE,
        );
        d.draw_texture_ex(
            &background2,
            Vector2 {
                x: background2.width.as_f32() + scrolling_fore,
                y: 0.,
            },
            0.,
            1.0,
            Color::WHITE,
        );

        player.draw(&mut d);
    }
}
