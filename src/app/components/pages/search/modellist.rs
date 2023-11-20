use std::{
    any::Any,
    cell::RefCell,
    task::{Context, Waker},
};

use civitai_tui::api::{
    self,
    endpoints::{models::Params, Endpoint},
    paginated::Paginated,
    send_request,
    types::model::Mode,
};
use futures::{
    future::BoxFuture,
    stream::BoxStream,
    task::{noop_waker, noop_waker_ref},
    FutureExt, TryFutureExt,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Paragraph, Widget},
};
use tokio::time::Instant;

use crate::app::components::{
    animations::loading::{Loading, Wave},
    pages::search::meta::MetaW,
    PolledFuture, State,
};

use super::model::Model;

const WAVE_LENGTH: usize = 6;
const MAX_MODELS: usize = 3;

pub struct ModelList {
    loaders: [Wave<WAVE_LENGTH>; 2],
    last_req: Instant,
    list: Option<PolledFuture<anyhow::Result<(Vec<Model>, api::PaginationMeta)>>>,
}

async fn map_models(
    models: Paginated<api::types::Model>,
) -> anyhow::Result<(Vec<Model>, api::PaginationMeta)> {
    let (pages, meta) = models.first_few();
    Ok((
        pages
            .take(MAX_MODELS)
            .cloned()
            .enumerate()
            .map(|(i, data)| Model::new(data, i == 0))
            .collect(),
        meta.clone(),
    ))
}

impl Default for ModelList {
    fn default() -> Self {
        Self {
            loaders: Default::default(),
            last_req: Instant::now(),
            list: Default::default(),
        }
    }
}

impl ModelList {
    pub fn query_update(&mut self, query: &str) {
        let now = Instant::now();

        // Only send requests max every three mins.
        if now.duration_since(self.last_req).as_secs() < 3 {
            return;
        }

        self.last_req = now;

        self.list.replace(PolledFuture::wrap(
            api::endpoints::models::models::get(Params {
                query: query.to_string().into(),
                ..Default::default()
            })
            .and_then(map_models),
        ));
    }
}

impl State for ModelList {
    fn widget(&self) -> impl ratatui::widgets::Widget + '_ {
        ModelListW(self)
    }
}

pub struct ModelListW<'a>(&'a dyn Any);

impl<'a> Widget for ModelListW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s: &ModelList = self.0.downcast_ref().unwrap();

        let list = s.list.as_ref();

        if list.is_none() {
            Paragraph::new("Type a query to get results.")
                .alignment(Alignment::Center)
                .render(area, buf);

            return;
        }

        let list = list.unwrap();

        if !list.ready() {
            // LOADING SCREEEN
            let layout = Layout::new()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Min(WAVE_LENGTH as u16),
                    Constraint::Percentage(50),
                    Constraint::Min(WAVE_LENGTH as u16),
                    Constraint::Percentage(25),
                ])
                .split(area);

            s.loaders[0].tick(()).render(layout[1], buf);
            s.loaders[1].tick(()).render(layout[3], buf);

            Paragraph::new("LOADING")
                .alignment(Alignment::Center)
                .render(layout[2], buf);

            return;
        }

        let list = list.inner();
        let list = list.as_ref().unwrap().as_ref();
        // Should have list defined by now.

        let (items, meta) = match list {
            Ok(list) => list,
            Err(err) => {
                Paragraph::new(format!("Encountered an error:\n{err}"))
                    .style(Style::new().light_red())
                    .alignment(Alignment::Center);

                return;
            }
        };

        let layout = Layout::new()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(2)])
            .split(area);

        let items_layout = Layout::new()
            .direction(Direction::Vertical)
            .constraints(items.iter().map(|_| Constraint::Percentage(25)).collect::<Vec<_>>())
            .split(layout[0]);

        // Render metadata last line.
        MetaW(meta).render(layout[1], buf);

        items
            .iter()
            .zip(items_layout.iter())
            .enumerate()
            .for_each(|(i, (model, area))| model.widget().render(*area, buf))

        // Show list of models here.
        // Paragraph::new("TODO: Got response from CivitAI!")
        //     .alignment(Alignment::Center)
        //     .render(area, buf);

        // panic!("{:?}", list)
    }
}
