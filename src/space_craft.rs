use piston_window::*;
use sprite::Sprite;

pub struct Aircraft {
    name: String,
    speed: f64,
    width: f64,
    height: f64,
    x: f64,
    y: f64,
    sprite: Sprite<piston_window::Texture<gfx_device_gl::Resources>>,
}

impl Aircraft {
    pub fn new(
        name: String,
        speed: f64,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
        sprite: Sprite<piston_window::Texture<gfx_device_gl::Resources>>,
    ) -> Aircraft {
        Aircraft {
            name,
            speed,
            width,
            height,
            x,
            y,
            sprite,
        }
    }

    pub fn move_up(&mut self) {
        self.y -= self.speed;
        self.sprite.set_position(self.x as f64, self.y as f64);
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn move_down(&mut self) {
        self.y += self.speed;
        self.sprite.set_position(self.x as f64, self.y as f64);
    }

    pub fn move_left(&mut self) {
        self.x -= self.speed;
        self.sprite.set_position(self.x as f64, self.y as f64);
    }

    pub fn move_right(&mut self) {
        self.x += self.speed;
        self.sprite.set_position(self.x as f64, self.y as f64);
    }

    pub fn get_sprite(&self) -> &Sprite<piston_window::Texture<gfx_device_gl::Resources>> {
        &self.sprite
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct Bullet {
    sprite: Sprite<piston_window::Texture<gfx_device_gl::Resources>>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
}

impl Bullet {
    pub fn new(
        sprite: Sprite<piston_window::Texture<gfx_device_gl::Resources>>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        speed: f64,
    ) -> Bullet {
        Bullet {
            sprite,
            x,
            y,
            width,
            height,
            speed,
        }
    }

    pub fn get_sprite(&self) -> &Sprite<piston_window::Texture<gfx_device_gl::Resources>> {
        &self.sprite
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn move_up(&mut self) {
        self.y -= self.speed;
        self.sprite.set_position(self.x as f64, self.y as f64);
    }

    pub fn move_down(&mut self) {
        self.y += self.speed;
        self.sprite.set_position(self.x as f64, self.y as f64);
    }

    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}
