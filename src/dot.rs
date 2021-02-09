use std::ops::Deref;

use crate::node::*;

pub struct Dot {
    counter: usize,
    node_vec: Vec<String>,
    edge_vec: Vec<String>,
}

fn make_dot_node(node_cnt: usize, node_val: String) -> String {
    format!("  {} [ label = {} ];", node_cnt, node_val)
}

fn make_dot_edge(source_cnt: usize, target_cnt: usize) -> String {
    format!("  {} -> {};", source_cnt, target_cnt)
}

impl Dot {
    pub fn new() -> Self{
        Self {
            counter: 0,
            node_vec: vec![],
            edge_vec: vec![]
        }
    }

    fn _rec_write(&mut self, node: &Box<Node>, pre_node_cnt: usize) {
        if node.lhs().is_none() && node.rhs().is_none() { return }
        let node_cnt = self.counter;
        if self.node_vec.is_empty() {
            let node_val = get_val(node.deref());
            let node_dot = make_dot_node(node_cnt, node_val);
            self.node_vec.push(node_dot);
            // self.counter += 1;
        }

        self.counter += 1;
        let rhs_val = get_val(&node.deref().rhs().unwrap());
        let rhs_cnt = self.counter;
        let rhs_dot = make_dot_node(rhs_cnt, rhs_val);
        
        self.counter += 1;
        let lhs_val = get_val(&node.deref().lhs().unwrap());
        let lhs_cnt = self.counter;
        let lhs_dot = make_dot_node(lhs_cnt, lhs_val);

        self.node_vec.push(lhs_dot);
        self.node_vec.push(rhs_dot);

        let edge_rhs = make_dot_edge(pre_node_cnt, rhs_cnt);
        let edge_lhs = make_dot_edge(pre_node_cnt, lhs_cnt);
        self.edge_vec.push(edge_rhs);
        self.edge_vec.push(edge_lhs);

        self._rec_write(&node.rhs().unwrap(), rhs_cnt);
        self._rec_write(&node.lhs().unwrap(), lhs_cnt);
    }

    pub fn write(&mut self, node: &Box<Node>) -> String {
        let mut res = vec![
            "digraph ast_tree {".to_string(),
            
        ];
        self._rec_write(node, 0);
        res.push(self.node_vec.join("\n").to_string());
        res.push(self.edge_vec.join("\n").to_string());
        res.push("}".to_string());
        res.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::*;
    #[test]
    fn test_dot_1() {
        let s = "2*3+4*5".to_string();
        let mut tokenizer = s.tokenize().peekable();

        let node = expr(&mut tokenizer);
        let mut dot = Dot::new();
        let s = dot.write(&node);
        assert_eq!(
            s,
            vec![
            "digraph ast_tree {",
            "  0 [ label = plus ];",
            "  2 [ label = mul ];",
            "  1 [ label = mul ];",
            "  4 [ label = 4 ];",
            "  3 [ label = 5 ];",
            "  6 [ label = 2 ];",
            "  5 [ label = 3 ];",
            "  0 -> 1;",
            "  0 -> 2;",
            "  1 -> 3;",
            "  1 -> 4;",
            "  2 -> 5;",
            "  2 -> 6;",
            "}",
            ].join("\n")
        )
    }
}
