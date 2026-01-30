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

    // --- Word block ---
    let word_block = Block::default()
        .title(" Word ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(get_focus_border(Focus::Word)));

    let word_content = if let Some(i) = app.list_state.selected() {
        if let Some(word) = app.results.get(i) {
            let mut lines = vec![];

            let hr = Line::from(vec![Span::styled(
                "-".repeat(body_chunks[1].width as usize - 2),
                Style::new().fg(Color::DarkGray),
            )]);

            let kanji = word.japanese[0].word.as_deref().unwrap_or(&word.slug);
            let reading = word.japanese[0].reading.as_deref().unwrap_or("---");

            let mut tags_spans = vec![];

            if let Some(true) = word.is_common {
                tags_spans.push(Span::styled(
                    "common word",
                    Style::new().bg(Color::Green).fg(Color::Black),
                ));
                tags_spans.push(Span::raw(" "));
            }

            for jlpt in &word.jlpt {
                let tag = jlpt.replace("-", " ");
                tags_spans.push(Span::styled(
                    format!(" {} ", tag),
                    Style::new().bg(Color::LightBlue).fg(Color::White),
                ));
                tags_spans.push(Span::raw(" "));
            }

            for tag in &word.tags {
                if tag.starts_with("wanikani") {
                    let wanikani_tag = tag.replace("wanikani", "wanikani level ");
                    tags_spans.push(Span::styled(
                        format!(" {} ", wanikani_tag),
                        Style::new().bg(Color::LightBlue).fg(Color::White),
                    ));
                    tags_spans.push(Span::raw(" "));
                }
            }

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

            lines.push(hr);

            if !tags_spans.is_empty() {
                lines.push(Line::from(tags_spans).alignment(HorizontalAlignment::Right));
            }

            let mut def_count = 1;

            for sense in &word.senses {
                if !sense.parts_of_speech.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            sense.parts_of_speech.join(", "),
                            Style::new().fg(Color::DarkGray),
                        ),
                    ]));
                }

                let def_text = sense.english_definitions.join("; ");
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {}. ", def_count),
                        Style::new().fg(Color::DarkGray),
                    ),
                    Span::raw(def_text),
                ]));

                def_count += 1;
                lines.push(Line::from(""));
            }

            Text::from(lines)
        } else {
            Text::from("Data error")
        }
    } else {
        Text::from("No word selected.")
    };

    let word_paragraph = Paragraph::new(word_content)
        .block(word_block)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .scroll((app.scroll, 0));

    f.render_widget(word_paragraph, body_chunks[1]);

    if let Focus::SearchBar = app.focus {
        f.set_cursor_position((
            chunks[0].x + app.input.chars().count() as u16 + 1,
            chunks[0].y + 1,
        ));
    }
}
