#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;

use piston_window::{EventLoop, PistonWindow, UpdateEvent, WindowSettings};

mod game;

use game::{Game};

widget_ids!(
    struct Ids {
        canvas,
        play_button
    }
);

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut window: PistonWindow = {
        let title = "character_creator_game";
        let resolution = [WIDTH, HEIGHT];
        let opengl = piston_window::OpenGL::V3_2;
        let mut window_result = WindowSettings::new(title, resolution)
            .exit_on_esc(true)
            .srgb(true)
            .opengl(opengl)
            .build();
        if window_result.is_err() {
            window_result = WindowSettings::new(title, resolution)
                .exit_on_esc(true)
                .srgb(false)
                .opengl(opengl)
                .build();
        }
        PistonWindow::new(opengl, 0, window_result
            .unwrap_or_else(|e| {
                panic!("Failed to build PistonWindow: {}", e)
            })
        )
    };

    window.set_ups(60);

    let mut ui = conrod::UiBuilder::new().build();

    let ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    let image_map = conrod::image::Map::new();

    let game = Game::new(assets);

    while let Some(event) = window.next() {
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{color, widget, Labelable, Colorable, Positionable, Sizeable, Borderable, Widget};

            let mut ui = ui.set_widgets();

            widget::Canvas::new().color(color::DARK_CHARCOAL).set(ids.canvas, &mut ui);

            for times_clicked in widget::Button::new()
                .align_label_middle()
                .middle_of(ids.canvas)
                .w_h(200.0, 200.0)
                .border(5.0)
                .border_color(color::BLACK)
                .label("Play")
                .label_color(color::WHITE)
                .label_font_size(32)
                .color(color::DARK_PURPLE)
                .set(ids.play_button, &mut ui) {
                    println!("{:?}", times_clicked);
            }
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(image: &T) -> &T {
                    image
                }
                conrod::backend::piston_window::draw(c, g, primitives, &mut text_texture_cache, &image_map, texture_from_image);
            }
        });
    }
}
