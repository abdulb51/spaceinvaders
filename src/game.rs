use macroquad::prelude::*;

use crate::custom::player::Player;
use crate::ui::text_button::TextButton;

pub async fn run() -> String {



let btn_exit = TextButton::new(
        0.0,
            0.0,
        50.0,
        50.0,
        "X",
        RED,
        BLACK,
        50
    );

    let mut player = Player::new(
        "assets/ship.png",
"assets/shipboost.png",
        55.0,
        405.0,
        150.0,
        150.0,
        true,
        1.0,
        0.0,
        5.0,
    ).await;


    loop {
        clear_background(WHITE);
        draw_text("game", 20.0, 40.0, 30.0, WHITE);

player.keypress();
player.move_player();



        if is_key_pressed(KeyCode::R) {
            return "win".to_string();
        }

if btn_exit.click() {
  return "main".to_string();
        }

player.get_image().draw();
        next_frame().await;
    }
}