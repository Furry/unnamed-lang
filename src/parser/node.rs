use super::span::Span;

pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
    pub children: Vec<Node>,
}

pub enum NodeKind {
    Root,
    Expression,
    Function,
    Block,
    Declaration
}

impl Node {
    pub fn new(kind: NodeKind, span: Span) -> Node {
        Node {
            kind: kind,
            span: span,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}