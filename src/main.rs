mod graph;

use crate::graph::prelude::*;

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
