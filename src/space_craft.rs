use piston_window::*;
use sprite::Sprite;

pub struct Aircraft {
    name: String,
    speed: u32,
    altitude: u32,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    sprite: Sprite<piston_window::Texture<gfx_device_gl::Resources>>,
}

impl Aircraft {
    pub fn new(
        name: String,
        speed: u32,
        altitude: u32,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
        sprite: Sprite<piston_window::Texture<gfx_device_gl::Resources>>,
    ) -> Aircraft {
        Aircraft {
            name,
            speed,
            altitude,
            width,
            height,
            x,
            y,
            sprite,
        }
    }

    fn move_up(&mut self) {
        self.y -= self.speed;
    }

    fn move_down(&mut self) {
        self.y += self.speed;
    }

    fn move_left(&mut self) {
        self.x -= self.speed;
    }

    fn move_right(&mut self) {
        self.x += self.speed;
    }
}
