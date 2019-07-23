use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;


pub struct Graph<T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug> {
    pub neighbors: HashMap<T, Vec<T>>,
    pub nodes: Vec<T>
}

impl <T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::fmt::Debug> Graph<T>{

    fn get_neighbors(&self, node: &T) -> std::option::Option<&Vec<T>>{
        return self.neighbors.get(node);
    }


    pub fn bfs(&self, start: &T, end: &T) -> std::option::Option<Vec<T>> {
        let mut queue: VecDeque<&T> = VecDeque::new();
        let mut node: &T = start; 
        let mut parent: HashMap<&T,&T> = HashMap::new();
        let mut visited: HashSet<&T> = HashSet::new();


        queue.push_back(node);
        visited.insert(node);

        while !queue.is_empty() {
            node = queue.pop_front().expect("node value was not found");   
            
            if node == end {
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

        node = end;
        
        if !parent.contains_key(&end){
            return None;
        }
       
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
