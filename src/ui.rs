use crate::app::{App, Focus};
use ratatui::style::{Color, Style};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &mut App) {
    // Get color on focused window
    let get_focus_border = |target: Focus| {
        if std::mem::discriminant(&app.focus) == std::mem::discriminant(&target) {
            Color::Yellow
        } else {
            Color::Gray
        }
    };

    // --- Layout initialize ---
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // --- Search bar ---
    let search_bar = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_style(Style::new().fg(get_focus_border(Focus::SearchBar))),
    );
    f.render_widget(search_bar, chunks[0]);

    // --- Search results list
    let items: Vec<ListItem> = app
        .results
        .iter()
        .map(|word| ListItem::new(word.slug.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Results ")
                .borders(Borders::ALL)
                .border_style(Style::new().fg(get_focus_border(Focus::ResultsList))),
        )
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, body_chunks[0], &mut app.list_state);

    // --- Word details block ---
    let details_block = Block::default()
        .title(" Word Details ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(get_focus_border(Focus::WordDetails)));

    let details_content = if let Some(i) = app.list_state.selected() {
        if let Some(word) = app.results.get(i) {
            // let kanji = word.slug;
            let meaning = word.senses[0].english_definitions.join(", ");

            format!("Word: {}\nMeaning: {}", word.slug, meaning)
        } else {
            "Data error".to_string()
        }
    } else {
        "No word selected.".to_string()
    };

    let details_paragraph = Paragraph::new(details_content)
        .block(details_block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(details_paragraph, body_chunks[1]);
}
