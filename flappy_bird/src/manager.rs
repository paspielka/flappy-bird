use macroquad::{prelude::*, audio::{load_sound, Sound}};

pub struct Manager {
    // textures
    pub background: Texture2D,
    pub gameover: Texture2D,
    pub menu: Texture2D,

    pub bird: Texture2D,
    pub pipe: Texture2D,

    // audio
    pub wing: Sound,
    pub hit: Sound
}

impl Manager {
    pub async fn new() -> Self {
        Self {
            // textures
            background: load_texture("assets/textures/background.png").await.unwrap(),
            gameover: load_texture("assets/textures/gameover.png").await.unwrap(),
            menu: load_texture("assets/textures/menu.png").await.unwrap(),

            bird: load_texture("assets/textures/bird.png").await.unwrap(),
            pipe: load_texture("assets/textures/pipe.png").await.unwrap(),

            // audio
            wing: load_sound("assets/audio/wing.ogg").await.unwrap(),
            hit: load_sound("assets/audio/hit.ogg").await.unwrap(),
        }
    }
}