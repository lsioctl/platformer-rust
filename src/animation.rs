use raylib::prelude::*;

pub enum Flip {
    Yes,
    No,
}

pub struct Animation {
    texture: Texture2D,
    flip: Flip,
    sprite_count: u32,
    height: f32,
    width: f32,
    sprite_index: u32,
    frame_counter: u32,
    // number of frames showb by second
    frame_speed: u32,
    is_playing: bool,
}

impl Animation {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        filename: &str,
        sprite_count: u32,
        flip: Flip,
    ) -> Self {
        let texture = rl.load_texture(&thread, filename).unwrap();

        let height = texture.height.as_f32();
        let width = height;

        let sprite_index = 0;
        let frame_counter = 0;
        let frame_speed = 8;

        let is_playing = false;

        Self {
            texture,
            flip,
            height,
            width,
            sprite_index,
            frame_counter,
            frame_speed,
            is_playing,
            sprite_count,
        }
    }

    pub fn play(&mut self, d: &mut RaylibDrawHandle, position: Vector2) {
        // reset state for a new play
        if self.is_playing == false {
            self.is_playing = true;
            self.sprite_index = 0;
            self.frame_counter = 0;
        }

        self.frame_counter = self.frame_counter + 1;

        if self.frame_counter > (60 / self.frame_speed) {
            self.frame_counter = 0;
            self.sprite_index = self.sprite_index + 1;
            if self.sprite_index > self.sprite_count - 1 {
                self.sprite_index = 0;
            }
        }

        let frame_rec = Rectangle {
            x: self.sprite_index.as_f32() * self.width,
            y: 0.,
            width: match self.flip {
                Flip::No => self.width,
                // negative width to flip the sprite
                Flip::Yes => -self.width,
            },
            height: self.height,
        };

        d.draw_texture_rec(&self.texture, frame_rec, position, Color::WHITE);
    }
}
