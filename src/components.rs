use std::collections::VecDeque;

use macroquad::prelude::*;
use num_complex::Complex;

use crate::{math::ToVec2 as _, time::Timer};

pub struct Player;
pub struct Controllable;
pub struct Enemy;
pub struct Boss;
pub struct Text(pub String);
pub struct Bullet;
pub struct DieOffScreen;
#[derive(Debug, Clone)]
pub struct BeenOnScreen(pub bool);
#[derive(Debug, Clone)]
pub struct Cooldown(pub Timer);

impl Cooldown {
    pub fn new(time: f32) -> Self {
        Self(Timer::new(time, true))
    }
}

pub struct HealthBar(pub Rect);

// It uses [`Resources`] sprite
#[derive(Debug)]
pub struct ParallaxBackground(pub Sprite);

// It uses [`Resources`] sprite
#[derive(Debug, Clone)]
pub struct Sprite(pub Rect);

impl Sprite {
    // INFO : You can't change this, it's depend on the sprite atlas
    const WIDTH: u32 = 16;
    const HEIGHT: u32 = 16;

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self(Rect::new(x, y, width, height))
    }

    pub fn new_from_index(x: u32, y: u32) -> Self {
        Self(Rect::new(
            (x * Self::WIDTH) as f32,
            (y * Self::HEIGHT) as f32,
            Self::WIDTH as f32,
            Self::HEIGHT as f32,
        ))
    }
}
pub struct Movement {
    timer: Timer,
    movement: Move,
}

impl Movement {
    pub fn new(how_long: f32, movement: Move) -> Self {
        Self {
            timer: Timer::new(how_long, false),
            movement,
        }
    }
}

pub struct Moves(pub VecDeque<Movement>);

impl Moves {
    pub fn new(vec: impl Into<VecDeque<Movement>>) -> Self {
        Self(vec.into())
    }

    pub fn update(&mut self, move_params: &MoveParams, position: &Transform2D) -> MoveParams {
        if let Some(current) = self.0.front_mut() {
            current.timer.update();

            if !current.timer.completed() {
                return match current.movement {
                    Move::MoveNext(a, b) => MoveParams::move_next(position.position, a, b),
                    Move::MoveFromToward(target, attraction) => {
                        MoveParams::move_from_towards(position.position, target, attraction)
                    }
                    Move::MoveTowardExp(target, attraction, exponent) => {
                        MoveParams::move_towards_exp(
                            move_params.velocity,
                            target,
                            attraction,
                            exponent,
                        )
                    }
                    Move::MoveFromTowardExp(target, attraction, exponent) => {
                        MoveParams::move_from_towards_exp(
                            position.position,
                            target,
                            attraction,
                            exponent,
                        )
                    }
                    Move::MoveTowards(target, attraction) => {
                        MoveParams::move_towards(move_params.velocity, target, attraction)
                    }
                    Move::MoveLinear(vel) => MoveParams::move_linear(vel),
                    Move::MoveAccelerated(vel, accel) => MoveParams::move_accelerated(vel, accel),
                    Move::MoveDampen(vel, retention) => MoveParams::move_dampen(vel, retention),
                    Move::MoveDampenRetention(retention) => {
                        MoveParams::move_dampen(move_params.velocity, retention)
                    }
                    Move::MoveAccelerated2(accel) => {
                        MoveParams::move_accelerated(move_params.velocity, accel)
                    }
                };
            }

            self.0.pop_front();
            return *move_params;
        }

        return *move_params;
    }
}

#[derive(Debug, Clone)]
pub struct BulletSetup(pub Sprite);

#[derive(Debug, Clone)]
pub enum AttackMove {
    AtPlayer {
        num: u16,
        speed: f32,
        spread: f32,
        total_shoot: u16,
        setup: BulletSetup,
    },
    Multiple(Vec<AttackMove>),
}

pub enum Move {
    MoveNext(MoveParams, f32),
    MoveFromToward(Complex<f32>, Complex<f32>),
    MoveTowardExp(Complex<f32>, Complex<f32>, f32),
    MoveFromTowardExp(Complex<f32>, Complex<f32>, f32),
    MoveTowards(Complex<f32>, Complex<f32>),
    MoveLinear(Complex<f32>),
    MoveAccelerated(Complex<f32>, Complex<f32>),
    MoveAccelerated2(Complex<f32>),
    MoveDampen(Complex<f32>, f32),
    MoveDampenRetention(f32),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MoveParams {
    pub velocity: Complex<f32>,
    pub acceleration: Complex<f32>,
    pub retention: f32,
    pub attraction: Complex<f32>,
    pub attraction_point: Complex<f32>,
    pub attraction_exponent: f32,
}
impl MoveParams {
    pub fn update(&mut self, pos: &mut Complex<f32>, delta: f32) -> Complex<f32> {
        let orig_velocity = self.velocity;
        *pos += orig_velocity * delta;

        self.velocity = self.acceleration * delta + self.retention * self.velocity;

        if self.attraction.norm() != 0.0 {
            let av = self.attraction_point - *pos;

            if self.attraction_exponent == 1.0 {
                self.velocity += self.attraction * av * delta;
            } else {
                let m = av.norm().powf(self.attraction_exponent - 0.5);
                self.velocity += self.attraction * av * m * delta;
            }
        }

        orig_velocity
    }

    pub fn move_next(
        pos: Complex<f32>,
        mut move_params: MoveParams,
        delta_time: f32,
    ) -> MoveParams {
        move_params.update(&mut Complex::new(pos.re, pos.im), delta_time);
        move_params
    }

    pub fn move_asymptotic(
        vel0: Complex<f32>,
        vel1: Complex<f32>,
        retention: Complex<f32>,
    ) -> MoveParams {
        MoveParams {
            velocity: vel0,
            acceleration: vel1 * (Complex::new(1.0, 0.0) - retention),
            retention: retention.re,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }

    pub fn move_asymptotic_halflife(
        vel0: Complex<f32>,
        vel1: Complex<f32>,
        halflife: f32,
    ) -> MoveParams {
        let retention = Complex::new(2.0_f32.powf(-1.0 / halflife), 0.0);
        Self::move_asymptotic(vel0, vel1, retention)
    }

    pub fn move_asymptotic_simple(vel: Complex<f32>, boost_factor: f32) -> MoveParams {
        let retention = 0.8;
        Self::move_asymptotic(
            vel * (Complex::new(1.0 + boost_factor, 0.0)),
            vel,
            Complex::new(retention, 0.0),
        )
    }

    pub fn move_from_towards(
        origin: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
    ) -> MoveParams {
        let towards_move = Self::move_towards(Complex::new(0.0, 0.0), target, attraction);
        Self::move_next(origin, towards_move, 0.0)
    }

    pub fn move_towards_exp(
        vel: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
        exponent: f32,
    ) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction,
            attraction_point: target,
            attraction_exponent: exponent,
        }
    }

    pub fn move_from_towards_exp(
        origin: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
        exponent: f32,
    ) -> MoveParams {
        let towards_exp_move =
            Self::move_towards_exp(Complex::new(0.0, 0.0), target, attraction, exponent);
        Self::move_next(origin, towards_exp_move, 0.0)
    }

    pub fn move_dampen(vel: Complex<f32>, retention: f32) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }

    pub fn move_towards(
        vel: Complex<f32>,
        target: Complex<f32>,
        attraction: Complex<f32>,
    ) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction,
            attraction_point: target,
            attraction_exponent: 1.0,
        }
    }

    pub fn move_linear(vel: Complex<f32>) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: Complex::new(0.0, 0.0),
            retention: 1.0,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }
    pub fn move_accelerated(vel: Complex<f32>, accel: Complex<f32>) -> MoveParams {
        MoveParams {
            velocity: vel,
            acceleration: accel,
            retention: 1.0,
            attraction: Complex::new(0.0, 0.0),
            attraction_point: Complex::new(0.0, 0.0),
            attraction_exponent: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform2D {
    pub position: Complex<f32>,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Transform2D {
    pub fn new(position: Complex<f32>, scale: Vec2, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn position(&self) -> &Complex<f32> {
        &self.position
    }

    pub fn scale(&self) -> &Vec2 {
        &self.scale
    }

    pub fn rotation(&self) -> &f32 {
        &self.rotation
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hitbox {
    pub radius: f32,
}

impl Hitbox {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn is_intersect(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &Self,
    ) -> bool {
        let distance_squared = vec2(current_pos.position().re, current_pos.position().im)
            .distance_squared(vec2(target_pos.position().re, target_pos.position().im));
        let sum_of_radii_squared = (self.radius + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }
    pub fn near(
        &self,
        current_pos: &Transform2D,
        target_pos: &Transform2D,
        target_hitbox: &Self,
    ) -> bool {
        let distance_squared = current_pos
            .position()
            .to_vec2()
            .distance_squared(target_pos.position().to_vec2());
        let sum_of_radii_squared = (self.radius + 0.05 + target_hitbox.radius).powi(2);
        distance_squared <= sum_of_radii_squared
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hitpoint {
    pub hp: f32,
    pub max_hp: f32,
    pub invulnerable: bool, // INFO : Phase for invulnerable stuff
}

impl Hitpoint {
    pub fn new(hp: f32) -> Self {
        Self {
            hp,
            max_hp: hp,
            invulnerable: false,
        }
    }

    pub fn is_dead(&self) -> bool {
        if self.invulnerable {
            return false;
        }

        return self.hp < 0.;
    }

    pub fn damage(&mut self, damage: f32) -> bool {
        if !self.invulnerable {
            self.hp -= damage;
            return self.hp <= 0.0;
        }

        false
    }
}
