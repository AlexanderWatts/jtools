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

    pub fn format(&self, ast: Node) -> &str {
        self.depth_traversal(ast, 0)
    }

    fn depth_traversal(&self, ast: Node, depth: usize) -> &str {
        match ast {
            Node::Object(children) => return "",
            Node::Property(key, value) => return "",
            Node::Array(children) => return "",
            Node::Literal(literal) => return "",
        }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn create_formatter() {
        assert_eq!(Formatter { space: 2 }, Formatter::new(2));
    }

    #[test]
    fn create_default_formatter() {
        assert_eq!(Formatter { space: 4 }, Formatter::default());
    }
}
