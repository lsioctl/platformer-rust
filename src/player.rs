use raylib::prelude::*;

pub struct Player {
    pub position: Vector2,
    pub texture: Texture2D,
    pub height: f32,
    pub width: f32,
    sprite_index: u32,
    frame_counter: u32,
    // number of frames showb by second
    frame_speed: u32,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let texture = rl
            .load_texture(&thread, "./ressources/Sprites/Player/Run.png")
            .unwrap();

        let height = texture.height.as_f32();
        let width = height;

        let sprite_index = 0;
        let frame_counter = 0;
        let frame_speed = 8;

        Self {
            position: Vector2 { x: 0., y: 0. },
            texture,
            height,
            width,
            sprite_index,
            frame_counter,
            frame_speed,
        }
    }

    pub fn animate_left(&mut self, d: &mut RaylibDrawHandle) {
        self.frame_counter = self.frame_counter + 1;

        if self.frame_counter > (60 / self.frame_speed) {
            self.frame_counter = 0;
            self.sprite_index = self.sprite_index + 1;
            if self.sprite_index > 9 {
                self.sprite_index = 0;
            }
        }

        let frame_rec = Rectangle {
            x: self.sprite_index.as_f32() * self.width,
            y: 0.,
            width: self.width,
            height: self.height,
        };

        // let frame_rec_right = Rectangle {
        //     x: sprite_index.as_f32() * player.width,
        //     y: 0.,
        //     // negative width to flip the sprite
        //     width: -player.width,
        //     height: player.height,
        // };

        d.draw_texture_rec(
            &self.texture,
            frame_rec,
            Vector2 { x: 120., y: 120. },
            Color::WHITE,
        );
    }

    pub fn animate_right(&mut self, d: &mut RaylibDrawHandle) {
        self.frame_counter = self.frame_counter + 1;

        if self.frame_counter > (60 / self.frame_speed) {
            self.frame_counter = 0;
            self.sprite_index = self.sprite_index + 1;
            if self.sprite_index > 9 {
                self.sprite_index = 0;
            }
        }

        let frame_rec = Rectangle {
            x: self.sprite_index.as_f32() * self.width,
            y: 0.,
            // negative width to flip the sprite
            width: -self.width,
            height: self.height,
        };

        d.draw_texture_rec(
            &self.texture,
            frame_rec,
            Vector2 { x: 120., y: 240. },
            Color::WHITE,
        );
    }
}
