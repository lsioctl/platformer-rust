use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    rl.set_target_fps(60);

    let player_text = rl
        .load_texture(&thread, "./ressources/Sprites/Player/Run.png")
        .unwrap();

    let player_height = player_text.height.as_f32();
    // square frame
    let player_width = player_height.as_f32();

    let mut sprite_index = 0;
    let mut frame_counter = 0;
    // number of frames showb by second
    let frame_speed = 8;

    // Detect window close button or ESC key
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        frame_counter = frame_counter + 1;

        if frame_counter > (60 / frame_speed) {
            frame_counter = 0;
            sprite_index = sprite_index + 1;
            if sprite_index > 9 {
                sprite_index = 0;
            }
        }

        let frame_rec = Rectangle {
            x: sprite_index.as_f32() * player_width,
            y: 0.,
            width: player_width,
            height: player_height,
        };

        let player_pos = Vector2 { x: 120., y: 120. };

        d.draw_texture_rec(&player_text, frame_rec, player_pos, Color::WHITE);
    }
}
