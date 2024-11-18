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

    fn depth_traversal(&self, ast: &Node, depth: usize) -> String {
        match ast {
            Node::Object(children) => return "".to_string(),
            Node::Property(key, value) => {
                return format!(
                    "{}: {}",
                    self.depth_traversal(key, depth),
                    self.depth_traversal(value, depth)
                )
            }
            Node::Array(children) => return "".to_string(),
            Node::Literal(literal) => return literal.to_string(),
        }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

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
