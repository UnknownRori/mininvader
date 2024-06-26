use crate::cmpx;
use crate::components::*;
use crate::konst::VIRTUAL_STAGE_ASPECT_RATIO;
use crate::vec2;
use crate::{rect, time::Timer};
use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

pub fn create_boss_1() -> impl FnOnce(&mut World) {
    |world| {
        create_boss(
            world,
            Transform2D::new(cmpx!(0.5, -0.02), vec2!(0.1), 0.),
            Sprite::new_from_index(2, 0),
            MoveParams::move_linear(cmpx!(0., 0.2)),
            Moves::new(vec![
                Movement::new(0.1, Move::MoveDampen(cmpx!(0., 0.8), 0.6)),
                Movement::new(
                    50.,
                    Move::MoveWanderLinear(rect!(0.02, 0.02, 0.9, 0.45), 0.2, 1.),
                ),
            ]),
            BossMoves::new(vec![
                BossMove::new(
                    2.,
                    12.,
                    AttackMove::Multiple(vec![
                        AttackMove::AtPlayer {
                            num: 5,
                            speed: 0.5,
                            spread: 20.,
                            total_shoot: 2,
                            cooldown: Cooldown::new(1.),
                            setup: BulletSetup(Sprite::new_from_index(0, 1)),
                        },
                        AttackMove::Circle {
                            sides: 16,
                            rotation: 12.,
                            rotation_per_fire: 6.,
                            setup: BulletSetup(Sprite::new_from_index(0, 1)),
                            cooldown: Cooldown::new(1.),
                        },
                    ]),
                ),
                BossMove::new(
                    12.,
                    12.,
                    AttackMove::Multiple(vec![AttackMove::AtPlayer {
                        num: 5,
                        speed: 0.5,
                        spread: 20.,
                        total_shoot: 2,
                        cooldown: Cooldown::new(1.),
                        setup: BulletSetup(Sprite::new_from_index(0, 1)),
                    }]),
                ),
            ]),
            Hitbox::new(0.05 * VIRTUAL_STAGE_ASPECT_RATIO),
            Hitpoint::invulnerable(),
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
    attack_move: BossMoves,
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
        AttackMove::Multiple(vec![]),
        moves,
        hitpoint,
        cooldown,
        HealthBar(Rect::new(0.04, 0.02, 0.9, 0.01)),
        Wanderable::new(transform.position),
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
                cooldown: Cooldown::new(1.),
            },
            Hitbox::new(0.03 * VIRTUAL_STAGE_ASPECT_RATIO),
            Hitpoint::new(2.),
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
                cooldown: Cooldown::new(1.),
            },
            Hitbox::new(0.03 * VIRTUAL_STAGE_ASPECT_RATIO),
            Hitpoint::new(2.),
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
            Hitbox::new(0.01 * VIRTUAL_STAGE_ASPECT_RATIO),
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
        Hitbox::new(0.0125 * VIRTUAL_STAGE_ASPECT_RATIO),
    ));
}

pub fn stage_text(text: &str) -> impl FnOnce(&mut World) {
    let text = text.to_string();

    move |world| {
        world.spawn((
            DieOffScreen,
            Transform2D::new(cmpx!(0.5, 0.3), vec2!(0.05), 0.),
            Text::Center(text),
            // MoveParams::move_accelerated(cmpx!(0.), cmpx!(0., -0.1)),
        ));
    }
}
