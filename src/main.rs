mod app;
mod models;
mod ui;

use crate::app::App;
use crate::models::JishoResponse;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = JishoResponse::search("hito").await?;
    let mut terminal = ratatui::init();
    let app = App::new(response.data);
    let res = run_app(&mut terminal, app);

    ratatui::restore();
    res
}

fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()?
            && key.kind == event::KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }
}
