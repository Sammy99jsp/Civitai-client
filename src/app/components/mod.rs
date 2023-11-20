pub mod animations;
pub mod app;
pub mod navbar;
pub mod pages;
pub mod textbox;

use std::{
    cell::{RefCell, RefMut},
    fmt::Display,
    ops::{Deref, DerefMut},
    task::{Context, Poll, Waker},
};

use crossterm::event::Event;
use futures::{future::BoxFuture, task::noop_waker_ref, Future, FutureExt};
use ratatui::{
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Paragraph, Widget},
};

///
/// The state of a reusable UI component.
///
pub(crate) trait State {
    fn widget(&self) -> impl Widget + '_;

    fn input(&mut self, event: Event) {}

    fn focus(&mut self) {}
    fn unfocus(&mut self) {}
}

///
/// Wraps a future and allows it to be easily pollet from within the UI code.
///
struct PolledFuture<T> {
    waker: &'static Waker,
    thing: RefCell<Option<T>>,
    fut: RefCell<Option<BoxFuture<'static, T>>>,
}

impl<T> PolledFuture<T> {
    fn inner(&self) -> impl DerefMut<Target = Option<T>> + '_ {
        RefMut::map(self.thing.borrow_mut(), |a| a)
    }

    fn ready(&self) -> bool {
        if self.thing.borrow().is_some() {
            // return self.thing.borrow().as_ref();
            return true;
        }

        let ctx = &mut Context::from_waker(self.waker);
        let mut fut = match self.fut.try_borrow_mut() {
            Err(_) => return false,
            Ok(o) => o,
        };

        let res = fut.as_mut().and_then(|fut| match fut.poll_unpin(ctx) {
            Poll::Ready(t) => Some(t),
            Poll::Pending => None,
        });

        if let Some(thing) = res {
            self.thing.borrow_mut().replace(thing);
            fut.take();
            return true;
        }

        false
    }

    fn wrap(fut: impl Future<Output = T> + Send + 'static) -> Self {
        Self {
            waker: noop_waker_ref(),
            thing: Default::default(),
            fut: RefCell::new(Some(fut.boxed())),
        }
    }
}

///
/// Representing a UI element, derived from its state.
///
/// [Deref] is implemented for your convenience,
/// so `self` can refer to your state.
///
/// ```ignore
/// use std::any::Any;
/// use ratatui::{widgets::{Widget, Paragraph}, prelude::{Rect, Buffer}};
/// use super::{Component, State};
///
/// pub struct Icon {
///     icon: char
/// };
///
/// impl State for Icon {}
///
/// pub struct IconW<'a>(&'a dyn Any);
///
/// impl<'a> Widget for Icon<'a> {
///     fn render(self, area: Rect, buf: &mut Buffer) {
///         let s: &Icon = self.0.downcast_ref().unwrap();
///
///         Paragraph::new(s.icon.to_string())
///             .render(area, buf);
///     }
/// }  
/// ```
///
pub(crate) trait Component<'a>: Widget {}

#[derive(Debug, Clone, Copy)]
pub struct Icon(char);

impl Widget for Icon {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.0.to_string())
            .style(Style::new().reset().bold())
            .render(area, buf)
    }
}

impl Deref for Icon {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

mod icons {
    use super::Icon;

    pub const QUIT: Icon = Icon('\u{f0a48}');
    pub const HELP: Icon = Icon('\u{f02d6}');
    pub const SEARCH: Icon = Icon('\u{ea6d}');
    pub const HASH: Icon = Icon('\u{f4df}');
    pub const IMAGE: Icon = Icon('\u{f02e9}');
}
