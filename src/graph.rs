use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;


pub struct Graph<T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug + Node> {
    pub neighbors: HashMap<T, Vec<T>>,
    pub nodes: Vec<T>
}

pub trait Node {
    fn equivalent(&self, other: &Self) -> bool;
}

impl Node for i32 {
    fn equivalent(&self, other: &i32) -> bool{
        return &self == &other
    }
}


impl <T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug + Node> Graph<T>{

    fn get_neighbors(&self, node: &T) -> std::option::Option<&Vec<T>>{
        return self.neighbors.get(node);
    }


    pub fn bfs(&self, start: &T, end: &T) -> std::option::Option<Vec<T>> {
        let mut queue: VecDeque<&T> = VecDeque::new();
        let mut node: &T = start; 
        let mut parent: HashMap<&T,&T> = HashMap::new();
        let mut visited: HashSet<&T> = HashSet::new();
        let mut last_node: std::option::Option<&T> = None;

        queue.push_back(node);
        visited.insert(node);

        while !queue.is_empty() {
            node = queue.pop_front().expect("node value was not found");   
            
            if node.equivalent(end) {
                last_node = Some(end);
                break;
            }

            for neighbor in self.get_neighbors(node).expect("node did not have neighbors"){
                if !visited.contains(neighbor){
                    visited.insert(neighbor);
                    parent.insert(neighbor, node);
                
                    queue.push_back(neighbor);
                }
            }

        }

        if last_node.is_none() || !parent.contains_key(last_node.unwrap()){
            return None;
        }

        node = last_node.unwrap();
        
       
        let mut path: Vec<T> = Vec::new();
        path.push(
            node.clone()
        );
        
        while parent.contains_key(node){
            node = parent.get(node).expect("did not recieve node when expected");
            path.push(
                node.clone()
            );
        }

        return Some(path);
    
    }
}
