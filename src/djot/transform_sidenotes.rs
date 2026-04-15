use std::borrow::Cow;
use std::collections::{BTreeSet, HashSet};

use jotdown::{AttributeKind, Attributes, Container, Event};
use maud::html;
use once_cell::sync::Lazy;
use rand::distr::{Alphanumeric, SampleString};

pub struct TransformSidenotes<'a, I: Iterator<Item = Event<'a>>> {
    parent: I,
    event_queue: Vec<Event<'a>>,
    used_keys: BTreeSet<String>,
}

impl<'a, I: Iterator<Item = Event<'a>>> TransformSidenotes<'a, I> {
    pub fn new(parent: I) -> Self {
        let mut used_keys = BTreeSet::new();
        used_keys.insert(String::new());
        Self {
            parent,
            event_queue: vec![],
            used_keys,
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for TransformSidenotes<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.event_queue.pop() {
            return Some(event);
        }

        let event = self.parent.next()?;
        let attrs = match &event {
            Event::Start(Container::Span, attrs) => attrs,
            _ => return Some(event),
        };

        let mut remaining_events_for_sidenote = Vec::new();
        loop {
            let next_event = self.parent.next()?;
            remaining_events_for_sidenote.push(next_event.clone());
            if matches!(next_event, Event::End(Container::Span)) {
                break;
            }
        }

        if !attrs.contains(&(AttributeKind::Class, "sidenote".into())) {
            return Some(event);
        }

        let mut random_key = String::new();
        while self.used_keys.contains(&random_key) {
            random_key = Alphanumeric.sample_string(&mut rand::rng(), 8);
        }
        self.used_keys.insert(random_key.clone());

        let html = html! {
            label for=(format!("sn-{random_key}")) class="margin-toggle sidenote-number" {}
            input type="checkbox" id=(format!("sn-{random_key}")) class="margin-toggle";
        }
        .into_string();

        for remaining_events in remaining_events_for_sidenote.into_iter().rev() {
            self.event_queue.push(remaining_events);
        }
        self.event_queue.push(event.clone());
        self.event_queue
            .push(Event::End(Container::RawInline { format: "html" }));
        self.event_queue.push(Event::Str(Cow::Owned(html)));


        Some(Event::Start(
            Container::RawInline { format: "html" },
            Attributes::default(),
        ))
    }
}
