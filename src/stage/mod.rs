use crate::entity::*;
use crate::utils::float_iter;
use crate::{cmpx, spawner::Spawner};

pub fn stage_1(spawner: &mut Spawner) {
    spawner.spawn(0., create_player);

    spawner.spawn(0., stage_text("Stage 1"));

    for start in float_iter(3., 6., 1.) {
        spawner.spawn(start, create_red_alien_wait(cmpx!(0.5, -0.02)));
    }

    for start in float_iter(7., 9., 1.) {
        spawner.spawn(start, create_red_alien(cmpx!(0.2, -0.02)));
    }

    for start in float_iter(10., 12., 1.) {
        spawner.spawn(start, create_red_alien(cmpx!(0.8, -0.02)));
    }

    spawner.spawn(14., create_boss_1());
}
