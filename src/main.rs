mod app;
mod models;
mod ui;

use crate::app::{App, Focus};
use crate::models::JishoResponse;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = JishoResponse::search("hito").await?;
    let mut terminal = ratatui::init();
    let app = App::new(response.data);
    let res = run_app(&mut terminal, app).await;

    ratatui::restore();
    res
}

async fn run_app(
    terminal: &mut DefaultTerminal,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()?
            && key.kind == event::KeyEventKind::Press
        {
            match app.focus {
                Focus::SearchBar => match key.code {
                    KeyCode::Tab => app.next_focus(),
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            let response = JishoResponse::search(&app.input).await?;
                            app.results = response.data;
                            app.focus = Focus::ResultsList;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.focus = Focus::ResultsList;
                    }
                    _ => {}
                },
                Focus::ResultsList => match key.code {
                    KeyCode::Tab => app.next_focus(),
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                },
                Focus::WordDetails => match key.code {
                    KeyCode::Tab => app.next_focus(),
                    _ => {}
                },
            }
        }
    }
}
