use crate::animation;
use raylib::prelude::*;

#[derive(Debug)]
enum Movement {
    Right,
    Left,
    Idle,
    JumpRight,
    JumpLeft,
}

enum LastFacing {
    Right,
    Left,
}

const MAX_JUMP_TIME: f32 = 0.5;
const GROUND_HEIGHT: f32 = 400.0;
const JUMP_FORCE_Y: f32 = 10.0;
const GRAVITY: f32 = 200.0;

pub struct Player {
    pub position: Vector2,
    speed: f32,
    animation_run_left: animation::Animation,
    animation_run_right: animation::Animation,
    animation_idle: animation::Animation,
    animation_jump_left: animation::Animation,
    animation_jump_right: animation::Animation,
    movement: Movement,
    is_jumping: bool,
    jumptime: f32,
    last_facing: LastFacing,
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
            position: Vector2 {
                x: 100.,
                y: GROUND_HEIGHT - 300.0,
            },
            speed: 100.0,
            animation_run_right,
            animation_run_left,
            animation_idle,
            animation_jump_left,
            animation_jump_right,
            movement: Movement::Idle,
            is_jumping: false,
            jumptime: 0.0,
            last_facing: LastFacing::Right,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            if !self.is_jumping {
                self.is_jumping = true;
                self.jump(delta_time);
            }
        }

        if self.is_jumping {
            self.movement = match self.last_facing {
                LastFacing::Left => Movement::JumpLeft,
                LastFacing::Right => Movement::JumpRight,
            };
            self.jump(delta_time);
        }

        self.apply_physics(delta_time);

        if !self.is_jumping {
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.position.x = self.position.x + self.speed * delta_time;
                self.movement = Movement::Right;
                self.last_facing = LastFacing::Right;
            } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                self.position.x = self.position.x - self.speed * delta_time;
                self.movement = Movement::Left;
                self.last_facing = LastFacing::Left;
            } else {
                self.movement = Movement::Idle;
            }
        } else {
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.position.x = self.position.x + self.speed * delta_time;
                self.movement = Movement::JumpRight;
                self.last_facing = LastFacing::Right;
            } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                self.position.x = self.position.x - self.speed * delta_time;
                self.movement = Movement::JumpLeft;
                self.last_facing = LastFacing::Left;
            }
        }
    }

    fn apply_physics(&mut self, delta_time: f32) {
        if !self.is_on_ground() {
            self.position.y = self.position.y + GRAVITY * delta_time;
        }
    }

    fn is_on_ground(&self) -> bool {
        self.position.y >= GROUND_HEIGHT
    }

    fn jump(&mut self, delta_time: f32) {
        self.jumptime = self.jumptime + delta_time;
        println!("{}", self.jumptime);

        match self.jumptime {
            x if x < MAX_JUMP_TIME / 2.0 => {
                println!("less");
                self.position.y = self.position.y - JUMP_FORCE_Y;
            }
            x if x > MAX_JUMP_TIME => {
                println!("------------------------------------");
                self.is_jumping = false;
                self.jumptime = 0.0;
            }
            _ => {
                println!("more");
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        println!("{:?}", self.movement);
        match self.movement {
            Movement::Right => self.animation_run_right.play(d, self.position),
            Movement::Left => self.animation_run_left.play(d, self.position),
            Movement::Idle => self.animation_idle.play(d, self.position),
            Movement::JumpRight => self.animation_jump_right.play(d, self.position),
            Movement::JumpLeft => self.animation_jump_left.play(d, self.position),
        }
    }
}
