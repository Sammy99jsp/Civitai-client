//!
//! Metadata shown at the bottom
//! 

use civitai_tui::api;
use ratatui::{widgets::{Widget, Paragraph, Block, Borders, BorderType}, prelude::{Rect, Buffer}, layout::{Layout, Constraint, Alignment}, style::{Style, Stylize}};

use crate::app::components::icons;

pub struct MetaW<'a>(pub(crate) &'a api::PaginationMeta);

impl<'a> Widget for MetaW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.0;
        let total = s.total_items;

        let text = format!("{}{total} results", icons::HASH);

        Paragraph::new(text)
            .style(Style::new().bold())
            .alignment(Alignment::Right)
            .block(
                Block::new()
                    .borders(Borders::TOP)
                    .border_type(BorderType::Thick)
                    .border_style(Style::new())
            )
            .render(area, buf);
    }
}