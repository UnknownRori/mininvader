use crate::cmpx;
use crate::components::*;
use crate::entity::*;
use crate::math::*;
use crate::renderer::Renderer;
use crate::resources::Resources;
use crate::utils::rand_dir;
use crate::vec2;
use crate::Action;
use crate::Controls;
use hecs::Entity;
use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

pub fn draw_boss_hitpoint(world: &World, renderer: &Renderer) {
    world
        .query::<(&Boss, &Hitpoint, &HealthBar)>()
        .iter()
        .for_each(|(_, (_, hp, healthbar))| {
            renderer.draw_hp_bar(hp.hp, hp.max_hp, healthbar.0);
        });
}

pub fn draw_hitbox_system(world: &World, renderer: &Renderer) {
    world
        .query::<(&Hitbox, &Transform2D)>()
        .iter()
        .for_each(|(_, (hitbox, transform))| {
            renderer.debug_draw_hitbox(&transform, hitbox.radius);
        });
}

pub fn draw_sprites_system(world: &World, resources: &Resources, renderer: &Renderer) {
    world
        .query::<(&Sprite, &Transform2D)>()
        .iter()
        .for_each(|(_, (sprite, transform))| {
            renderer.draw_sprite(&resources.sprite, sprite.0, transform)
        })
}
pub fn draw_text_system(world: &World, resources: &Resources, renderer: &Renderer) {
    world
        .query::<(&Text, &Transform2D)>()
        .iter()
        .for_each(|(_, (text, transform))| renderer.draw_text(&text.0, &resources.font, transform))
}

pub fn player_controls(world: &mut World, controls: &Controls) {
    let mut pending = Vec::new();

    world
        .query::<(
            &Player,
            &Controllable,
            &mut Transform2D,
            &mut MoveParams,
            &mut Cooldown,
        )>()
        .iter()
        .for_each(|(_, (_, _, transform, move_params, cooldown))| {
            let mut new_pos = cmpx!(0.);
            let move_speed = 12.5; // TODO : Make this correspond player mode

            if controls.is_down(Action::Left) {
                new_pos += Complex::new(-move_speed, 0.0);
            }

            if controls.is_down(Action::Right) {
                new_pos += Complex::new(move_speed, 0.0);
            }

            if controls.is_down(Action::Up) {
                new_pos += Complex::new(0.0, -move_speed);
            }

            if controls.is_down(Action::Down) {
                new_pos += Complex::new(0.0, move_speed);
            }

            let move_speed = if controls.is_down(Action::Focus) {
                1. / 2.6
            } else {
                1.
            };

            move_params.acceleration = new_pos * move_speed;

            let rect = Rect::new(0.05, 0.05, 0.95, 0.95);
            transform.position = transform.position.clamp(&cmpx!(0.05), &cmpx!(0.95));
            if !rect.contains(transform.position().to_vec2()) {
                move_params.acceleration = cmpx!(0.);
            }

            if controls.is_down(Action::Attack) && cooldown.0.completed() {
                cooldown.0.update();
                pending.push(create_player_generic_bullet(transform.position));
            } else {
                cooldown.0.update();
            }
        });

    for i in pending {
        (i)(world);
    }
}
pub fn update_moves(world: &World) {
    world
        .query::<(&mut Moves, &mut MoveParams, &Transform2D)>()
        .iter()
        .for_each(|(_, (moves, move_params, position))| {
            *move_params = moves.update(&move_params, position);
        });
}
pub fn update_movement(world: &World) {
    world
        .query::<(&mut Transform2D, &mut MoveParams)>()
        .iter()
        .for_each(|(_, (transform, move_params))| {
            move_params.update(&mut transform.position, get_frame_time());
        });
}

pub fn update_boss_move(world: &mut World) {
    let players = world
        .query::<(&Player, &Controllable, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let mut boss = world
        .query::<(&Boss, &Enemy, &Transform2D, &BossMoves)>()
        .iter()
        .map(|(id, (_, _, transform, boss))| {
            (
                id.clone(),
                transform.clone(),
                match boss.0.front() {
                    Some(attack) => Some(attack.clone()),
                    None => None,
                },
            )
        })
        .collect::<Vec<_>>();

    boss.iter_mut().for_each(|(id, transform, boss)| {
        if let Some(ref mut attack) = boss {
            if let Some(player) = players.first() {
                handle_fire_bullet(world, &id, &attack.attack, transform, &player.1);

                update_cooldown_attack(&mut attack.attack);
                let mut boss_move = world.get::<&mut BossMoves>(*id).unwrap();
                *boss_move.0.front_mut().unwrap() = attack.clone();
            }

            // TODO : Test this thing
            let mut collection_attack_ref = world.get::<&mut BossMoves>(*id).unwrap();

            {
                let mut boss_hp = world.get::<&mut Hitpoint>(*id).unwrap();
                if boss_hp.invulnerable {
                    let hp = collection_attack_ref
                        .0
                        .front()
                        .map(|a| Hitpoint::new(a.hp))
                        .unwrap_or(Hitpoint::invulnerable());
                    *boss_hp = hp
                }
            }

            let attack_ref = collection_attack_ref.0.front_mut().unwrap();
            attack_ref.timeout.update();

            if attack_ref.timeout.completed() {
                collection_attack_ref.0.pop_front();
                let mut boss_hp = world.get::<&mut Hitpoint>(*id).unwrap();
                let hp = collection_attack_ref
                    .0
                    .front()
                    .map(|a| Hitpoint::new(a.hp))
                    .unwrap_or(Hitpoint::invulnerable());
                *boss_hp = hp
            }
        }
    })
}

pub fn collision(world: &mut World) {
    let players = world
        .query::<(&Player, &Controllable, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let enemies = world
        .query::<(&Enemy, &Transform2D, &Hitbox)>()
        .without::<&Bullet>()
        .without::<&Boss>()
        .iter()
        .map(|(id, (_, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let player_bullets = world
        .query::<(&Player, &Bullet, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let enemy_bullets = world
        .query::<(&Enemy, &Bullet, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let boss = world
        .query::<(&Boss, &Enemy, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    {
        if let Some(player) = players.first() {
            for enemy_bullet in enemy_bullets {
                if player
                    .2
                    .is_intersect(&player.1, &enemy_bullet.1, &enemy_bullet.2)
                {
                    let _ = world.despawn(enemy_bullet.0);
                    // TODO : Reduce Player life
                }
            }
        }
    }

    {
        for player_bullet in &player_bullets {
            for boss in &boss {
                if player_bullet
                    .2
                    .is_intersect(&player_bullet.1, &boss.1, &boss.2)
                {
                    let _ = world.despawn(player_bullet.0);

                    // TODO : Make the damage based on bullet type
                    match world.satisfies::<&Hitpoint>(boss.0) {
                        Ok(exist) if exist => {
                            world.get::<&mut Hitpoint>(boss.0).unwrap().damage(0.5);
                        }
                        _ => {}
                    };

                    let despawn = match world.get::<&Hitpoint>(boss.0) {
                        Ok(hitpoint) if hitpoint.is_dead() => true,
                        _ => false,
                    };

                    if despawn {
                        let _ = world.despawn(boss.0);
                    }
                }
            }
        }
    }

    {
        for player_bullet in player_bullets {
            for enemy in &enemies {
                if player_bullet
                    .2
                    .is_intersect(&player_bullet.1, &enemy.1, &enemy.2)
                {
                    let _ = world.despawn(player_bullet.0);

                    // TODO : Make the damage based on bullet type
                    match world.satisfies::<&Hitpoint>(enemy.0) {
                        Ok(exist) if exist => {
                            world.get::<&mut Hitpoint>(enemy.0).unwrap().damage(0.5);
                        }
                        _ => {}
                    };

                    let despawn = match world.get::<&Hitpoint>(enemy.0) {
                        Ok(hitpoint) if hitpoint.is_dead() => true,
                        _ => false,
                    };

                    if despawn {
                        let _ = world.despawn(enemy.0);
                    }
                }
            }
        }
    }
}
pub fn fire_bullets(world: &mut World) {
    let player = world
        .query::<(&Player, &Controllable, &Transform2D)>()
        .iter()
        .map(|(_, (_, _, transform))| transform.clone())
        .collect::<Vec<_>>();

    let pending = world
        .query::<(&AttackMove, &Transform2D)>()
        .iter()
        .map(|(id, (attack, transform))| (id.clone(), attack.clone(), transform.clone()))
        .collect::<Vec<_>>();

    if let Some(player) = player.first() {
        for (id, mut attack_move, transform) in pending {
            handle_fire_bullet(world, &id, &attack_move, &transform, player);

            update_cooldown_attack(&mut attack_move);
            *(world.get::<&mut AttackMove>(id).unwrap()) = attack_move;
        }
    }
}

fn update_cooldown_attack(attack: &mut AttackMove) {
    match attack {
        AttackMove::AtPlayer { cooldown, .. } => cooldown.0.update(),
        AttackMove::Circle { cooldown, .. } => cooldown.0.update(),
        AttackMove::Multiple(attacks) => attacks
            .iter_mut()
            .for_each(|attack| update_cooldown_attack(attack)),
    };
}

fn handle_fire_bullet(
    world: &mut World,
    id: &Entity,
    attack_move: &AttackMove,
    transform: &Transform2D,
    player: &Transform2D,
) {
    match attack_move {
        AttackMove::AtPlayer {
            num,
            speed,
            spread,
            total_shoot,
            cooldown,
            setup,
        } if cooldown.0.completed() => {
            if *total_shoot <= 0 {
                return;
            }

            if *num > 1 {
                for i in 0..*num as i32 {
                    let angle = (i - 1) as f32 * spread;
                    let dir =
                        transform.position.dir(player.position()) * Complex::cdir(angle) * speed
                            + (rand_dir() * 0.005).to_cmpx();
                    let move_params = MoveParams::move_linear(dir);
                    let transform = Transform2D {
                        rotation: dir.rot(),
                        scale: vec2!(0.05),
                        ..*transform
                    };
                    create_enemy_bullet(
                        world,
                        transform,
                        setup.0.clone(),
                        move_params,
                        Hitbox::new(0.01),
                    );
                }
                return;
            }

            let dir = transform.position.dir(player.position()) * speed;
            let move_params = MoveParams::move_linear(dir);
            let transform = Transform2D {
                scale: vec2!(0.05),
                rotation: dir.rot(),
                ..*transform
            };
            create_enemy_bullet(world, transform, setup.0, move_params, Hitbox::new(0.01));
        }
        AttackMove::Multiple(moves) => moves
            .iter()
            .for_each(|attack_move| handle_fire_bullet(world, id, attack_move, transform, player)),
        AttackMove::Circle {
            sides,
            rotation_per_fire,
            rotation,
            cooldown,
            setup,
        } if cooldown.0.completed() => {
            for side in 0..*sides {
                let rotation =
                    (side as f32 / *sides as f32) * std::f32::consts::PI * 2. + *rotation;
                let dir = Complex::cdir(rotation);
                let move_params = MoveParams::move_linear(dir);

                let transform = Transform2D {
                    scale: vec2!(0.05),
                    rotation: dir.rot(),
                    ..*transform
                };
                create_enemy_bullet(world, transform, setup.0, move_params, Hitbox::new(0.01));
            }
        }

        AttackMove::AtPlayer { .. } | AttackMove::Circle { .. } => {}
    }
}

pub fn scan_been_onscreen(world: &mut World) {
    let pending = world
        .query::<(&Transform2D, &DieOffScreen, Option<&BeenOnScreen>)>()
        .iter()
        .map(|(id, (transfrom, _, been_on_screen))| {
            (
                id.clone(),
                transfrom.clone(),
                match been_on_screen {
                    Some(a) => a.clone(),
                    None => BeenOnScreen(false),
                },
            )
        })
        .collect::<Vec<_>>();

    let container = Rect::new(0., 0., 1., 1.);
    for i in pending {
        if container.contains(i.1.position.to_vec2()) {
            let _ = world.insert_one(i.0, BeenOnScreen(true));
        } else {
            let _ = world.insert_one(i.0, i.2);
        }
    }
}
pub fn clean_offscreen(world: &mut World) {
    let pending = world
        .query::<(&Transform2D, &BeenOnScreen)>()
        .iter()
        .filter(|(_, (transform, been_on_screen))| {
            let container = Rect::new(0., 0., 1., 1.);
            been_on_screen.0 && (!container.contains(transform.position.to_vec2()))
        })
        .map(|(id, (_, _))| id.clone())
        .collect::<Vec<_>>();

    for i in pending {
        let _ = world.despawn(i);
    }
}
