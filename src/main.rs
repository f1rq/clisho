mod app;
mod models;
mod ui;

use crate::app::{App, InputMode};
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

async fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()?
            && key.kind == event::KeyEventKind::Press {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        if !app.input.is_empty() {
                            let response = JishoResponse::search(&app.input).await?;
                            app.results = response.data;
                            app.list_state.select(Some(0));
                        }
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}
