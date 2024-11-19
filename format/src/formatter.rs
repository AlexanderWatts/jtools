use ast::node::Node;

#[derive(Debug, PartialEq)]
pub struct Formatter {
    space: usize,
}

impl Default for Formatter {
    fn default() -> Self {
        Self { space: 4 }
    }
}

impl Formatter {
    pub fn new(space: usize) -> Self {
        Self { space }
    }

    pub fn format(&self, ast: Node) -> String {
        self.depth_traversal(&ast, 0)
    }

    fn depth_traversal(&self, ast: &Node, mut depth: usize) -> String {
        match ast {
            Node::Object(children) => {
                if children.is_empty() {
                    return String::from("{}");
                }

                let delimeter_spacing = " ".repeat(depth * self.space);
                depth += 1;
                let children_spacing = " ".repeat(depth * self.space);

                let mut object = String::from("{\n");

                let values = children
                    .iter()
                    .enumerate()
                    .map(|(i, child)| {
                        let mut value = String::new();
                        value.push_str(&children_spacing);
                        value.push_str(&self.depth_traversal(child, depth));

                        if i < children.len() - 1 {
                            value.push_str(",");
                        }

                        value.push_str("\n");
                        return value;
                    })
                    .collect::<String>();

                object.push_str(&values);
                object.push_str(&delimeter_spacing);
                object.push_str("}");

                depth -= 1;

                return object;
            }
            Node::Property(key, value) => {
                return format!(
                    "{}: {}",
                    self.depth_traversal(key, depth),
                    self.depth_traversal(value, depth)
                )
            }
            Node::Array(children) => {
                let delimeter_spacing = " ".repeat(depth * self.space);
                depth += 1;
                let children_spacing = " ".repeat(depth * self.space);

                if children.is_empty() {
                    return String::from("[]");
                }

                let mut array = String::from("[\n");

                let values = children
                    .iter()
                    .enumerate()
                    .map(|(i, child)| {
                        let mut value = String::new();
                        value.push_str(&children_spacing);
                        value.push_str(&self.depth_traversal(child, depth));

                        if i < children.len() - 1 {
                            value.push_str(",");
                        }

                        value.push_str("\n");
                        return value;
                    })
                    .collect::<String>();

                array.push_str(&values);
                array.push_str(&delimeter_spacing);
                array.push_str("]");

                depth -= 1;

                return array;
            }
            Node::Literal(literal) => return literal.to_string(),
        }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn format_object() {
        let ast = Node::Object(vec![Node::Property(
            Box::new(Node::Literal("\"foundTreasure\"")),
            Box::new(Node::Literal("false")),
        )]);

        let f = Formatter::default();

        assert_eq!("{\n    \"foundTreasure\": false\n}", f.format(ast));
    }

    #[test]
    fn format_array() {
        let ast = Node::Array(vec![
            Node::Array(vec![Node::Literal("true"), Node::Literal("false")]),
            Node::Literal("42"),
        ]);

        let f = Formatter::default();

        assert_eq!(
            "[\n    [\n        true,\n        false\n    ],\n    42\n]",
            f.format(ast)
        );
    }

    #[test]
    fn format_property() {
        let ast = Node::Property(
            Box::new(Node::Literal("\"message\"")),
            Box::new(Node::Literal("\"in a bottle\"")),
        );
        let f = Formatter::default();

        assert_eq!("\"message\": \"in a bottle\"", f.format(ast));
    }

    #[test]
    fn format_literal() {
        let ast = Node::Literal("true");
        let f = Formatter::default();

        assert_eq!("true", f.format(ast));
    }

    #[test]
    fn create_formatter() {
        assert_eq!(Formatter { space: 2 }, Formatter::new(2));
    }

    #[test]
    fn create_default_formatter() {
        assert_eq!(Formatter { space: 4 }, Formatter::default());
    }
}
