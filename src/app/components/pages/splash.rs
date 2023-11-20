//!
//! Slpash screen for the app.
//! 

use std::any::Any;

use lazy_static::lazy_static;
use ratatui::{widgets::{Widget, Paragraph, Block, Borders, Padding}, prelude::{Rect, Buffer}, text::{Text, Span, Line}, style::{Color, Style}, layout::{Alignment, self, Layout, Direction, Constraint}};

use crate::app::components::{State, Component};

pub struct Splash;

impl State for Splash {
    fn widget(&self) -> impl Widget{
        SplashW(self)
    }
}

const SPLASH_RAW:  &str = include_str!("SPLASH");
pub const COLORS: [Color; 2] = [Color::Rgb(193, 194, 197), Color::Rgb(25, 113, 194)];

lazy_static! {
    static ref SPLASH: Text<'static> = Text::from(
        SPLASH_RAW.lines()
            .map(|ln| ln.split('B').enumerate().map(|(i, part)| Span::styled(part, Style::new().fg(COLORS[i % 2]))).collect::<Vec<_>>())
            .map(Line::from)
            .map(|l| l.alignment(Alignment::Center))
            .collect::<Vec<_>>()
    );
}

pub struct SplashW<'a>(&'a dyn Any);

impl<'a> Widget for SplashW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let splash = Paragraph::new(SPLASH.clone())
            .block(Block::new()
            .padding(Padding::uniform(1)));
            // .borders(Borders::ALL));
        let container = Block::new()
            .borders(Borders::NONE);

        let inner = container.inner(area);
        container.render(area, buf);

        let height = SPLASH_RAW.lines().count() as u16 + 4;
        let width = SPLASH_RAW.lines().next().unwrap().replace('B', "").len() as u16;

        let layout = Layout::new()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length((inner.height - height) / 2), Constraint::Min(height), Constraint::Length((inner.height - height) / 2)])
            .split(inner);

        let layout = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(10), Constraint::Min(width), Constraint::Length(10)])
            .split(layout[1]);

        splash.render(layout[1], buf);
    }
}

impl<'a> Component<'a> for SplashW<'a> {}