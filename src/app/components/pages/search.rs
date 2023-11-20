pub mod model;
pub mod modellist;
pub mod meta;
pub mod img;

use std::any::Any;

use crossterm::event::Event;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    widgets::Widget,
};

use crate::app::components::{
    animations::loading::{Loading, Wave},
    icons,
    textbox::TextBox,
    Component, State,
};

use self::modellist::ModelList;

pub struct Search {
    focus: bool,
    query: TextBox,
    results: ModelList,
}

impl Search {
    pub fn new() -> Self {
        Self {
            focus: false,
            query: TextBox::new(format!("{} ", icons::SEARCH)),
            results: ModelList::default(),
        }
    }
}

impl Default for Search {
    fn default() -> Self {
        Self::new()
    }
}

impl State for Search {
    fn widget(&self) -> impl Widget {
        SearchW(self)
    }

    fn input(&mut self, event: crossterm::event::Event) {
        self.query.input(event.clone());

        if let Event::Key(_) = event {
            self.results.query_update(self.query.value());
        }
    }

    fn focus(&mut self) {
        self.focus = true;
        self.query.focus();
    }

    fn unfocus(&mut self) {
        self.focus = false;
        self.query.unfocus();
    }
}

pub struct SearchW<'a>(&'a dyn Any);

impl<'a> Widget for SearchW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.0.downcast_ref::<Search>().unwrap();

        let layout = Layout::new()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Percentage(100)])
            .split(area);

        // Render search box
        s.query.widget().render(layout[0], buf);

        s.results
            .widget()
            .render(layout[1], buf);
    }
}

impl<'a> Component<'a> for SearchW<'a> {}
