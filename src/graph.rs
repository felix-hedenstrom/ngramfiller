#![feature(hash_set_entry)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub trait Node: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::fmt::Debug {
    fn equivalent(&self, other: &Self) -> bool;
}
pub trait Graph<T: Node> {
    fn get_neighbors(&self, node: &T) -> Option<Vec<T>>;

    fn bfs(&self, start: T, end: T) -> std::option::Option<Vec<T>> {

        let mut visited: HashSet<T> = HashSet::new();
        

        let mut node: &T = visited.get_or_insert(start);

        let mut queue: VecDeque<&T> = VecDeque::new();
        let mut parent: HashMap<&T, &T> = HashMap::new();
        let mut last_node: std::option::Option<&T> = None;

        queue.push_back(node);

        while !queue.is_empty() {
            node = queue.pop_front().unwrap();

            if node.equivalent(&end) {
                last_node = Some(node);
                break;
            }

            for neighbor in self.get_neighbors(&node).unwrap_or(vec![])
            {
                if !visited.contains(&neighbor) {

                    let new_node: &T = visited.get_or_insert(neighbor);

                    parent.insert(new_node, node);

                    queue.push_back(new_node);
                }
            }
        }

        if last_node.is_none() {
            return None;
        }

        node = last_node.unwrap();

        let mut path: Vec<T> = Vec::new();

        path.push(node.clone());

        while parent.contains_key(&node) {
            node = parent
                .get(&node)
                .expect("did not recieve node when expected")
                .clone();
            path.push(node.clone());
        }
        path.reverse();
        return Some(path);
    }
}

impl Node for i32 {
    fn equivalent(&self, other: &i32) -> bool {
        return &self == &other;
    }
}

#[cfg(test)]
mod graph_tests {

    impl<T> Graph<T> for SimpleGraph<T>
    where
        T: Node,
    {
        fn get_neighbors(&self, node: &T) -> Option<Vec<T>> {
            match self.neighbors.get(node) {
                Some(x) => return Some(x.to_vec()),
                None => return None,
            }
        }
    }

    pub struct SimpleGraph<T>
    where
        T: Node,
    {
        pub neighbors: HashMap<T, Vec<T>>,
    }
    use super::*;

    #[test]
    fn test_simple_bfs() {
        let mut nb = HashMap::new();
        nb.insert(1, vec![2, 3]);
        nb.insert(3, vec![2]);
        nb.insert(2, vec![5]);
        nb.insert(5, vec![4]);

        let sg: SimpleGraph<i32> = SimpleGraph::<i32> { neighbors: nb };
        assert!(vec![1, 2, 5, 4] == sg.bfs(1, 4).unwrap());
    }

    #[test]
    fn test_super_simple_path() {
        let mut nb = HashMap::new();
        nb.insert(2, vec![1]);

        let sg: SimpleGraph<i32> = SimpleGraph::<i32> { neighbors: nb };

        assert!(vec![2, 1] == sg.bfs(2, 1).unwrap());
    }

    #[test]
    fn test_alpha_and_omega() {
        let mut nb = HashMap::new();
        nb.insert(1, vec![1, 2, 3, 4, 5]);
        nb.insert(2, vec![1]);

        let sg: SimpleGraph<i32> = SimpleGraph::<i32> { neighbors: nb };

        assert!(vec![1] == sg.bfs(1, 1).unwrap());
    }
}
