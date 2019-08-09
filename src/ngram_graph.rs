use cpython::{Python, PyResult, PyDict, PyList, PyInt, PyErr, PyString};


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
}


//impl <NGram> Graph<NGram> for NGramGraph{
//    pub fn get_neighbors(&self, node: &NGram) -> Option<Vec<NGram>>{
//    
//    }
//}


fn keys(dict: PyDict, py: Python) -> Vec<String>{
   return dict.items(py).into_iter().map(|(k, v)| FromPyObject::extract(py, &k).unwrap()).rev().collect();
}
