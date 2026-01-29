use crate::app::{App, Focus};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
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
        .map(|word| ListItem::new(word.japanese[0].word.as_deref().unwrap_or(&word.slug)))
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
            let mut lines = vec![];

            let kanji = word.japanese[0].word.as_deref().unwrap_or(&word.slug);
            let reading = word.japanese[0].reading.as_deref().unwrap_or("---");
            let meaning = word.senses[0].english_definitions.join(", ");

            lines.push(
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(
                        kanji,
                        Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
                    ),
                ])
                .alignment(HorizontalAlignment::Center),
            );

            lines.push(
                Line::from(vec![
                    Span::raw("   "),
                    Span::styled(reading, Style::new().fg(Color::Cyan)),
                ])
                .alignment(HorizontalAlignment::Center),
            );

            Text::from(lines)
        } else {
            Text::from("Data error")
        }
    } else {
        Text::from("No word selected.")
    };

    let details_paragraph = Paragraph::new(details_content)
        .block(details_block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(details_paragraph, body_chunks[1]);

    if let Focus::SearchBar = app.focus {
        f.set_cursor_position((
            chunks[0].x + app.input.chars().count() as u16 + 1,
            chunks[0].y + 1,
        ));
    }
}
