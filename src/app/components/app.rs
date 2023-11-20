//!
//! The root element of the app.
//!
//! Handles global hotkeys (ESC menu), and screen layout.
//!

use std::io::Stdout;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lazy_static::lazy_static;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
    Frame, Terminal,
};
use tokio::sync::OnceCell;

use super::{navbar::NavBar, pages::{splash::Splash, search::Search}, textbox::TextBox, State};

type Term = Terminal<CrosstermBackend<Stdout>>;

pub enum Pages {
    Splash(Splash),
    Search(Search),
}

impl Pages {
    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match self {
            Pages::Splash(ref s) => frame.render_widget(s.widget(), area),
            Pages::Search(ref s) => frame.render_widget(s.widget(), area),
        }
    }

    fn input(&mut self, event: Event) {
        match self {
            Pages::Splash(ref mut s) => s.input(event),
            Pages::Search(ref mut s) => s.input(event),
        }
    }

    fn focus(&mut self) {
        match self {
            Pages::Splash(ref mut s) => s.focus(),
            Pages::Search(ref mut s) => s.focus(),
        }
    }

    fn unfocus(&mut self) {
        match self {
            Pages::Splash(ref mut s) => s.unfocus(),
            Pages::Search(ref mut s) => s.unfocus(),
        }
    }
}

pub struct App {
    terminal: Term,
    // // // // // // // //
    ///
    /// Main page content.
    ///
    page: Pages,

    esc_navbar: NavBar,

    esc: bool,
}

lazy_static! {
    static ref INITED: OnceCell<()> = OnceCell::new();
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        if !INITED.initialized() {
            // Setup terminal
            enable_raw_mode()?;
            let mut stdout = std::io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            let terminal = Terminal::new(backend)?;

            return Ok(Self {
                terminal,
                page: Pages::Splash(Splash),
                esc_navbar: NavBar::default(),
                esc: false,
            });
        }

        Err(anyhow::format_err!("Already setup app!"))
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.terminal.draw(|frame| {
                let layout = Layout::new()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100), Constraint::Min(2)])
                    .split(frame.size());

                self.page.render(frame, layout[0]);
                frame.render_widget(self.esc_navbar.widget(), layout[1]);
            })?;

            if event::poll(std::time::Duration::from_millis(50))? {
                let ev = event::read()?;
                if let Event::Key(KeyEvent {
                    kind: KeyEventKind::Press | KeyEventKind::Repeat,
                    code,
                    ..
                }) = ev
                {
                    match code {
                        KeyCode::Esc => {
                            self.handle_esc();
                            continue;
                        },
                        KeyCode::Char('q') if self.esc => break,
                        KeyCode::Char('s') if self.esc => {
                            self.page = Pages::Search(Search::new());
                            continue;
                        },
                        _ => {}
                    }
                }

                self.page.input(ev);
            }
        }

        Ok(())
    }

    fn handle_esc(&mut self) {
        if self.esc {
            self.esc = false;
            self.esc_navbar.unfocus();
            self.page.focus();
        } else {
            self.esc = true;
            self.esc_navbar.focus();
            self.page.unfocus();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // restore terminal
        if let std::io::Result::Err(err) = try {
            disable_raw_mode()?;
            execute!(
                self.terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            self.terminal.show_cursor()?;
        } {
            eprintln!("Error while closing:\n{err}")
        }
    }
}
