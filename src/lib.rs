use controls::*;
use hecs::World;
use macroquad::prelude::*;
use renderer::Renderer;
use resources::Resources;
use spawner::Spawner;
use stage::*;
use system::*;
use utils::FPSCounter;

mod components;
mod controls;
mod entity;
mod konst;
mod math;
mod renderer;
mod resources;
mod score;
mod spawner;
mod stage;
mod system;
mod time;
mod utils;

pub struct Game {
    resources: Resources,
    renderer: Renderer,
    controls: Controls,
    world: World,

    fps: FPSCounter,

    spawner: Spawner,
}

impl Game {
    pub async fn new() -> Self {
        let world = World::new();
        let controls = init_controls();
        let resources = Resources::new().await;
        let renderer = Renderer::default();
        let fps = FPSCounter::default();

        let mut spawner = Spawner::default();
        stage_1(&mut spawner);

        Self {
            world,
            controls,
            resources,
            renderer,

            fps,

            spawner,
        }
    }

    pub fn update(&mut self) {
        let delta = get_frame_time();

        self.fps.update();
        self.spawner.update(&mut self.world, delta);
        update_cooldown(&self.world);
        player_controls(&mut self.world, &self.controls);
        update_moves(&self.world);
        update_movement(&self.world);
        collision(&mut self.world);
        fire_bullets(&mut self.world);
        scan_been_onscreen(&mut self.world);
        clean_offscreen(&mut self.world);
    }

    pub fn render(&self) {
        self.renderer.init();
        let world = &self.world;
        let renderer = &self.renderer;
        let fps = &self.fps;
        let resources = &self.resources;

        draw_sprites_system(world, resources, renderer);
        draw_text_system(world, resources, renderer);
        // draw_hitbox_system(world, renderer);
        draw_boss_hitpoint(world, renderer);
        self.renderer.finalize(fps, resources);

        // macroquad_profiler::profiler(macroquad_profiler::ProfilerParams {
        //     fps_counter_pos: vec2!(0., 0.),
        // });
    }

    pub async fn run(&mut self) {
        loop {
            self.update();
            self.render();
            next_frame().await;
        }
    }
}

pub fn window() -> Conf {
    Conf {
        window_title: String::from("Mininvader"),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        icon: None,
        sample_count: 1,
        high_dpi: true,
        platform: Default::default(),
    }
}
