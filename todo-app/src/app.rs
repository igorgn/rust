use std::ops::Not;

use ratatui::widgets::ListState;

pub struct App {
    pub should_exit: bool,
    pub todo_list: TodoList,
    pub state: ListState,
}

pub struct TodoList {
    todo_items: Vec<TodoItem>,
}

pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub status: State,
}

pub enum State {
    Todo,
    Done,
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::Todo => State::Done,
            State::Done => State::Todo,
        }
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            todo_list: TodoList::default(),
            state: ListState::default(),
        }
    }
}

impl App {
    pub fn add_todo(&mut self, title: &str, description: &str) {
        let new_todo = TodoItem {
            title: title.to_string(),
            description: description.to_string(),
            status: State::Todo,
        };
        self.todo_list.add(new_todo);
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }
    pub fn select_next(&mut self) {
        self.state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }
    pub fn get_list(&self) -> &Vec<TodoItem> {
        &self.todo_list.todo_items
    }

    pub fn delete_current(&mut self) {
        if let Some(index) = self.state.selected() {
            self.todo_list.delete(index);
        }
    }

    pub fn toggle_status(&mut self) {
        self.todo_list.toggle_status(self.state.selected().unwrap());
    }
}
impl Default for TodoList {
    fn default() -> Self {
        Self {
            todo_items: vec![
                TodoItem {
                    title: "Test1".to_string(),
                    description: "First item".to_string(),
                    status: State::Todo,
                },
                TodoItem {
                    title: "Test2".to_string(),
                    description: "Some second item".to_string(),
                    status: State::Todo,
                },
                TodoItem {
                    title: "Test3".to_string(),
                    description: "Banana explanation".to_string(),
                    status: State::Todo,
                },
            ],
        }
    }
}

impl TodoList {
    fn add(&mut self, item_to_add: TodoItem) {
        self.todo_items.push(item_to_add);
    }

    fn delete(&mut self, index: usize) {
        self.todo_items.remove(index);
    }

    fn toggle_status(&mut self, index: usize) {
        self.todo_items[index].status = match self.todo_items[index].status {
            State::Todo => State::Done,
            State::Done => State::Todo,
        }
    }
}
