use piston_window::*;
mod space_craft;
mod sprite_module;
use piston::window::WindowSettings;

fn create_window(size: Vec<i16>, centered: bool) -> PistonWindow {
    WindowSettings::new("Space Armada", [size[0] as u32, size[1] as u32])
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap()
}

fn create_player(window: &mut PistonWindow) -> space_craft::Aircraft {
    let player = space_craft::Aircraft::new(
        String::from("Foxomax"),
        0.5,
        130.0,
        103.0,
        400.0,
        300.0,
        sprite_module::get_sprite(64.0, 64.0, 130.0, 108.0, 400.0, 300.0, 0.5, window),
    );
    player
}

fn create_bullet(window: &mut PistonWindow, x: f64, y: f64) -> space_craft::Bullet {
    let bullet = space_craft::Bullet::new(
        sprite_module::get_sprite(336.0, 176.0, 128.0, 64.0, 400.0, 300.0, 0.3, window),
        x,
        y,
        10.0,
        400.0,
        300.0,
    );
    bullet
}

fn init_game() {
    let mut window: PistonWindow = create_window(Vec::from([800, 600]), true);
    let mut player = create_player(&mut window);
    let mut bullets: Vec<space_craft::Bullet> = vec![];
    let assets = find_folder::Search::ParentsThenKids(0, 0)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("fonts/Orbitron-Medium.ttf"))
        .unwrap();

    let mut keys_pressed = vec![];

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            glyphs.factory.encoder.flush(device);
            player.get_sprite().draw(c.transform, g);
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if !keys_pressed.contains(&key) {
                keys_pressed.push(key);
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            keys_pressed.retain(|&k| k != key);
        }

        for key in &keys_pressed {
            match key {
                Key::Up => {
                    if player.get_y() > player.get_speed() {
                        player.move_up();
                    }
                }
                Key::Down => {
                    if player.get_y() < 600.0 - player.get_speed() {
                        player.move_down();
                    }
                }
                Key::Left => {
                    if player.get_x() > player.get_speed() {
                        player.move_left();
                    }
                }
                Key::Right => {
                    if player.get_x() < 800.0 - player.get_speed() {
                        player.move_right();
                    }
                }
                Key::W => {
                    if player.get_y() > player.get_speed() {
                        player.move_up();
                    }
                }
                Key::S => {
                    if player.get_y() < 600.0 - player.get_speed() {
                        player.move_down();
                    }
                }
                Key::A => {
                    if player.get_x() > player.get_speed() {
                        player.move_left();
                    }
                }
                Key::D => {
                    if player.get_x() < 800.0 - player.get_speed() {
                        player.move_right();
                    }
                }
                Key::Space => {
                    let bullet = create_bullet(&mut window, player.get_x(), player.get_y());
                    bullets.push(bullet);
                }
                _ => {}
            }
        }

        for bullet in &mut bullets {
            bullet.move_up();
        }

        bullets.retain(|bullet| bullet.get_y() > 0.0);
    }
}

fn main() {
    println!("...");
    init_game();
}
