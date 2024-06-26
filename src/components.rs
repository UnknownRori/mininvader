use std::collections::VecDeque;

use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    cmpx,
    math::{ComplexExt, ToComplex, ToVec2 as _},
    time::Timer,
    utils::rand_vec2,
    vec2,
};

pub struct Player;
pub struct Controllable;
pub struct Enemy;
pub struct Boss;
pub struct Bullet;
pub struct DieOffScreen;
#[derive(Debug, Clone)]
pub struct BeenOnScreen(pub bool);
#[derive(Debug, Clone)]
pub struct Cooldown(pub Timer);

pub struct Wanderable {
    last_position: Complex<f32>,
    target_position: Option<Complex<f32>>,
}

impl Wanderable {
    pub fn new(start_position: Complex<f32>) -> Self {
        Self {
            last_position: start_position,
            target_position: None,
        }
    }
}

pub enum Text {
    Left(String),
    Center(String),
    Right(String),
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        match self {
            Text::Left(str) => str,
            Text::Center(str) => str,
            Text::Right(str) => str,
        }
    }
}

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
#[derive(Debug, Clone, Copy)]
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

    pub fn update(
        &mut self,
        move_params: &MoveParams,
        position: &Transform2D,
        wanderable: Option<&mut Wanderable>,
    ) -> MoveParams {
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
                    Move::MoveWanderLinear(zone, velocity, wait) => {
                        let wanderable =
                            wanderable.expect("You should put Wanderable tag on the entity");

                        if wanderable.target_position.is_none() {
                            let mut tried = 8;
                            let mut new_target = rand_vec2(0., 1.);
                            while tried > 0 {
                                tried -= 1;

                                new_target = rand_vec2(0., 1.);
                                let x = zone.x + new_target.x * zone.w;
                                let y = zone.y + new_target.y * zone.h;
                                new_target = vec2!(x, y);

                                if new_target.distance_squared(position.position.to_vec2()) > 1.0 {
                                    break;
                                }
                            }

                            println!("{}", &new_target);

                            wanderable.target_position = Some(new_target.to_cmpx());

                            return MoveParams::move_linear(cmpx!(0.));
                        }

                        let vel =
                            position.position.dir(&wanderable.target_position.unwrap()) * velocity;
                        let move_params = MoveParams::move_towards(
                            vel,
                            wanderable.target_position.unwrap(),
                            cmpx!(0.),
                        );

                        if wanderable
                            .target_position
                            .unwrap()
                            .distance_squared(&position.position)
                            < 0.05
                        {
                            wanderable.target_position = None;
                        }

                        move_params
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
pub struct BossMove {
    pub timeout: Timer,
    pub hp: f32,
    pub attack: AttackMove,
}

impl BossMove {
    pub fn new(timeout: f32, hp: f32, attack: AttackMove) -> Self {
        Self {
            timeout: Timer::new(timeout, false),
            hp,
            attack,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BossMoves(pub VecDeque<BossMove>);

impl BossMoves {
    pub fn new(moves: impl Into<VecDeque<BossMove>>) -> Self {
        Self(moves.into())
    }
}

#[derive(Debug, Clone)]
pub enum AttackMove {
    AtPlayer {
        num: u16,
        speed: f32,
        spread: f32,
        total_shoot: u16,
        cooldown: Cooldown,
        setup: BulletSetup,
    },
    Circle {
        sides: u16,
        rotation_per_fire: f32,
        rotation: f32,
        cooldown: Cooldown,
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
    MoveWanderLinear(Rect, f32, f32),
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

    pub fn invulnerable() -> Self {
        Self {
            hp: 1.,
            max_hp: 1.,
            invulnerable: true,
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
