mod app;
mod panel;
mod workspace;
mod floating_panel;
mod panel_builtin;

use std::io::stdout;
use std::time::Duration;
use crossterm::event;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crate::app::Mos;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = Mos::new();

    loop {
        while event::poll(Duration::from_millis(0))? {
            let ev = event::read()?;
            app.handle_terminal_event(ev);
        }

        app.update();

        terminal.draw(|frame| {
            app.render(frame);
        })?;

        //  std::thread::sleep(Duration::from_millis(16));
    }
}
