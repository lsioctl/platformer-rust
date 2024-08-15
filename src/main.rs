use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let player_text = rl
        .load_texture(&thread, "./ressources/Sprites/Player/Run.png")
        .unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_circle(20, 20, 5., Color::BURLYWOOD);

        let frame_rec = Rectangle {
            x: 0.,
            y: 0.,
            width: 125.,
            height: 125.,
        };
        let player_pos = Vector2 { x: 120., y: 120. };

        d.draw_texture_rec(&player_text, frame_rec, player_pos, Color::WHITE);
    }
}
