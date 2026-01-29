use crate::models::WordData;
use ratatui::widgets::ListState;

pub enum Focus {
    SearchBar,
    ResultsList,
    WordDetails,
}

pub struct App {
    pub results: Vec<WordData>,
    pub list_state: ListState,
    pub input: String,
    pub focus: Focus,
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
            focus: Focus::ResultsList,
        }
    }

    pub fn next_focus(&mut self) {
        self.focus = match self.focus {
            Focus::SearchBar => Focus::ResultsList,
            Focus::ResultsList => Focus::WordDetails,
            Focus::WordDetails => Focus::SearchBar,
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
