use crate::cmpx;
use crate::components::*;
use crate::math::*;
use crate::time::Timer;
use crate::vec2;
use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

pub fn create_boss_1() -> impl FnOnce(&mut World) {
    |world| {
        create_boss(
            world,
            Transform2D::new(cmpx!(0.5, -0.02), vec2!(0.1), 0.),
            Sprite::new_from_index(2, 0),
            MoveParams::move_linear(cmpx!(0., 0.1)),
            Moves::new(vec![]),
            AttackMove::Multiple(vec![]),
            Hitbox::new(0.05),
            Hitpoint::new(10.),
            Cooldown::new(1.),
        );
    }
}

pub fn create_boss(
    world: &mut World,
    transform: Transform2D,
    sprite: Sprite,
    movement: MoveParams,
    moves: Moves,
    attack_move: AttackMove,
    hitbox: Hitbox,
    hitpoint: Hitpoint,
    cooldown: Cooldown,
) {
    world.spawn((
        Enemy,
        Boss,
        transform,
        sprite,
        hitbox,
        movement,
        attack_move,
        moves,
        hitpoint,
        cooldown,
        HealthBar(Rect::new(0.04, 0.02, 0.9, 0.01)),
    ));
}

pub fn create_red_alien_wait(transform: Complex<f32>) -> impl FnOnce(&mut World) {
    move |world| {
        create_enemy(
            world,
            Transform2D::new(transform, vec2!(0.1), 0.),
            Sprite::new_from_index(1, 0),
            MoveParams::move_linear(cmpx!(0., 0.2)),
            Moves::new(vec![
                Movement::new(0.1, Move::MoveDampen(cmpx!(0., 0.8), 0.6)),
                Movement::new(1., Move::MoveDampen(cmpx!(0., 0.1), 0.55)),
                Movement::new(2., Move::MoveAccelerated2(cmpx!(0., 0.1))),
            ]),
            AttackMove::AtPlayer {
                num: 3,
                spread: 0.1,
                speed: 0.5,
                total_shoot: 1,
                setup: BulletSetup(Sprite::new_from_index(0, 1)),
            },
            Hitbox::new(0.03),
            Hitpoint::new(2.),
            Cooldown::new(1.),
        );
    }
}

pub fn create_red_alien(transform: Complex<f32>) -> impl FnOnce(&mut World) {
    move |world| {
        create_enemy(
            world,
            Transform2D::new(transform, vec2!(0.1), 0.),
            Sprite::new_from_index(1, 0),
            MoveParams::move_linear(cmpx!(0., 0.2)),
            Moves::new(vec![
                Movement::new(0.5, Move::MoveTowardExp(cmpx!(0., 1.), cmpx!(0.1), 0.1)),
                Movement::new(1.2, Move::MoveTowardExp(cmpx!(-1., 0.), cmpx!(-0.1), 0.1)),
            ]),
            AttackMove::AtPlayer {
                num: 1,
                spread: 2.,
                speed: 0.5,
                total_shoot: 1,
                setup: BulletSetup(Sprite::new_from_index(1, 1)),
            },
            Hitbox::new(0.03),
            Hitpoint::new(2.),
            Cooldown::new(1.),
        );
    }
}

pub fn create_enemy(
    world: &mut World,
    transform: Transform2D,
    sprite: Sprite,
    movement: MoveParams,
    moves: Moves,
    attack_move: AttackMove,
    hitbox: Hitbox,
    hitpoint: Hitpoint,
    cooldown: Cooldown,
) {
    world.spawn((
        Enemy,
        DieOffScreen,
        transform,
        sprite,
        hitbox,
        movement,
        attack_move,
        moves,
        hitpoint,
        cooldown,
    ));
}

pub fn create_enemy_bullet(
    world: &mut World,
    transform: Transform2D,
    sprite: Sprite,
    movement: MoveParams,
    hitbox: Hitbox,
) {
    world.spawn((
        Enemy,
        Bullet,
        DieOffScreen,
        movement,
        transform,
        sprite,
        hitbox,
    ));
}

pub fn create_player_generic_bullet(pos: Complex<f32>) -> impl FnOnce(&mut World) {
    move |world| {
        create_player_bullet(
            world,
            Transform2D {
                position: pos,
                scale: vec2!(0.05),
                rotation: 0.,
            },
            Sprite::new_from_index(0, 1),
            MoveParams::move_linear(cmpx!(0., -2.0)),
            Hitbox::new(0.01),
        )
    }
}

pub fn create_player_bullet(
    world: &mut World,
    transform: Transform2D,
    sprite: Sprite,
    movement: MoveParams,
    hitbox: Hitbox,
) {
    world.spawn((
        Player,
        Bullet,
        DieOffScreen,
        movement,
        transform,
        sprite,
        hitbox,
    ));
}

pub fn create_player(world: &mut World) {
    let offset = cmpx!(0.5, 0.8);

    world.spawn((
        Player,
        Controllable,
        Sprite::new_from_index(0, 0),
        Transform2D::new(offset, vec2!(0.1), 0.),
        MoveParams::move_dampen(cmpx!(0.), 0.85),
        Cooldown(Timer::new(0.1, true)),
        Hitbox::new(0.028),
    ));
}

pub fn stage_text(text: &str) -> impl FnOnce(&mut World) {
    let text = text.to_string();

    move |world| {
        world.spawn((
            DieOffScreen,
            Transform2D::new(cmpx!(0.5, 0.3), vec2!(0.05), 0.),
            Text(text),
            MoveParams::move_accelerated(cmpx!(0.), cmpx!(0., -0.1)),
        ));
    }
}
