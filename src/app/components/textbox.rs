use std::any::Any;

use super::{pages::splash::COLORS, Component, State};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget, BorderType},
};

pub struct TextBox {
    focus: bool,
    prefix: Option<String>,
    value: String,
}

impl TextBox {
    pub fn new(prefix: impl ToString) -> Self {
        Self {
            focus: false,
            prefix: Some(prefix.to_string()),
            value: Default::default(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl State for TextBox {
    fn widget(&self) -> impl Widget {
        TextBoxW(self)
    }
    fn input(&mut self, event: Event) {
        use KeyCode::{Backspace, Char, Enter};
        use KeyEventKind::*;

        if let Event::Key(KeyEvent {
            kind: Press | Repeat,
            code,
            ..
        }) = event
        {
            match code {
                Backspace => {
                    self.value.pop();
                }
                Enter => todo!(),
                Char(ch) => self.value.push(ch),
                _ => {}
            }
        }
    }

    fn focus(&mut self) {
        self.focus = true;
    }

    fn unfocus(&mut self) {
        self.focus = false;
    }
}

pub struct TextBoxW<'a>(&'a dyn Any);

impl<'a> Widget for TextBoxW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s: &TextBox = self.0.downcast_ref().unwrap();

        let raw_text = if !s.value.is_empty() {
            s.value.clone()
        } else {
            "Search models…".to_string()
        };

        let total = s.prefix.as_ref().map(String::len).unwrap_or_default() + raw_text.len();

        let mut to_display = raw_text;
        if total > area.width as usize {
            to_display = format!("…{}", &to_display[(total - (area.width as usize - 1))..]);
        }

        Paragraph::new(format!(
            "{}{}",
            s.prefix.as_ref().unwrap_or(&String::new()),
            to_display
        ))
        .block(
            Block::new()
                .borders(Borders::BOTTOM)
                .border_type(BorderType::Thick)
                .border_style(if s.focus {
                    Style::new().fg(COLORS[1])
                } else {
                    Style::reset()
                }),
        )
        .render(area, buf)
    }
}

impl<'a> Component<'a> for TextBoxW<'a> {}
