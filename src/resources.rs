use macroquad::prelude::*;

pub struct Resources {
    pub sprite: Texture2D,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Self {
        let sprite = load_texture("./assets/atlas.png").await.unwrap();
        sprite.set_filter(FilterMode::Nearest);

        let mut font = load_ttf_font("./assets/PressStart2P-Regular.ttf")
            .await
            .unwrap();
        font.set_filter(FilterMode::Nearest);

        Self { sprite, font }
    }
}
