use raylib::prelude::*;

pub struct Background {
    texture_back_list: Vec<Texture2D>,
    texture_front: Texture2D,
    scrolling_back: f32,
    scrolling_front: f32,
    back_index: usize,
}

impl Background {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let texture_back0 = rl
            .load_texture(&thread, "./ressources/Backgrounds/Layer0_0.png")
            .unwrap();
        let texture_back1 = rl
            .load_texture(&thread, "./ressources/Backgrounds/Layer0_1.png")
            .unwrap();
        let texture_back2 = rl
            .load_texture(&thread, "./ressources/Backgrounds/Layer0_2.png")
            .unwrap();

        Self {
            texture_back_list: vec![texture_back0, texture_back1, texture_back2],
            texture_front: rl
                .load_texture(&thread, "./ressources/Backgrounds/Layer1_0.png")
                .unwrap(),
            scrolling_back: 0.0,
            scrolling_front: 0.0,
            back_index: 0,
        }
    }

    // Draw backgound with parallax
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.scrolling_back = self.scrolling_back - 0.5;
        // self.scrolling_front = self.scrolling_front - 0.5;
        // println!("{}", self.scrolling_back);

        if self.scrolling_back <= -self.texture_back_list[self.back_index].width.as_f32() {
            self.back_index = (self.back_index + 1) % 3;
            self.scrolling_back = 0.as_f32();
        };
        if self.scrolling_front <= -self.texture_front.width.as_f32() {
            self.scrolling_front = 0.as_f32();
        }

        d.draw_texture_ex(
            &self.texture_back_list[self.back_index],
            Vector2 {
                x: self.scrolling_back,
                y: 0.,
            },
            0.,
            1.,
            Color::WHITE,
        );
        d.draw_texture_ex(
            &self.texture_back_list[(self.back_index + 1) % 3],
            Vector2 {
                x: self.texture_back_list[self.back_index].width.as_f32() + self.scrolling_back,
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

    // /// Draw backgound without parallax
    // pub fn draw_static(&mut self, d: &mut RaylibDrawHandle) {
    //     d.draw_texture(&self.texture_back0, 0, 0, Color::WHITE);
    //     d.draw_texture(&self.texture_front, 0, 0, Color::WHITE);
    // }
}
