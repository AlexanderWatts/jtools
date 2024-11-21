use std::collections::HashMap;

use ast::node::Node;

use crate::parser_error::ParserError;

pub struct PropertyMap<'source> {
    pub ordered_properties: Vec<Node<'source>>,
    map: HashMap<&'source str, usize>,
}

impl<'source> PropertyMap<'source> {
    pub fn new() -> Self {
        Self {
            ordered_properties: vec![],
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: &'source str, ast: Node<'source>) -> Result<&Node, ParserError> {
        if self.map.contains_key(key) {
            return Err(ParserError::UnexpectedToken);
        }

        let ordered_properties_position = self.ordered_properties.len();
        self.ordered_properties.push(ast);
        self.map.insert(key, ordered_properties_position);

        Ok(&self.ordered_properties[ordered_properties_position])
    }
}

#[cfg(test)]
mod ordered_map_tests {
    use super::*;

    #[test]
    fn error_given_duplicates() {
        let mut om = PropertyMap::new();

        assert_eq!(
            Ok(&Node::Property(
                Box::new(Node::Literal("one")),
                Box::new(Node::Literal("1"))
            )),
            om.insert(
                "one",
                Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
            )
        );

        assert_eq!(
            Err(ParserError::UnexpectedToken),
            om.insert(
                "one",
                Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
            )
        );
    }

    #[test]
    fn maintain_insertion_order() {
        let mut om = PropertyMap::new();

        let _ = om.insert(
            "one",
            Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
        );
        let _ = om.insert(
            "two",
            Node::Property(Box::new(Node::Literal("two")), Box::new(Node::Literal("2"))),
        );
        let _ = om.insert(
            "three",
            Node::Property(
                Box::new(Node::Literal("three")),
                Box::new(Node::Literal("3")),
            ),
        );
        let _ = om.insert(
            "four",
            Node::Property(
                Box::new(Node::Literal("four")),
                Box::new(Node::Literal("4")),
            ),
        );

        assert_eq!(
            vec![
                Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
                Node::Property(Box::new(Node::Literal("two")), Box::new(Node::Literal("2"))),
                Node::Property(
                    Box::new(Node::Literal("three")),
                    Box::new(Node::Literal("3")),
                ),
                Node::Property(
                    Box::new(Node::Literal("four")),
                    Box::new(Node::Literal("4")),
                ),
            ],
            om.ordered_properties
        );
    }
}
