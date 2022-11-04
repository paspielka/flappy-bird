use macroquad::audio::PlaySoundParams;
use macroquad::audio::Sound;
use macroquad::audio::load_sound;
use macroquad::audio::play_sound;
use macroquad::prelude::*;
use macroquad::math::Rect;

pub struct Player {
    velocity: f32,
    gravity: f32,
    y: f32,

    texture: Texture2D,
    audio: Sound
}

impl Player {
    pub async fn new() -> Self {
        Self {
            velocity: 0.0,
            gravity: 0.2,
            
            texture: load_texture("assets/bird.png").await.unwrap(),
            audio: load_sound("assets/audio_wing.ogg").await.unwrap(),

            y: screen_height()/2.,
        }
    }

    pub async fn draw(&self) {
        draw_texture_ex(
            self.texture,
            screen_width()/4.,
            self.y,
            WHITE,
            DrawTextureParams {
                rotation: self.velocity * 0.1,
                ..Default::default()
            }
        );
    }

    pub fn draw_hitbox(&self) -> Rect {
        Rect {
            x: screen_width()/4.,
            y: self.y,
            w: 40.,
            h: 20.
        }
    }

    pub fn update(&mut self) {
        self.y += self.velocity;
        self.velocity += self.gravity;

        if is_key_pressed(KeyCode::Space) {
            self.velocity += -10.;
            play_sound(self.audio, PlaySoundParams {looped: false, volume: 0.3})
        }
    }
}