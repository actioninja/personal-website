use jotdown::{AttributeKind, Attributes, Container, Event, LinkType, SpanLinkType};
use maud::html;

pub struct TransformHeaders<'a, I: Iterator<Item = Event<'a>>> {
    parent: I,
    event_queue: Vec<Event<'a>>,
}

impl<'a, I: Iterator<Item = Event<'a>>> TransformHeaders<'a, I> {
    pub fn new(parent: I) -> Self {
        Self {
            parent,
            event_queue: vec![],
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for TransformHeaders<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.event_queue.pop() {
            return Some(event);
        }

        let (level, has_section, id, attrs) = match self.parent.next()? {
            Event::Start(
                Container::Heading {
                    level,
                    has_section,
                    id,
                },
                attrs,
            ) => (level, has_section, id, attrs),
            other => return Some(other),
        };

        let mut events = Vec::new();
        loop {
            match self.parent.next()? {
                Event::End(Container::Heading { .. }) => break,
                event => events.push(event),
            }
        }

        // let id: Cow<_> = util::to_id(&id).into();
        let level = level + 1;

        let heading = Container::Heading {
            level,
            has_section,
            id: id.clone(),
        };
        let link = Container::Link(
            format!("#{id}").into(),
            LinkType::Span(SpanLinkType::Inline),
        );
        let mut link_attrs = Attributes::new();
        link_attrs.push((AttributeKind::Class, "heading-ref".into()));
        self.event_queue.push(Event::End(heading.clone()));
        self.event_queue.push(Event::End(link.clone()));
        for event in events.into_iter().rev() {
            self.event_queue.push(event);
        }

        self.event_queue.push(Event::Start(link, link_attrs));
        Some(Event::Start(heading, attrs))
    }
}
