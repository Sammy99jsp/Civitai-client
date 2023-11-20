use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::Poll,
};

use futures::{future::BoxFuture, FutureExt, Stream};
use serde::{de::DeserializeOwned, Deserialize};

use crate::api::send_request;

use super::PaginationMeta;

#[derive(Debug, Deserialize)]
pub struct Paginated<T> {
    items: Vec<T>,
    metadata: PaginationMeta,
}

impl<T> Paginated<T> {
    pub fn first_few(&self) -> (impl Iterator<Item = &T> + '_, &PaginationMeta) {
        (self.items.iter(), &self.metadata)
    }

    pub fn map<U>(self, mapper: impl Fn(T) -> U) -> Paginated<U> {
        let items = self.items.into_iter().map(mapper).collect();

        Paginated::<U> {
            items,
            metadata: self.metadata
        }
    }
}

impl<'a, T: Clone + DeserializeOwned + 'a> Paginated<T> {
    pub fn into_stream(self) -> PageIterator<'a, T> {
        PageIterator {
            meta: self.metadata,
            current_item: 0,
            items: self.items,
            fut: None,
        }
    }
}

impl<'a, T> Unpin for PageIterator<'a, T> {}

pub struct PageIterator<'a, T> {
    meta: PaginationMeta,
    current_item: usize,

    items: Vec<T>,
    fut: Option<BoxFuture<'a, anyhow::Result<Paginated<T>>>>,
}

impl<'a, T: Clone + DeserializeOwned + 'a> Stream for PageIterator<'a, T> {
    type Item = T;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.current_item < self.items.len() {
            let item = self.items.get(self.current_item).cloned().unwrap();
            self.current_item += 1;
            return Poll::Ready(Some(item));
        }
        
        if self.fut.is_none() {
            // Get the new page, if it exists.
            if let Some(url) = self.meta.next_page.as_ref() {
                self.fut = Some(
                    send_request::<Paginated<T>>(
                        url.clone(),
                        []
                    )
                    .boxed(),
                );
            } else {
                return Poll::Ready(None);
            }
        }


        // If we're already waiting for a new page to arrive, wait.
        // If it's just arrived, we can update our internal state accordigly.
        if let Some(ref mut fut) = self.fut {
            let p = fut.poll_unpin(cx);
            // println!("Polling result: {:?}", if p.is_pending() {
            //     "Pending..."
            // } else {
            //     "Ready(T)"
            // });
            return p.map(|p| {
                p.ok().map(|p| {
                    self.current_item = 1;
                    self.fut = None;
                    self.items = p.items;
                    self.meta = p.metadata;

                    self.items[0].clone()
                })
            });
        }


        // Else -- no pages left, return None and be done with it.
        Poll::Ready(None)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.meta.total_items, Some(self.meta.total_items))
    }
}
