mod ui;
mod input;
mod editor;
mod handler;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style, Stylize};
use ratatui::Terminal;
use std::fmt::Display;
use std::io::{BufRead, StdoutLock};
use std::ops::AddAssign;
use std::str::FromStr;
use std::{env, fmt, fs, io};
use std::time::{Duration, Instant};
use crate::editor::Editor;
use crate::handler::config_handler::ConfigHandler;

#[derive(Debug, Copy, Clone)]
enum Mode {
    Normal,
    Insert,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Command => write!(f, "COMMAND"),
        }
    }
}

#[derive(Debug)]
struct Command {
    content: String,
    result: Option<String>,
}

impl Command {
    fn new() -> Self {
        Self {
            content: String::new(),
            result: None,
        }
    }

    fn clear(&mut self) {
        self.content.clear();
    }

    fn pop(&mut self) {
        self.content.pop();
    }
}

impl AddAssign<&str> for Command {
    fn add_assign(&mut self, rhs: &str) {
        self.content.push_str(rhs);
    }
}

#[derive(Debug)]
struct Toast {
    message: String,
    start_time: Instant,
    duration: Duration,
}

#[derive(Debug)]
struct Mosaic<'a> {
    mode: Mode,
    should_quit: bool,
    command: Command,
    toast: Option<Toast>,
    editors: Vec<Editor<'a>>,
    current_editor: usize,
}

impl<'a> Mosaic<'a> {
    fn new(mode: Mode, editor: Editor<'a>) -> Self {
        Self {
            mode,
            should_quit: false,
            command: Command::new(),
            toast: None,
            editors: vec![editor],
            current_editor: 0,
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.command.clear();
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn show_toast(&mut self, message: &str, duration: Duration) {
        let toast = Toast {
            message: message.to_string(),
            start_time: Instant::now(),
            duration,
        };

        self.toast = Some(toast);
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut file_path: Option<String> = None;
    let mut initial_content = String::new();
    
    if let Some(arg1) = env::args().nth(1) {
        file_path = Some(arg1.clone());
        match fs::read_to_string(&arg1) {
            Ok(content) => {
                initial_content = content;
            }
            Err(_) => {
                initial_content = String::new();
            }
        }
    }

    //text_area.set_line_number_style(Style::default().fg(Color::DarkGray));
    //text_area.set_tab_length(4);

    let mosaic = Mosaic::new(Mode::Normal, Editor::new(initial_content.as_str(), file_path));

    let command_handler = handler::command_handler::CommandHandler::new();
    let mut config_handler = ConfigHandler::new(command_handler);
    config_handler.load_config();

    let res = run(&mut terminal, mosaic);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run(terminal: &mut Terminal<CrosstermBackend<StdoutLock>>, mut mosaic: Mosaic) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            ui::draw(frame, &mut mosaic); // pass immutable state
        })?;

        if mosaic.toast.is_some() {
            let toast = mosaic.toast.as_ref().unwrap();
            if toast.start_time.elapsed() >= toast.duration {
                mosaic.toast = None;
            }
        }

        input::handle(&mut mosaic).expect("TODO: panic message");

        if mosaic.should_quit {
            break Ok(());
        }
    }
}