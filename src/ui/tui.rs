use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};
use std::io;

use crate::app::App;

pub fn run(app: &App) -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let items: Vec<ListItem> = app
                .logs
                .iter()
                .map(|log| ListItem::new(format!("[{:?}] {:?}", log.level, log.message)))
                .collect();

            let list = List::new(items).block(Block::default().title("Logs").borders(Borders::ALL));
            f.render_widget(list, size);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}
