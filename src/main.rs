use std::fmt::{Display, Write};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct NodeIndex(usize);

impl Display for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for NodeIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Node<L: Display> {
    edges: Vec<Edge>,
    label: L,
}

#[derive(Debug)]
pub struct Graph<L: Display> {
    nodes: Vec<Node<L>>,
}

#[derive(Debug)]
pub struct Edge {
    from: NodeIndex,
    to: NodeIndex,
}

impl<L: Display> Graph<L> {
    /// Generate the dot file to render this graph. `write!` operations
    /// return a fmt::Result, and in the event of a failure this function
    /// returns `None`.
    fn gen_dot(&self) -> Option<String> {
        let mut s = String::with_capacity(1024);
        write!(&mut s, "digraph {{\n").ok()?;

        // We must breadth-first search the tree, printing each edge and then
        // printing out a same-rank qualifier for each node found with our
        // previous step of bfs.

        let mut visited = vec![false; self.nodes.len()];
        let mut frontier = vec![];
        let mut next_frontier = vec![];
        let mut count = 0usize;

        // Mark our first node as visited and put it in the frontier.
        visited[0] = true;
        frontier.push(NodeIndex(0));
        count += 1;

        loop {
            // All the nodes in our frontier were found in the same step of
            // depth first search, and thus have the same rank.
            if count == self.nodes.len() {
                // Our loop termination condition. We set the rank here to max
                // to indicate that this rank should be displayed last.
                write!(&mut s, "  {{ rank=\"max\"; ").ok()?;
                for &idx in &frontier {
                    write!(&mut s, "\"{}\" ", &self.nodes[*idx].label).ok()?;
                }
                write!(&mut s, "}}\n").ok()?;
                // Break out of our loop.
                break;
            } else {
                write!(&mut s, "  {{ rank=\"same\"; ").ok()?;
                for &idx in &frontier {
                    write!(&mut s, "\"{}\" ", &self.nodes[*idx].label).ok()?;
                }
                write!(&mut s, "}}\n").ok()?;
            }

            for &idx in &frontier {
                let node = &self.nodes[*idx];
                for edge in &node.edges {
                    // Print out the edge.
                    let from_label = &self.nodes[*edge.from].label;
                    let to_label = &self.nodes[*edge.to].label;
                    write!(&mut s, "  \"{}\" -> \"{}\"\n", from_label, to_label).ok()?;
                    let to = edge.to;
                    // If we haven't visited the next node, mark it as
                    // visited and add it to our next frontier.
                    if !visited[*to] {
                        visited[*to] = true;
                        next_frontier.push(to);
                    }
                }
            }

            // Increase the count of nodes we have processed so far.
            count += next_frontier.len();

            // Forget the previous frontier, as we no longer need it.
            frontier.clear();

            // Swap the (now-empty) frontier with the next frontier. This is
            // useful to avoid needless allocations or copies.
            std::mem::swap(&mut next_frontier, &mut frontier);
        }

        write!(&mut s, "}}").ok()?;
        Some(s)
    }
}

/// Helper macro to quickly construct Node literals.
macro_rules! N {
    // N!("label"; 7)
    ($label:expr; $from:expr) => (Node {
        edges: vec![],
        label: format!("{}", $label),
    });
    // N!(7)
    ($from:expr) => (Node {
        edges: vec![],
        label: format!("{}", $from),
    });
    // N!(7 => 8, 9, 10)
    ($from:expr => $( $to:expr ),*) => (Node {
        edges: vec![$( Edge { from: NodeIndex($from), to: NodeIndex($to) }, )*],
        label: format!("{}", $from),
    });
    // N!("label"; 7 => 8, 9, 10)
    ($label:expr; $from:expr => $( $to:expr ),*) => (Node {
        edges: vec![$( Edge { from: NodeIndex($from), to: NodeIndex($to) }, )*],
        label: format!("{}", $label),
    });
}

/// Helper macro to quickly construct Graph literals.
macro_rules! G {
    [$( $node:expr ),* ] => (Graph {
        nodes: vec![$( $node, )*],
    });
    // Optional trailing comma.
    [$( $node:expr ),*, ] => (Graph {
        nodes: vec![$( $node, )*],
    });
}

fn main() {
    let g = G![
        N!("Root"; 0 => 1, 2, 3, 6),
        N!(1 => 4, 5),
        N!(2 => 4, 5),
        N!(3 => 4, 5),
        N!("Leaf 1"; 4),
        N!("Leaf 2"; 5),
        N!("Leaf 3"; 6),
    ];
    println!("{}", g.gen_dot().unwrap());
}
