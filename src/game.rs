use conrod;
use ::Ids;

use std::path::{PathBuf};

pub struct Game {
    assets: PathBuf,
}

impl Game {
    pub fn new(assets: PathBuf) -> Game {
        Game {
            assets: assets,
        }
    }

    pub fn draw_ui(&mut self, ref mut ui: conrod::UiCell, ids: &Ids) {
        use conrod::{color, widget, Labelable, Colorable, Positionable, Sizeable, Borderable, Widget};

        widget::Canvas::new().color(color::DARK_CHARCOAL).set(ids.canvas, ui);

        for _ in widget::Button::new()
            .align_label_middle()
            .middle_of(ids.canvas)
            .w_h(200.0, 200.0)
            .border(5.0)
            .border_color(color::BLACK)
            .label("Play")
            .label_color(color::WHITE)
            .label_font_size(32)
            .color(color::DARK_PURPLE)
            .set(ids.play_button, ui) {
                self.play_clicked();
        }
    }

    fn play_clicked(&mut self) {

    }
}
