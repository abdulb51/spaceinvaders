use crate::ui::still_image::StillImage;
use macroquad::prelude::*;
use macroquad::input::KeyCode;
pub struct Bullet {
    shootspot: StillImage,
    projectile: StillImage,
   pub move_dir: Vec2,
    speed: f32,
  pub  shootbtn: KeyCode,
}

impl Bullet {
    pub async fn new(
        shootspot: &str,
        projectile: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stretch: bool,
        speed: f32,
        shootbtn: KeyCode,
    ) -> Self {
        // let shootspot = StillImage::new(shootspot, 20.0, 20.0, x, y, false, 1.0).await;
        let projectile = StillImage::new(projectile, width, height, x, y, stretch, 1.0).await;

        Bullet {
            
            shootspot: shootspot,
            projectile,
            move_dir: Vec2::new(0.0, 0.0),
            speed: speed,
            shootbtn: shootbtn,
        }
    }
    
    
    
    
    pub fn get_image(&self) -> &StillImage {
        &self.projectile
    }

pub fn get_speed(&self) -> f32 {
        self.speed
    }
pub fn shoot(&self) {  {
    
      if is_key_pressed (self.shootbtn) {
self.move_dir = vec2(0.0, -1.0);
    }
}



 }



 }