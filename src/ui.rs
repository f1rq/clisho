use crate::app::{App, InputMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());
    
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), 
            Constraint::Percentage(70),
        ])
        .split(chunks[1]);

    let (border_color, title) = match app.input_mode {
        InputMode::Normal => (
            ratatui::style::Color::Gray, 
            " [Normal Mode] Press 's' to Search "
        ),
        InputMode::Editing => (
            ratatui::style::Color::Yellow,
            " [Editing Mode ] Press 'Enter' to confirm, 'Esc' to cancel "
        )
    };

    let search_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(border_color));

    let search_bar = Paragraph::new(app.input.as_str()).block(search_block);
    f.render_widget(search_bar, chunks[0]);

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

    f.render_stateful_widget(list, body_chunks[0], &mut app.list_state);

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
        Paragraph::new(details)
            .block(Block::default()
            .title("Word Details")
            .borders(Borders::ALL))
            .wrap(ratatui::widgets::Wrap { trim: true });
    
    f.render_widget(details_paragraph, body_chunks[1]);
}
