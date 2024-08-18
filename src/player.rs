use raylib::prelude::*;

pub struct Player {
    pub position: Vector2,
    pub texture: Texture2D,
    pub height: f32,
    pub width: f32,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Player {
        let texture = rl
            .load_texture(&thread, "./ressources/Sprites/Player/Run.png")
            .unwrap();

        let height = texture.height.as_f32();
        let width = height;

        return Player {
            position: Vector2 { x: 0., y: 0. },
            texture,
            height,
            width,
        };
    }
}
