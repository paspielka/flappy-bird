use macroquad::{prelude::*, rand::RandomRange};

struct Vec2 {
    x: f32,
    y: f32
}

pub struct Pipe {
    texture: Texture2D,
    pos: Vec2,

    speed: f32,
    rot: f32,
}

impl Pipe {
    pub async fn new(_y: f32, _rot: f32) -> Self {
        Self {
            texture: load_texture("assets/pipe.png").await.unwrap(),
            pos: Vec2 {
                x: screen_width()+150.,
                y: _y,
            },

            speed: 3.,
            rot: _rot,
        }
    }

    pub async fn check_hitbox(player_hitbox: &Rect, pipe: &mut Pipe, _h: f32) {
        let pipe_hitbox = Rect {
            x: pipe.pos.x,
            y: pipe.pos.y,
            w: 50.,
            h: _h,
        };

        if pipe_hitbox.overlaps(player_hitbox) {
            Pipe::reset_game();
        }
    }

    pub fn reset_game() {
        
    }

    pub async fn update(arr: &mut [Pipe], player_hitbox: &Rect) {
        for pipe in arr.iter_mut() {
            draw_texture_ex(
                pipe.texture,
                pipe.pos.x,
                pipe.pos.y,
                WHITE,

                DrawTextureParams {
                    rotation: pipe.rot,

                    ..Default::default()
                }
            );
            
            if pipe.rot != 0. {
                Pipe::check_hitbox(player_hitbox, pipe, 320.).await;
            }
            else {
                Pipe::check_hitbox(player_hitbox, pipe, 400.).await;
            }

            pipe.pos.x -= pipe.speed;
        }     
    }

    pub async fn spawn(arr: &mut Vec<Pipe>) {
        arr.push(Pipe::new(screen_height() - RandomRange::gen_range(100, 200) as f32, 0.).await);
        arr.push(Pipe::new(screen_height() - RandomRange::gen_range(600, 700) as f32, 3.15).await);
    }
}