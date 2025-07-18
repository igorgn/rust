use color_eyre::owo_colors::Style;
use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Modifier, Stylize}, text::Text, widgets::{Block, List, ListItem, }, Frame
};

use crate::{
    app::{App, State, TodoItem},
    helpers::zip_arrays,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let (items, descs) = { zip_arrays(&app) };

    let list_titles = List::new(items)
        .block(Block::bordered().title("Todo Items"))
        .add_modifier(Modifier::BOLD)
        .highlight_symbol(">>> ");

    let description_block = Text::raw(
        &app.get_list()
            .get(app.state.selected().unwrap())
            .unwrap()
            .description,
    );

    frame.render_stateful_widget(list_titles, chunks[0], &mut app.state.clone());
    frame.render_widget(description_block, chunks[1]);
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let checkbox = match value.status {
            State::Todo => "☐",
            State::Done => "☑",
        };
        ListItem::new(format!("{} {}", checkbox, value.title))
    }
}
