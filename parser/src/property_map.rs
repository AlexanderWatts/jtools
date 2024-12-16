use std::collections::{hash_map::Entry, HashMap};

use ast::node::Node;

/// Store object properties in insertion order
///
/// ## Description
///
/// The two rules of storing object properties:
///
/// 1) Duplicate property keys are not allowed and should return an error
/// 2) The property order must be maintained frpm a given input as the user expects
///
/// ## Why maintain a vector and a hashmap?
///
/// Before a property can be stored there must be a check to see if a key with the same name
/// already exists. This check has to be done for every property so it is desirable to have an O(1)
/// lookup which is why a `HashMap` of keys is used. Unfortunately, a `HashMap` does not maintain
/// insertion order and breaks the second rule so a vector, which does maintain order, is also
/// used. If a properties key does not exist in the `HashMap` then it is added to the vector,
/// otherwise it returns an error.
///
/// ## Examples
///
/// ```
/// use parser::property_map::PropertyMap;
/// use ast::node::Node;
///
/// let mut pm = PropertyMap::new();
///         
/// // Sucessfully adds property
/// pm.insert(
///     "one",
///     Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
/// );
///
/// // Fails to add duplicate property
/// pm.insert(
///     "one",
///     Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
/// );
/// ```
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

    pub fn insert(&mut self, key: &'source str, ast: Node<'source>) -> Option<usize> {
        match self.map.entry(key) {
            Entry::Occupied(_) => None,
            Entry::Vacant(vacant_entry) => {
                let property_position = self.ordered_properties.len();
                self.ordered_properties.push(ast);
                Some(*vacant_entry.insert(property_position))
            }
        }
    }
}

#[cfg(test)]
mod property_map_tests {
    use super::*;

    #[test]
    fn error_given_duplicates() {
        let mut pm = PropertyMap::new();

        assert_eq!(
            Some(0),
            pm.insert(
                "one",
                Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
            )
        );

        assert_eq!(
            None,
            pm.insert(
                "one",
                Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
            )
        );
    }

    #[test]
    fn maintain_insertion_order() {
        let mut pm = PropertyMap::new();

        let _ = pm.insert(
            "one",
            Node::Property(Box::new(Node::Literal("one")), Box::new(Node::Literal("1"))),
        );
        let _ = pm.insert(
            "two",
            Node::Property(Box::new(Node::Literal("two")), Box::new(Node::Literal("2"))),
        );
        let _ = pm.insert(
            "three",
            Node::Property(
                Box::new(Node::Literal("three")),
                Box::new(Node::Literal("3")),
            ),
        );
        let _ = pm.insert(
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
            pm.ordered_properties
        );
    }
}
