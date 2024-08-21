use raylib::prelude::*;

pub struct Background {
    texture_back: Texture2D,
    texture_front: Texture2D,
    scrolling_back: f32,
    scrolling_front: f32,
}

impl Background {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            texture_back: rl
                .load_texture(&thread, "./ressources/Backgrounds/Layer0_0.png")
                .unwrap(),
            texture_front: rl
                .load_texture(&thread, "./ressources/Backgrounds/Layer1_0.png")
                .unwrap(),
            scrolling_back: 0.0,
            scrolling_front: 0.0,
        }
    }

    // Draw backgound with parallax
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.scrolling_back = self.scrolling_back - 0.1;
        self.scrolling_front = self.scrolling_front - 0.5;

        if self.scrolling_back <= -self.texture_back.width.as_f32() {
            self.scrolling_back = 0.as_f32();
        };
        if self.scrolling_front <= -self.texture_front.width.as_f32() {
            self.scrolling_front = 0.as_f32();
        }

        d.draw_texture_ex(
            &self.texture_back,
            Vector2 {
                x: self.scrolling_back,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );
        d.draw_texture_ex(
            &self.texture_back,
            Vector2 {
                x: self.texture_back.width.as_f32() + self.scrolling_back,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );

        d.draw_texture_ex(
            &self.texture_front,
            Vector2 {
                x: self.scrolling_front,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );
        d.draw_texture_ex(
            &self.texture_front,
            Vector2 {
                x: self.texture_front.width.as_f32() + self.scrolling_front,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );
    }

    /// Draw backgound without parallax
    pub fn draw_static(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.texture_back, 0, 0, Color::WHITE);
        d.draw_texture(&self.texture_front, 0, 0, Color::WHITE);
    }
}
