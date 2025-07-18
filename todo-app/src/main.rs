mod app;
mod helpers;
mod ui;

use crossterm::event::{self};

use app::*;
use helpers::*;
use ui::*;

fn main() {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    app.add_todo("Added", "Some description");
    // app.itms.add(new_item);
    app.select_first();

    loop {
        if app.should_exit {
            break;
        }

        terminal
            .draw(|frame| draw(frame, &mut app))
            .expect("failed to draw frame");

        let event = event::read().unwrap();
        handle_event(&mut app, event);
    }
    ratatui::restore();
}
