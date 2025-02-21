use ast::node::Node;

/// Minify JSON converting AST into String
///
/// ## Description
///
/// The minifier converts an AST into a string of JSON without formatting using depth traversal,
/// O(n), to walk the tree.
///
/// ## Examples
/// ```
/// use ast::node::Node;
/// use format::minifier::Minifier;
///
/// let ast = Node::Array(vec![
///     Node::Array(vec![Node::Literal("true"), Node::Literal("false")]),
///     Node::Literal("42"),
/// ]);
///
/// let minifier = Minifier;
///
/// assert_eq!("[[true,false],42]", minifier.minify(&ast));
/// ```
pub struct Minifier;

impl Minifier {
    pub fn minify(&self, ast: &Node) -> String {
        self.depth_traversal(ast)
    }

    fn depth_traversal(&self, ast: &Node) -> String {
        match ast {
            Node::Object(children) => format!(
                "{{{}}}",
                children
                    .iter()
                    .enumerate()
                    .map(|(i, child)| {
                        let mut child = self.depth_traversal(child);

                        if i < children.len() - 1 {
                            child.push_str(",");
                        }

                        child
                    })
                    .collect::<String>()
            ),
            Node::Property(key, value) => format!(
                "{}:{}",
                self.depth_traversal(key),
                self.depth_traversal(value)
            ),
            Node::Array(children) => format!(
                "[{}]",
                children
                    .iter()
                    .enumerate()
                    .map(|(i, child)| {
                        let mut child = self.depth_traversal(child);

                        if i < children.len() - 1 {
                            child.push_str(",");
                        }

                        child
                    })
                    .collect::<String>()
            ),
            Node::Literal(literal) => literal.to_string(),
        }
    }
}

#[cfg(test)]
mod minifier_tests {
    use super::*;

    #[test]
    fn minify_objects() {
        let ast = Node::Object(vec![Node::Property(
            Box::new(Node::Literal("\"foundTreasure\"")),
            Box::new(Node::Literal("false")),
        )]);

        let m = Minifier;

        assert_eq!("{\"foundTreasure\":false}", m.minify(&ast));
    }

    #[test]
    fn minify_properties() {
        let ast = Node::Property(
            Box::new(Node::Literal("\"message\"")),
            Box::new(Node::Literal("\"in a bottle\"")),
        );

        let m = Minifier;

        assert_eq!("\"message\":\"in a bottle\"", m.minify(&ast));
    }

    #[test]
    fn minify_arrays() {
        let ast = Node::Array(vec![
            Node::Array(vec![Node::Literal("true"), Node::Literal("false")]),
            Node::Literal("42"),
        ]);

        let m = Minifier;

        assert_eq!("[[true,false],42]", m.minify(&ast));
    }
}
