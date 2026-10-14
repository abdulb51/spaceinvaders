use macroquad::prelude::*;

use crate::custom::player::Player;

pub async fn run() -> String {





    let mut player = Player::new(
        "assets/subaru.png",
"assets/subaruflip.png",
        55.0,
        405.0,
        150.0,
        150.0,
        true,
        1.0,
        0.0,
        5.0,
        0.0,
        0,
        0,
    ).await;


    loop {
        clear_background(WHITE);
        draw_text("game", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "win".to_string();
        }

        next_frame().await;
    }
}