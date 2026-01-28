use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let items: Vec<ListItem> = app
        .results
        .iter()
        .map(|word| ListItem::new(word.slug.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Search Results")
                .borders(Borders::ALL),
        )
        .highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::Blue))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    let details = if let Some(i) = app.list_state.selected() {
        if let Some(selected_word) = app.results.get(i) {
            let definitions = selected_word.senses[0].english_definitions.join(", ");
            format!("Word: {}\nMeaning: {}", selected_word.slug, definitions)
        } else {
            "Data error".to_string()
        }
    } else {
        "No word selected.".to_string()
    };

    let details_paragraph =
        Paragraph::new(details).block(Block::default().title("Word Details").borders(Borders::ALL));
    f.render_widget(details_paragraph, chunks[1]);
}
