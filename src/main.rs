use std::result;

use ratatui:: {
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    DefaultTerminal
};
use crossterm::event::{self, Event, KeyCode};
use crate::models::{JishoResponse, WordData};

mod models;

struct App {
    results: Vec<WordData>,
    list_state: ListState,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = JishoResponse::search("ohayo").await?;
    
    let mut terminal = ratatui::init();
    let mut app = App { 
        results: response.data,
        list_state: ListState::default()
    };
    app.list_state.select(Some(0));
    let result = run_app(&mut terminal, app);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    let i = match app.list_state.selected() { 
                        Some(i) => if i >= app.results.len() - 1 { 0 } else { i + 1 },
                        None => 0,
                    };
                    app.list_state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match app.list_state.selected() {
                        Some(i) => if i == 0 { app.results.len() - 1 } else { i - 1 },
                        None => 0,
                    };
                    app.list_state.select(Some(i))
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(f.area());

    let items: Vec<ListItem> = app.results
            .iter()
            .map(|word| ListItem::new(word.slug.as_str()))
            .collect();

    let list = List::new(items)
            .block(Block::default().title("Search Results").borders(Borders::ALL))
            .highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::Blue))
            .highlight_symbol(">> ");
    
    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    let details = if let Some(i) = app.list_state.selected() {
        if let Some(selected_word) = app.results.get(i) {
            let definitions = selected_word.senses[0].english_definitions.join(", ");
            format!("Word: {}\nMeaning: {}", selected_word.slug, definitions)
        } else { "Data error".to_string()} 
    } else { "No word selected.".to_string() };
 
    let details_paragraph = Paragraph::new(details)
        .block(Block::default().title("Word Details").borders(Borders::ALL));
    f.render_widget(details_paragraph, chunks[1]);

}