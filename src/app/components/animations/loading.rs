//!
//! Loading animations.
//!
//! These must always give an output of fixed width
//! (as to not move whilst ticking).
//!

use std::{any::Any, cell::RefCell};

use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    widgets::{Paragraph, Widget},
};

use crate::app::components::State;

///
/// Encapsulates all possible states of a spinner.
///
pub(crate) trait Loading: State {
    ///
    /// External state, like % done.
    ///
    type Ext;

    fn new(state: Self::Ext) -> Self;

    fn tick(&self, ext: Self::Ext) -> impl Widget;
}

#[derive(Debug)]
pub struct Wave<const N: usize>(RefCell<[usize; N]>);

impl<const N: usize> Wave<N> {
    const STATES: &'static [char] = &[
        ' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', '█', '█', '▇', '▆', '▅', '▄', '▃', '▂', '▁',
    ];
}

impl<const N: usize> Default for Wave<N> {
    fn default() -> Self {
        Self(RefCell::new([0; N]))
    }
}

pub struct WaveW<'a, const N: usize>(&'a dyn Any);

impl<'a, const N: usize> WaveW<'a, N> {}

impl<'a, const N: usize> Widget for WaveW<'a, N> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = self.0.downcast_ref::<Wave<N>>().unwrap();
        let waves =
            s.0.borrow()
                .iter()
                .map(|i| Wave::<N>::STATES.get(*i).unwrap())
                .fold(String::new(), |mut s, b| {
                    s.push(*b);
                    s
                });

        Paragraph::new(waves)
            .alignment(Alignment::Center)
            .render(area, buf)
    }
}

impl<const N: usize> State for Wave<N> {
    fn widget(&self) -> impl Widget {
        WaveW::<N>(self)
    }
}

impl<const N: usize> Loading for Wave<N> {
    type Ext = ();

    fn new(_: Self::Ext) -> Self {
        Self(RefCell::new([0; N]))
    }

    fn tick(&self, _: Self::Ext) -> impl Widget {
        let mut arr = self.0.borrow_mut();
        let inc_from = (arr
            .iter()
            .enumerate()
            .filter(|(_, v)| **v > 0)
            .map(|(i, _)| i)
            .next()
            .unwrap_or(N) as isize
            - 1)
        .max(0) as usize;

        arr[inc_from..].iter_mut().for_each(|v| {
            *v = (*v + 1) % Self::STATES.len();
        });

        self.widget()
    }
}
