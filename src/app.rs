use crate::models::WordData;
use ratatui::widgets::ListState;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub results: Vec<WordData>,
    pub list_state: ListState,
    pub input: String,
    pub input_mode: InputMode,
}

impl App {
    pub fn new(results: Vec<WordData>) -> Self {
        let mut list_state = ListState::default();
        if !results.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            results,
            list_state,
            input: String::new(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.results.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.results.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
