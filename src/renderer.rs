use macroquad::prelude::*;

use crate::{
    components::Transform2D,
    konst::{DESIRED_ASPECT_RATIO, VIRTUAL_STAGE_HEIGHT, VIRTUAL_STAGE_WIDTH},
    resources::Resources,
    utils::{get_adjusted_screen, FPSCounter},
    vec2,
};

pub struct Renderer {
    game: Camera2D,
}

impl Default for Renderer {
    fn default() -> Self {
        let coordinate = Rect::new(0.0, 0.0, 1.0, 1.0);
        let game_render_target = render_target(VIRTUAL_STAGE_WIDTH, VIRTUAL_STAGE_HEIGHT);
        game_render_target.texture.set_filter(FilterMode::Nearest);
        let game = create_camera2d(coordinate, game_render_target);

        Self { game }
    }
}

impl Renderer {
    pub fn draw_sprite(&self, sprite: &Texture2D, rect: Rect, transform: &Transform2D) {
        set_camera(&self.game);
        let half_scale = *transform.scale() / 2.;

        draw_texture_ex(
            sprite,
            transform.position().re - half_scale.x,
            transform.position().im - half_scale.y,
            WHITE,
            DrawTextureParams {
                source: Some(rect),
                dest_size: Some(*transform.scale()),
                rotation: -*transform.rotation(), // Not sure why it need to be negative
                ..Default::default()
            },
        );
        set_default_camera();
    }

    pub fn draw_text(&self, text: &str, font: &Font, transform: &Transform2D) {
        set_camera(&self.game);
        let (font_size, font_scale, font_scale_aspect) = camera_font_scale(transform.scale.x);
        let dimension = measure_text(text, Some(font), font_size, font_scale);
        draw_text_ex(
            text,
            transform.position.re - dimension.width / 2.5,
            transform.position.im,
            TextParams {
                font: Some(font),
                font_size,
                font_scale,
                font_scale_aspect,
                rotation: 0.,
                color: WHITE,
            },
        );
        set_default_camera();
    }

    pub fn draw_hp_bar(&self, current: f32, max: f32, pos: Rect) {
        set_camera(&self.game);
        draw_rectangle(pos.x, pos.y, pos.w * (current / max), pos.h, RED);
        set_default_camera();
    }

    pub fn debug_draw_hitbox(&self, pos: &Transform2D, radius: f32) {
        set_camera(&self.game);
        draw_circle(
            pos.position().re,
            pos.position().im,
            radius,
            Color::new(1., 0., 0., 0.8),
        );
        set_default_camera();
    }

    pub fn init(&self) {
        set_camera(&self.game);

        clear_background(BLACK);

        set_default_camera();
    }

    pub fn finalize(&self, fps: &FPSCounter, resources: &Resources) {
        // set_camera(&self.game);
        // fps.draw(&resources.font, vec2!(1.), 0.045);
        //
        // set_default_camera();

        let width = screen_width();
        let height = screen_height();
        let adjusted = get_adjusted_screen(DESIRED_ASPECT_RATIO);
        let offset = vec2((width - adjusted.x) / 2f32, (height - adjusted.y) / 2f32);
        let texture = &self.game.render_target.as_ref().unwrap().texture;
        clear_background(Color::new(0.2, 0.2, 0.2, 1.));
        draw_texture_ex(
            texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(adjusted),
                ..Default::default()
            },
        );

        fps.draw(&resources.font, vec2!(width, height - 14.), 14.);
        set_default_camera();
    }
}

pub fn create_camera2d(rect: Rect, render_target: RenderTarget) -> Camera2D {
    let mut camera = Camera2D::from_display_rect(rect);
    camera.zoom = vec2(1. / rect.w * 2., 1. / rect.h * 2.);
    camera.render_target = Some(render_target);
    camera
        .render_target
        .as_ref()
        .unwrap()
        .texture
        .set_filter(FilterMode::Nearest);

    camera
}
