use ast::node::Node;

pub struct Minifier;

impl Minifier {
    pub fn minify(&self, ast: &Node) -> String {
        self.depth_traversal(ast)
    }

    fn depth_traversal(&self, ast: &Node) -> String {
        match ast {
            Node::Object(vec) => todo!(),
            Node::Property(node, node1) => todo!(),
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
    fn minify_arrays() {
        let ast = Node::Array(vec![
            Node::Array(vec![Node::Literal("true"), Node::Literal("false")]),
            Node::Literal("42"),
        ]);

        let m = Minifier;

        assert_eq!("[[true,false],42]", m.minify(&ast));
    }
}
