mod graph;

use std::collections::HashMap;

fn main() {
    let mut h: HashMap<i32, Vec<i32>> = HashMap::new();

    h.insert(1, vec![2]);
    h.insert(2, vec![1,3]);
    h.insert(3, vec![1,2,3,5]);
    h.insert(4, vec![1]);
    h.insert(5, vec![4]);

    let g = graph::Graph::<i32>{
            neighbors: h,
            nodes: vec![1,2,3,4,5]
    };

    println!("{:?}", g.bfs(&1,&4))
}
