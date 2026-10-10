use crate::ast::NodeKind;

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
}

impl <'source> IntoIterator for EventVec<'source> {
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
