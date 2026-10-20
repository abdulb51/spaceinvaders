use crate::ui::still_image::StillImage;
use macroquad::prelude::*;
use macroquad::input::KeyCode;
pub struct Bullet {
    image: StillImage,
    shootspot: StillImage,
    projectile: StillImage,
    move_dir: Vec2,
    speed: f32,
  pub  shootbtn: KeyCode,
}

impl Bullet {
    pub async fn new(
        image: &str,
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
        let image = StillImage::new(image, width, height, x, y, stretch, 1.0).await;
        let shootspot = StillImage::new(shootspot, 20.0, 20.0, x, y, false, 1.0).await;
        let projectile = StillImage::new(projectile, width, height, x, y, stretch, 1.0).await;

        Bullet {
            image,
            shootspot,
            projectile,
            move_dir: Vec2::new(0.0, 0.0),
            speed: speed,
            shootbtn: shootbtn,
        }
    }
    
    
    
    
    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn 

    








}



