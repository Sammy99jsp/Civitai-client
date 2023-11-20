use std::any::Any;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::app::components::{icons, pages::splash::COLORS};

use super::{Component, Icon, State};

#[derive(Debug, Clone, Copy)]
pub struct ShortcutGuide(Icon, char, &'static str);

impl Widget for ShortcutGuide {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(2),
                Constraint::Min(2),
                Constraint::Percentage(100),
            ])
            .split(area);

        self.0.render(layout[0], buf);

        Paragraph::new(self.1.to_string())
            .style(Style::new().bold().red())
            .render(layout[1], buf);

        Paragraph::new(self.2.to_string())
            .alignment(Alignment::Left)
            .render(layout[2], buf);
    }
}

#[derive(Debug, Default)]
pub struct NavBar(bool);

impl State for NavBar {
    fn widget(&self) -> impl Widget {
        NavBarW(self)
    }

    fn focus(&mut self) {
        self.0 = true;
    }

    fn unfocus(&mut self) {
        self.0 = false;
    }
}

pub struct NavBarW<'a>(&'a dyn Any);

impl<'a> Widget for NavBarW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.0.downcast_ref::<NavBar>().unwrap();

        const ELEMENTS: [ShortcutGuide; 3] = [
            ShortcutGuide(icons::SEARCH, 'S', "Search"),
            ShortcutGuide(icons::HELP, 'H', "Help"),
            ShortcutGuide(icons::QUIT, 'Q', "Quit"),
        ];

        let container = Block::new()
            .borders(Borders::TOP)
            .border_type(BorderType::Double)
            .border_style(if state.0 {
                Style::new().fg(COLORS[1])
            } else {
                Style::reset()
            });

        let inner = container.inner(area);

        container.render(area, buf);

        let els = [
            Constraint::Percentage(100),
            Constraint::Min(11),
            Constraint::Min(10),
            Constraint::Min(10),
        ];
        let layout = Layout::new()
            .direction(Direction::Horizontal)
            .constraints(els)
            .split(inner);

        if state.0 {
            Paragraph::new("ACTIVE").style(Style::new().fg(COLORS[1]).bold())
        } else {
            Paragraph::new("INACTIVE")
        }
        .render(layout[0], buf);

        ELEMENTS[0].render(layout[1], buf);
        ELEMENTS[1].render(layout[2], buf);
        ELEMENTS[2].render(layout[3], buf);
    }
}

impl<'a> Component<'a> for NavBarW<'a> {}
