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

const GROUND_HEIGHT: f32 = 400.0;
const JUMP_VELOCITY_Y: f32 = -300.0;
const GRAVITY: f32 = 500.0;
const PLAYER_SPEED: f32 = 300.0;
const GROUND_DRAG: f32 = 100.0;
const AIR_DRAG: f32 = 50.0;

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
    last_facing: LastFacing,
    velocity_y: f32,
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
                y: GROUND_HEIGHT,
            },
            speed: PLAYER_SPEED,
            animation_run_right,
            animation_run_left,
            animation_idle,
            animation_jump_left,
            animation_jump_right,
            movement: Movement::Idle,
            is_jumping: false,
            last_facing: LastFacing::Right,
            velocity_y: 0.0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        if self.is_jumping {
            self.movement = match self.last_facing {
                LastFacing::Left => Movement::JumpLeft,
                LastFacing::Right => Movement::JumpRight,
            };
            // self.jump(delta_time);
        }

        if self.is_on_ground() {
            let speed = self.speed - GROUND_DRAG;
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.position.x = self.position.x + speed * delta_time;
                self.movement = Movement::Right;
                self.last_facing = LastFacing::Right;
            } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                self.position.x = self.position.x - speed * delta_time;
                self.movement = Movement::Left;
                self.last_facing = LastFacing::Left;
            } else {
                self.movement = Movement::Idle;
            }

            if rl.is_key_down(KeyboardKey::KEY_SPACE) {
                self.is_jumping = true;
                self.movement = match self.last_facing {
                    LastFacing::Left => Movement::JumpLeft,
                    LastFacing::Right => Movement::JumpRight,
                }
            }
        } else {
            let speed = self.speed - AIR_DRAG;
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                self.position.x = self.position.x + speed * delta_time;
                self.movement = Movement::JumpRight;
                self.last_facing = LastFacing::Right;
            } else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                self.position.x = self.position.x - speed * delta_time;
                self.movement = Movement::JumpLeft;
                self.last_facing = LastFacing::Left;
            }
        }

        self.apply_physics(delta_time);
    }

    fn apply_physics(&mut self, delta_time: f32) {
        if self.is_jumping {
            self.velocity_y = JUMP_VELOCITY_Y;
            self.is_jumping = false;
        } else if self.is_on_ground() {
            self.velocity_y = 0.0;
            self.is_jumping = false;
        } else {
            self.velocity_y = self.velocity_y + GRAVITY * delta_time;
        }
        self.position.y = self.position.y + self.velocity_y * delta_time;
    }

    fn is_on_ground(&self) -> bool {
        self.position.y >= GROUND_HEIGHT
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
