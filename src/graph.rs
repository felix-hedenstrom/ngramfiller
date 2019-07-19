use std::collections::HashMap;
use std::collections::LinkedList;


struct Graph<T> {
    neighbors: HashMap<T, LinkedList<T>>
}




pub fn bfs() -> i32 {
    let mut h = HashMap::new();

    h.insert(1, LinkedList::new());

    let g = Graph{neighbors: h}; 

    return g.neighbors.get(&1).size();

}

