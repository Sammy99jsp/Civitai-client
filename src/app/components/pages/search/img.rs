use std::{
    any::Any,
    borrow::Borrow,
    cell::RefCell,
    fs,
    ops::{Deref, DerefMut},
};

use anyhow::anyhow;
use image::DynamicImage;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap, StatefulWidget},
};
use ratatui_image::{picker::Picker, protocol::ResizeProtocol, Resize, ResizeImage};

use crate::app::components::{animations::loading::Wave, icons, PolledFuture, State};

struct ConsoleImg(Box<dyn ResizeProtocol>);

async fn get_image(url: String) -> anyhow::Result<ConsoleImg> {
    let url = url.to_string();

    let r = reqwest::get(&url).await?;
    let b = r.bytes().await?;
    let fname = url.split('/').last().unwrap();
    fs::write(format!("test_{}", fname), b.clone()).unwrap();
    let ext = fname.split('.').last().unwrap();
    let reader = std::io::Cursor::new(b.to_vec());
    let img = image::io::Reader::with_format(
        reader,
        image::ImageFormat::from_extension(ext)
            .map(Ok)
            .unwrap_or(Err(anyhow!("Unknown file type: {ext}")))?,
    )
    .decode()?;
    let mut picker = Picker::from_termios()
        .map_err(|a| anyhow!("{a}"))?;
    picker.guess_protocol();
    Ok(ConsoleImg(picker.new_resize_protocol(img)))
}

pub struct Img {
    contents: PolledFuture<anyhow::Result<ConsoleImg>>,
    loading: Wave<5>,
}

impl Img {
    pub fn new(url: impl ToString) -> Self {
        Self {
            contents: PolledFuture::wrap(get_image(url.to_string())),
            loading: Default::default(),
        }
    }
}

impl State for Img {
    fn widget(&self) -> impl ratatui::widgets::Widget + '_ {
        ImgW(self)
    }
}

pub struct ImgW<'a>(&'a dyn Any);

impl<'a> Widget for ImgW<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.0.downcast_ref::<Img>().unwrap();
        let block = Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .title(format!(" {} Image", icons::IMAGE));
        let inner = block.inner(area);
        block.render(area, buf);

        if !s.contents.ready() {
            // Loading still

            s.loading.widget().render(inner, buf)
        } else {
            let mut img = s.contents.inner();
            let img = img.as_mut().unwrap();

            if let Err(ref err) = img {
                let vlayout = Layout::new()
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Min(5),
                        Constraint::Percentage(50),
                    ])
                    .direction(Direction::Vertical)
                    .split(inner);

                Paragraph::new(err.to_string())
                    .style(Style::new().light_red())
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true })
                    .render(vlayout[1], buf);

                return;
            }

            if let Ok(img) = img {
                let image = ResizeImage::new(None);
                image.render(inner, buf, &mut img.0) 
            }
        }
    }
}
