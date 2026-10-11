use crate::{ast::NodeKind, token::TokenInner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event<'source> {
    Node(NodeKind),
    Token(NodeKind, &'source str),
    FinNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventVec<'source>(Vec<EventVecItem<'source>>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventVecItem<'source> {
    Event(Event<'source>),
    Vector(EventVec<'source>),
}

pub struct EventVecIter<'source, 'a> {
    vec: Vec<&'a Event<'source>>,
}

impl<'source> EventVec<'source> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_event(&mut self, event: Event<'source>) {
        self.0.push(EventVecItem::Event(event));
    }

    pub fn push_vector(&mut self, vector: EventVec<'source>) {
        self.0.push(EventVecItem::Vector(vector));
    }

    pub fn iter(&self) -> EventVecIter<'source, '_> {
        let mut vec = Vec::new();
        self.flatten(&mut vec);
        EventVecIter { vec }
    }

    fn flatten<'a>(&'a self, vec: &mut Vec<&'a Event<'source>>) {
        for item in &self.0 {
            match item {
                EventVecItem::Event(event) => vec.push(event),
                EventVecItem::Vector(vector) => vector.flatten(vec),
            }
        }
    }

    fn flatten_into<'a>(&'a self, vec: &mut Vec<Event<'source>>) {
        for item in &self.0 {
            match item {
                EventVecItem::Event(event) => vec.push(*event),
                EventVecItem::Vector(vector) => vector.flatten_into(vec),
            }
        }
    }

    pub fn push_token<'a>(&mut self, kind: NodeKind, text: &'a TokenInner<'source>) {
        if !text.leading_trivia().is_empty() {
            self.push_event(Event::Token(
                NodeKind::Trivia,
                str::from_utf8(text.leading_trivia()).unwrap(),
            ));
        }
        self.push_event(Event::Token(kind, str::from_utf8(text.token()).unwrap()));
        if !text.trailing_trivia().is_empty() {
            self.push_event(Event::Token(
                NodeKind::Trivia,
                str::from_utf8(text.trailing_trivia()).unwrap(),
            ));
        }
    }

    pub fn push_node(&mut self, kind: NodeKind, f: impl FnOnce(&mut EventVec<'source>)) {
        self.push_event(Event::Node(kind));
        f(self);
        self.push_event(Event::FinNode);
    }

    pub fn new_with(f: impl FnOnce(&mut EventVec<'source>)) -> Self {
        let mut event_vec = EventVec::new();
        f(&mut event_vec);
        event_vec
    }
}

impl<'source> IntoIterator for EventVec<'source> {
    type Item = Event<'source>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        self.flatten_into(&mut vec);
        vec.into_iter()
    }
}

impl<'source, 'a: 'source> Iterator for EventVecIter<'source, 'a> {
    type Item = &'source Event<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }
}

impl<'source> FromIterator<EventVec<'source>> for EventVec<'source> {
    fn from_iter<T: IntoIterator<Item = EventVec<'source>>>(iter: T) -> Self {
        let mut event_vec = EventVec::new();
        for event in iter {
            event_vec.push_vector(event);
        }
        event_vec
    }
}

#[macro_export]
/*
event_vec! {
    NodeKind::Root {
        NodeKind::Def {
            NodeKind::Ident {
                token(NodeKind::TIdent, "foo");
            }
        }
    }
}
*/
macro_rules! event_vec {
    (@items $event_vec:ident;) => {};

    (@items $event_vec:ident; token($kind:expr, $leading:expr, $text:expr, $trailing:expr); $($rest:tt)*) => {
        $event_vec.push_token($kind, &$crate::token::TokenInner::new($leading.as_bytes(), $text.as_bytes(), $trailing.as_bytes()));
        $crate::event_vec!(@items $event_vec; $($rest)*);
    };

    (@items $event_vec:ident; tokeni($kind:expr, $text:expr); $($rest:tt)*) => {
        $event_vec.push_token($kind, $text);
        $crate::event_vec!(@items $event_vec; $($rest)*);
    };

    (@items $event_vec:ident; vec($vec:expr); $($rest:tt)*) => {
        $event_vec.push_vector($vec);
        $crate::event_vec!(@items $event_vec; $($rest)*);
    };

    (@items $event_vec:ident; $kind:path { $($body:tt)* } $($rest:tt)*) => {
        $event_vec.push_node($kind, |event_vec| {
            $crate::event_vec!(@items event_vec; $($body)*);
        });
        $crate::event_vec!(@items $event_vec; $($rest)*);
    };

    ($($item:tt)*) => {{
        $crate::ast::event::EventVec::new_with(|event_vec| {
            $crate::event_vec!(@items event_vec; $($item)*);
        })
    }};
}
