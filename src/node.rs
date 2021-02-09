use std::{iter::Peekable, ops::Deref};

use crate::token::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    NdADD,
    NdSub,
    NdMul,
    NdDiv,
    NdNum
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

pub fn mul(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    let mut node = primary(tokenizer);
    loop {
        if consume("*", tokenizer) {
            node = Box::new(Node::new(
                NodeKind::NdMul, Some(node), Some(primary(tokenizer)), None
            ))
        } else if consume("/", tokenizer) {
            node = Box::new(Node::new(
                NodeKind::NdDiv, Some(node), Some(primary(tokenizer)), None
            ))
        } else {
            return node
        }
    }
}

pub fn expr(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    let mut node = mul(tokenizer);

    loop {
        if consume("+", tokenizer) {
            node = Box::new(Node::new(NodeKind::NdADD, Some(node), Some(mul(tokenizer)), None));
        } else if consume("-", tokenizer) {
            node = Box::new(Node::new(NodeKind::NdSub, Some(node), Some(mul(tokenizer)), None));
        } else {
            return node
        }
    }
}

pub fn primary(tokenizer: &mut Peekable<TokenIter>) -> Box<Node> {
    if consume("(", tokenizer) {
        let node = expr(tokenizer);
        let _expect = consume(")", tokenizer);
        return node 
    }

    Box::new(Node::new(NodeKind::NdNum, None, None, Some(tokenizer.next().unwrap().val)))
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
        }
        _ => {}
    }

    println!("  push rax");
}

#[warn(unreachable_patterns)]
pub fn get_val(node: &Node) -> String {
    match node.kind() {
        NodeKind::NdADD => { "plus".to_string() },
        NodeKind::NdSub => { "sub".to_string() },
        NodeKind::NdDiv => { "div".to_string()},
        NodeKind::NdMul => { "mul".to_string() },
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
        let s = "2*3+4*5".to_string();
        let mut tokenizer = s.tokenize().peekable();

        let node = expr(&mut tokenizer);
        
        gen(&node);
    }
}