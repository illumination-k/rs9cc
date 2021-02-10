use std::{iter::Peekable, ops::Deref};

use crate::token::*;



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    NdADD, // +
    NdSub, // -
    NdMul, // *
    NdDiv, // /
    NdEq, // ==
    NdNe, // !=
    NdLt, // <
    NdLe, // <=
    NdNum, // Interger
}

#[derive(Debug, Clone)]
pub struct Node {
    node_kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: Option<String>,
}

impl Node {
    fn new(node_kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>, val: Option<String>) -> Self {
        Self {
            node_kind,
            lhs,
            rhs,
            val: val
        }
    }

    fn new_binary(node_kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Node::new(node_kind, Some(lhs), Some(rhs), None)
    }

    fn new_binary_with_box(node_kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Node::new_binary(node_kind, lhs, rhs))
    }

    fn new_num_node<S: ToString>(val: S) -> Self {
        Self {
            node_kind: NodeKind::NdNum,
            lhs: None,
            rhs: None,
            val: Some(val.to_string())
        }
    }

    fn new_num_node_with_box<S: ToString>(val: S) -> Box<Self> {
        Box::new(Node::new_num_node(val))
    }

    pub fn kind(&self) -> &NodeKind {
        &self.node_kind
    }

    pub fn val(&self) -> Option<String> {
        self.val.clone()
    }

    pub fn rhs(&self) -> Option<Box<Node>> {
        self.rhs.clone()
    }

    pub fn lhs(&self) -> Option<Box<Node>> {
        self.lhs.clone()
    }
}

pub fn expr(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    equaility(tokenizer)
}

pub fn equaility(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    let mut node = relational(tokenizer);

    loop {
        if consume("==", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdEq, node, relational(tokenizer))
        } else if consume("!=", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdNe, node, relational(tokenizer))
        } else {
            return node
        }
    }
}

pub fn relational(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    let mut node = add(tokenizer);

    loop {
        if consume("<", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdLt, node, add(tokenizer))
        } else if consume("<=", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdLe, node, add(tokenizer))
        } else if consume(">", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdLt, add(tokenizer), node)
        } else if consume("<=", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdLe, add(tokenizer), node)
        } else {
            return node
        }
    }
}

pub fn add(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    let mut node = mul(tokenizer);
    loop {
        if consume("+", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdADD, node, mul(tokenizer))
        } else if consume("-", tokenizer) {
            node = Node::new_binary_with_box(NodeKind::NdSub, node, mul(tokenizer))
        } else {
            return node
        }
    }

}

pub fn mul(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    let mut node = unary(tokenizer);
    loop {
        if consume("*", tokenizer) {
            node = Box::new(Node::new(
                NodeKind::NdMul, Some(node), Some(unary(tokenizer)), None
            ))
        } else if consume("/", tokenizer) {
            node = Box::new(Node::new(
                NodeKind::NdDiv, Some(node), Some(unary(tokenizer)), None
            ))
        } else {
            return node
        }
    }
}

pub fn unary(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    if consume("+", tokenizer) {
        return primary(tokenizer)
    }

    if consume("-", tokenizer) {
        return Node::new_binary_with_box(NodeKind::NdSub, Node::new_num_node_with_box(0), primary(tokenizer))
    }

    primary(tokenizer)
}

pub fn primary(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    if consume("(", tokenizer) {
        let node = expr(tokenizer);
        let _expect = consume(")", tokenizer);
        return node 
    }

    match tokenizer.peek() {
        Some(t) => {
            if t.token_kind != TokenKind::TkNum {
                return expr(tokenizer)
            }
        }
        None => { unreachable!() }
    }

    Box::new(Node::new_num_node(tokenizer.next().expect("NdNum is expected").val))
}

pub fn gen(node: &Box<Node>) {
    // dbg!(node);
    if node.deref().node_kind == NodeKind::NdNum {
        // println!("NdNum");
        // dbg!(node);
        println!("  push {}", node.val.clone().expect("Not val in NdNum node"));
        return;
    }

    gen(&node.deref().lhs.clone().expect("msg"));
    gen(&node.deref().rhs.clone().expect("msg"));
    
    println!("  pop rdi");
    println!("  pop rax");

    match node.deref().node_kind {
        NodeKind::NdADD => { println!("  add rax, rdi")},
        NodeKind::NdSub => { println!("  sub rax, rdi")},
        NodeKind::NdMul => { println!("  imul rax, rdi")},
        NodeKind::NdDiv => {
            println!("  cqo");
            println!("  idiv rdi");
        },
        _ => {}
    }

    println!("  push rax");
}

#[allow(unreachable_patterns)]
pub fn get_val(node: &Node) -> String {
    match node.kind() {
        NodeKind::NdADD => { "plus".to_string() },
        NodeKind::NdSub => { "sub".to_string() },
        NodeKind::NdDiv => { "div".to_string()},
        NodeKind::NdMul => { "mul".to_string() },
        NodeKind::NdEq => { "eq".to_string() },
        NodeKind::NdNe => { "ne".to_string() },
        NodeKind::NdLe => { "le".to_string() },
        NodeKind::NdLt => { "lt".to_string() },
        NodeKind::NdNum => { node.val().unwrap() },
        _ => { unimplemented!() }
    }
} 

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_node() {
        let s = "1 + 2 * (3 - 1)".to_string();
        let mut tokenizer = s.tokenize().peekable();

        let node = expr(&mut tokenizer);
        dbg!(node);
    }

    #[test]
    fn test_gen() {
        let s = "-2*3+4*5".to_string();
        let mut tokenizer = s.tokenize().peekable();

        let node = expr(&mut tokenizer);
        
        gen(&node);

    }

    #[test]
    fn test_unary_node() {
        let s = "- - 10".to_string();
        let mut tokenizer= s.tokenize().peekable();
        let node = expr(&mut tokenizer);
        dbg!(&node);
        gen(&node);
    }

    #[test]
    fn test_eq() {
        let s = "9+(-1+2)==10".to_string();
        let mut tokenizer = s.tokenize().peekable();
        for t in s.tokenize() {
            println!("{:?}", t);
        }
        let node = expr(&mut tokenizer);
        dbg!(&node);
    }
}