use std::rc::Rc;

use piston_window::*;
use sprite::*;
// 130 x 103

pub fn get_sprite(
    sprite_x: f32,
    sprite_y: f32,
    width: f32,
    height: f32,
    x: f64,
    y: f64,
    scale_factor: f64,
    window: &mut PistonWindow,
) -> Sprite<piston_window::Texture<gfx_device_gl::Resources>> {
    let sprites = find_folder::Search::ParentsThenKids(0, 0)
        .for_folder("assets")
        .unwrap();
    let sprite_sheet = sprites.join("sprite/sprites.png");

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let tex = Rc::new(
        Texture::from_path(
            &mut texture_context,
            sprite_sheet,
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap(),
    );

    let mut sprite = Sprite::from_texture_rect(
        tex,
        [
            sprite_x as f64,
            sprite_y as f64,
            width as f64,
            height as f64,
        ],
    );

    sprite.set_position(x as f64, y as f64);
    sprite.set_anchor(0.5 as f64, 0.5 as f64);
    sprite.set_scale(scale_factor, scale_factor);
    // return the sprite
    sprite
}
