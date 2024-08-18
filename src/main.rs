use platformer_rust::animation;
use std::time::Instant;

use raylib::prelude::*;

fn main() {
    let mut last_frame_time = Instant::now();
    let mut animation_elapsed_time = 0.as_f32();

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let mut player_animation_run_left = animation::Animation::new(
        &mut rl,
        &thread,
        "./ressources/Sprites/Player/Run.png",
        9,
        animation::Flip::No,
    );
    let mut player_animation_run_right = animation::Animation::new(
        &mut rl,
        &thread,
        "./ressources/Sprites/Player/Run.png",
        9,
        animation::Flip::Yes,
    );

    let mut player_animation_idle = animation::Animation::new(
        &mut rl,
        &thread,
        "./ressources/Sprites/Player/Idle.png",
        1,
        animation::Flip::No,
    );

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

        player_animation_run_left.play(&mut d, Vector2 { x: 100.0, y: 100.0 });
        player_animation_run_right.play(&mut d, Vector2 { x: 100.0, y: 200.0 });
        player_animation_idle.play(&mut d, Vector2 { x: 100.0, y: 300.0 });
    }
}
