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
        String::from("Player"),
        5,
        0,
        130,
        103,
        0,
        0,
        sprite_module::get_sprite(0, 0, 130, 103, window),
    );
    player
}

fn init_game() {
    let mut window: PistonWindow = create_window(Vec::from([800, 600]), true);
    let player = create_player(&mut window);
    let assets = find_folder::Search::ParentsThenKids(0, 0)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("fonts/Orbitron-Medium.ttf"))
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.1, 0.0, 0.2, 1.0], g);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g)
                .unwrap();
            glyphs.factory.encoder.flush(device);
        });
    }
}

fn main() {
    println!("Initializing game...");
    init_game();
}
