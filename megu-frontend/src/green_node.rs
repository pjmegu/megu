use rowan::*;

use crate::ast::event::Event;

pub fn make_green_node<'a, 'source: 'a>(
    events: impl Iterator<Item = &'a Event<'source>>,
) -> GreenNode {
    let mut builder = GreenNodeBuilder::new();
    
    for event in events {
        match event {
            Event::Node(kind) => builder.start_node(rowan::SyntaxKind((*kind).into())),
            Event::Token(kind, text) => builder.token(rowan::SyntaxKind((*kind).into()), text),
            Event::FinNode => builder.finish_node(),
        }
    }

    builder.finish()
}
