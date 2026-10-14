use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    image: StillImage,
    flipimage: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
    pub old_pos: Vec2,
    pub damage: f32,
    pub health: f32,
    level: i32,
    xp: i32,
}

impl Player {
    pub async fn new(
        image_name: &str,
        flipimage_name: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stretch: bool,
        zoom: f32,
        damage: f32,
        speed: f32,
        health: f32,
        level: i32,
        xp: i32,
    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;
        let flipimage = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Player {
            image,
            flipimage,
            speed: speed,
            move_dir: Vec2::new(0.0, 0.0),
            old_pos: Vec2::new(0.0, 0.0),
            damage: damage,
            health: health,
            level: level,
            xp: xp,
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn keypress(&mut self) {
        self.move_dir = vec2(0.0, 0.0);
        self.old_pos = vec2(self.image.get_x(), self.image.get_y());
        // Keyboard input
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.move_dir.x += self.speed;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.move_dir.x -= self.speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.move_dir.y += self.speed;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.move_dir.y -= self.speed;
        }

        if self.move_dir.length() > 0.0 {
            self.move_dir = self.move_dir.normalize() * 3.0;
        }
    }

    pub fn draw(&self) {
        self.image.draw();
    }

    pub fn move_player(&mut self) {
        self.image.set_x(self.image.get_x() + self.move_dir.x);
        self.image.set_y(self.image.get_y() + self.move_dir.y);
    }
    pub fn move_back_x(&mut self) {
        self.image.set_x(self.old_pos.x);
    }
    pub fn move_back_y(&mut self) {
        self.image.set_y(self.old_pos.y);
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }
    pub fn set_health(&mut self, health: f32) {
        self.health = health;
    }
    pub fn get_level(&self) -> i32 {
        self.level
    }
    pub fn set_level(&mut self, level: i32) {
        self.level = level;
    }
    pub fn get_xp(&self) -> i32 {
        self.xp
    }
    pub fn set_xp(&mut self, xp: i32) {
        self.xp = xp;
    }
    pub fn get_damage(&self) -> f32 {
        self.damage
    }
    pub fn set_damage(&mut self, damage: f32) {
        self.damage = damage;
    }

    pub fn level_up(&mut self) {
     
     self.damage = self.damage * 1.2;
        self.health = self.health * 1.1;
    }
    pub fn gain_xp(&mut self, xp: i32) {
        self.xp += xp;
        if self.xp >= 100 {
            self.level_up();
            self.xp = 0;
            self.level += 1;
        }
    }
}
