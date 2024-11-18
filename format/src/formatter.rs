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

impl<'source> Formatter {
    pub fn new(space: usize) -> Self {
        Self { space }
    }

    pub fn format(&self, ast: Node<'source>) -> &'source str {
        self.depth_traversal(ast, 0)
    }

    fn depth_traversal(&self, ast: Node<'source>, depth: usize) -> &'source str {
        match ast {
            Node::Object(children) => return "",
            Node::Property(key, value) => return "",
            Node::Array(children) => return "",
            Node::Literal(literal) => return literal,
        }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

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
