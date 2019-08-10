use cpython::{Python, PyDict};

use crate::graph::Graph;
use crate::ngram::NGram;

enum Either<T1, T2> {
    SubGraph(T1),
    LastWords(T2),
}


use std::collections::HashMap;
use cpython::FromPyObject;

pub struct NGramGraph {
    n: u32,
    follows: 
        Either<
            HashMap<String, NGramGraph>,
            HashMap<String, Vec<String>>> 
    
}

impl NGramGraph {
    pub fn new(py: Python, data: PyDict, n: u32) -> NGramGraph{
        
        if n == 2{
            let mut follows = HashMap::<String, Vec<String>>::new();
            
            for (word, following_words) in data.items(py){
                follows.insert(FromPyObject::extract(py, &word).unwrap(), keys(following_words.cast_into::<PyDict>(py).unwrap(), py));

            }

            return NGramGraph{
                n: n,
                follows: Either::LastWords(follows)
            };
        }else{

            let mut follows = HashMap::<String, NGramGraph>::new();
            
            for (word, following_words) in data.items(py){
                follows.insert(FromPyObject::extract(py, &word).unwrap(), NGramGraph::new(py, following_words.cast_into::<PyDict>(py).unwrap(), n - 1));
            }

            return NGramGraph{
                n: n,
                follows: Either::SubGraph(follows)
            };

        }

    }

    pub fn size(&self) -> u32{
        return self.n;
    }

    fn get_neighbors_helper(&self, tokens: Vec<&String>) -> Option<Vec<NGram>>{
        let endings = &self.follows;
        
        match endings {
            Either::SubGraph(x) => println!("{:?}", x.keys()),
            Either::LastWords(x) => println!("{:?}", x.keys())
        }


        return Some(vec![NGram::new(vec![String::from("this"), String::from("is"), String::from("a"), String::from("test")])]); 
    }
}


impl Graph<NGram> for NGramGraph{
    fn get_neighbors(&self, node: &NGram) -> Option<Vec<NGram>>{
        return self.get_neighbors_helper(node.tokens().iter().skip(1).collect()); 
    }

}


fn keys(dict: PyDict, py: Python) -> Vec<String>{
   return dict.items(py).into_iter().map(|(k, _v)| FromPyObject::extract(py, &k).unwrap()).rev().collect();
}
