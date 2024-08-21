use crate::animation;
use raylib::prelude::*;

enum Movement {
    Right,
    Left,
    Idle,
    JumpRight,
    JumpLeft,
}

pub struct Player {
    pub position: Vector2,
    speed: f32,
    animation_run_left: animation::Animation,
    animation_run_right: animation::Animation,
    animation_idle: animation::Animation,
    animation_jump_left: animation::Animation,
    animation_jump_right: animation::Animation,
    movement: Movement,
}

impl Player {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // I do not want to store the texture in Player struct
        // Texture is small in size, and implements Copy trait
        // moreover I think it belongs to the animation
        let animation_run_left = animation::Animation::new(
            rl.load_texture(&thread, "./ressources/Sprites/Player/Run.png")
                .unwrap(),
            10,
            animation::Flip::No,
        );
        let animation_run_right = animation::Animation::new(
            rl.load_texture(&thread, "./ressources/Sprites/Player/Run.png")
                .unwrap(),
            10,
            animation::Flip::Yes,
        );

        let animation_idle = animation::Animation::new(
            rl.load_texture(&thread, "./ressources/Sprites/Player/Idle.png")
                .unwrap(),
            1,
            animation::Flip::No,
        );

        let animation_jump_left = animation::Animation::new(
            rl.load_texture(&thread, "./ressources/Sprites/Player/Jump.png")
                .unwrap(),
            11,
            animation::Flip::No,
        );

        let animation_jump_right = animation::Animation::new(
            rl.load_texture(&thread, "./ressources/Sprites/Player/Jump.png")
                .unwrap(),
            11,
            animation::Flip::Yes,
        );

        Self {
            position: Vector2 { x: 100., y: 100. },
            speed: 100.0,
            animation_run_right,
            animation_run_left,
            animation_idle,
            animation_jump_left,
            animation_jump_right,
            movement: Movement::Idle,
        }
    }
    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.position.x = self.position.x + self.speed * delta_time;
            self.movement = Movement::Right;
            if rl.is_key_down(KeyboardKey::KEY_SPACE) {
                self.movement = Movement::JumpRight
            }
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.position.x = self.position.x - self.speed * delta_time;
            self.movement = Movement::Left;
            if rl.is_key_down(KeyboardKey::KEY_SPACE) {
                self.movement = Movement::JumpLeft
            }
        } else {
            self.movement = Movement::Idle;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        match self.movement {
            Movement::Right => self.animation_run_right.play(d, self.position),
            Movement::Left => self.animation_run_left.play(d, self.position),
            Movement::Idle => self.animation_idle.play(d, self.position),
            Movement::JumpRight => self.animation_jump_right.play(d, self.position),
            Movement::JumpLeft => self.animation_jump_left.play(d, self.position),
        }
    }
}
