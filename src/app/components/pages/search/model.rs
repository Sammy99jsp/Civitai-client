use std::{any::Any, iter, ops::Deref};

use civitai_tui::api::{self, types::model::Mode, paginated::Paginated};
use futures::{TryFutureExt, Future};
use ratatui::{
    layout::{Alignment, Layout, Constraint, Direction},
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget, Padding}, style::{Style, Stylize},
};

use crate::app::components::{State, PolledFuture};

use super::img::Img;

pub struct Model {
    data: api::types::Model,
    images: PolledFuture<anyhow::Result<Vec<Img>>>,
    first: bool,
}

const IMAGES: usize = 3;

pub async fn at_most<const N: usize>(pag: Paginated<api::types::model::Image> ) -> anyhow::Result<Vec<Img>> {
    let (few, _) = pag.first_few();

    Ok(few
        .take(N)
        .map(|img| Img::new(&img.url))
        .collect())
}

impl Model {
    pub fn new(data: api::types::Model, first: bool) -> Self {
        let images = PolledFuture::wrap(data.versions[0].get_images().and_then(at_most::<IMAGES>));
        Self {
            data,
            images,
            first,
        }
    }
}

impl State for Model {
    fn widget(&self) -> impl Widget + '_ {
        ModelW(self)
    }
}

pub struct ModelW<'a>(pub &'a dyn Any);

impl<'a> Widget for ModelW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // TODO: Put name, and picture
        let s = self.0.downcast_ref::<Model>().unwrap();

        let b = if !s.first {
            // Apply border to top.
            Block::new().borders(Borders::TOP)
        } else {
            Block::new()
        };
        let inner = b.inner(area);

        b.render(area, buf);

        let mut constraints = vec![Constraint::Percentage(40)];

        if s.images.ready() {
            match s.images.inner().as_ref().unwrap() {
                Ok(v) => constraints.extend(v.iter().map(|_| Constraint::Percentage((60 / v.len()) as u16))),
                Err(_) => constraints.push(Constraint::Percentage(60)),
            };
        } else {
            constraints.push(Constraint::Percentage(60));
        }

        let layout = Layout::new()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner);

        let name = Paragraph::new(s.data.name.clone())
            .style(Style::new().bold());

        name.render(layout[0], buf);

        if !s.images.ready() {
            let vertical = Layout::new()
                .constraints([Constraint::Percentage(50), Constraint::Min(1), Constraint::Percentage(50)])
                .direction(Direction::Vertical)
                .split(layout[1]);

            Paragraph::new("Loading pictures")
                .alignment(Alignment::Center)
                .render(vertical[1], buf);

            return;
        }

        let res = &s.images;
        let mut res = res.inner();
        let res = res.as_mut().unwrap().as_mut();

        if let Err(ref err) = res {
            let vertical = Layout::new()
                .constraints([Constraint::Percentage(50), Constraint::Min(2), Constraint::Percentage(50)])
                .direction(Direction::Vertical)
                .split(layout[1]);

            Paragraph::new(format!("An error occurred: {err}"))
                .alignment(Alignment::Center)
                .render(vertical[1], buf);
        }

        if let Ok(imgs) = res {
            imgs
                .iter_mut()
                .enumerate()
                .map(|(i, img)| (i+1, img))
                .for_each(|(i, img)| img.widget().render(layout[i], buf));

        }        
    }
}
