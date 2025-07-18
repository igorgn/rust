use crossterm::event::{Event, KeyCode};
use ratatui::widgets::ListItem;

use crate::app::{App, State};

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('j') => app.select_next(),
            KeyCode::Char('k') => app.select_previous(),
            KeyCode::Char(' ') => app.toggle_status(),
            KeyCode::Char('r') => app.delete_current(),
            KeyCode::Char('q') => app.should_exit = true,
            _ => {}
        }
    }
}

pub fn zip_arrays(app: &App) -> (Vec<ListItem>, Vec<ListItem>) {
    app.get_list()
        .iter()
        .map(|item| {
            let checkbox = match item.status {
                State::Todo => "☐".to_string(),
                State::Done => "☑".to_string(),
            };
            (
                ListItem::new(format!("{} {}", checkbox, item.title.clone())),
                ListItem::new(item.description.clone()),
            )
        })
        .unzip()
}
