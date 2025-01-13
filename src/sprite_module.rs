use std::rc::Rc;

use piston_window::*;
use sprite::*;
// 130 x 103

pub fn get_sprite(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
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
    let mut sprite = Sprite::from_texture(tex);
    sprite.set_position(x as f64, y as f64);
    sprite.set_anchor(width as f64, height as f64);
    // return the sprite
    sprite
}
