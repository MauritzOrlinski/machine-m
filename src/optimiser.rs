// code optimiser

use std::collections::HashMap;

use crate::ast::{Ops, Program};

struct ControlFlowGraph {
    ops: HashMap<usize, Ops>,
    edges: HashMap<usize, usize>,
}

impl From<Program> for ControlFlowGraph {
    fn from(value: Program) -> Self {
        let mut ops = HashMap::new();
        let mut edges = HashMap::new();
        for (line, op) in value.iter().clone().enumerate() {
            ops.insert(line, op.clone());
            match op {
                Ops::Jump(Some(i)) => {
                    edges.insert(line, *i);
                }
                Ops::Positive(i) => {
                    edges.insert(line, *i);
                }
                Ops::Negative(i) => {
                    edges.insert(line, *i);
                }
                Ops::Zero(i) => {
                    edges.insert(line, *i);
                }
                _ => (),
            }
        }
        Self { ops, edges }
    }
}

impl ControlFlowGraph {
    /// removes any nops from the program graph
    fn remove_nops(&mut self) {}
    /// finds ops that are colapsable
    fn colapse_ops(&mut self) {}
    /// marks reachable parts of the code
    fn reachablility_analysis(&mut self) {}
}
