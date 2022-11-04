use macroquad::window::Conf;
use macroquad::prelude::*;
use pipes::Pipe;

mod pipes;
mod bird;

#[macroquad::main(window_config)]
async fn main() {
    let mut player = bird::Player::new().await;
    let mut pipe_arr: Vec<Pipe> = vec![];

    let mut despawn_timer: u16 = 0;
    let mut spawn_timer: u16 = 0;

    let mut score: u16 = 0;

    loop {
        clear_background(WHITE);

        //pipes
        Pipe::update(&mut pipe_arr, &player.draw_hitbox()).await;
        despawn_timer += 1;
        spawn_timer += 1;

        //player
        player.draw().await;
        player.update();

        //game manager
        if despawn_timer > 600 {
            despawn_timer = 0;

            for _ in 0..2 {
                pipe_arr.remove(0); //remove the pipes out of bounds
            }
        }
        else if spawn_timer > 100{
            spawn_timer = 0;
            score += 1;

            Pipe::spawn(&mut pipe_arr).await;
        }
        
        //score
        draw_text(score.to_string().as_str(), screen_width()/2. - 20., screen_height()/8., 32., BLACK);

        next_frame().await;
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: String::from("flappy bird"),
        window_resizable: false,
        
        window_height: 480,
        window_width: 800,

        ..Default::default()
    }
}