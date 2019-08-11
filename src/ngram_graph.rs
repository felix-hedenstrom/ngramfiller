use cpython::{PyDict, Python};

use crate::graph::Graph;
use crate::ngram::NGram;

enum Either<T1, T2> {
    SubGraph(T1),
    LastWords(T2),
}

use cpython::FromPyObject;
use std::collections::HashMap;

pub struct NGramGraph {
    n: usize,
    follows: Either<HashMap<String, NGramGraph>, HashMap<String, Vec<String>>>,
}

impl NGramGraph {
    pub fn new(py: Python, data: PyDict, n: usize) -> NGramGraph {
        if n == 2 {
            let mut follows = HashMap::<String, Vec<String>>::new();

            for (word, following_words) in data.items(py) {
                follows.insert(
                    FromPyObject::extract(py, &word).unwrap(),
                    keys(following_words.cast_into::<PyDict>(py).unwrap(), py),
                );
            }

            return NGramGraph {
                n: n,
                follows: Either::LastWords(follows),
            };
        } else {
            let mut follows = HashMap::<String, NGramGraph>::new();

            for (word, following_words) in data.items(py) {
                follows.insert(
                    FromPyObject::extract(py, &word).unwrap(),
                    NGramGraph::new(py, following_words.cast_into::<PyDict>(py).unwrap(), n - 1),
                );
            }

            return NGramGraph {
                n: n,
                follows: Either::SubGraph(follows),
            };
        }
    }

    pub fn size(&self) -> usize {
        return self.n;
    }
    /// Returns the complete set of NGrams contained within a NGramGraph
    /// Calls possible subgraphs if they exist
    fn get_neighbors_all(&self) -> Vec<NGram> {
        let mut all_ngrams = Vec::<NGram>::new();
        println!("Get them all!");
        match &self.follows {
            Either::SubGraph(g) => {
                let keys = g.keys();
                for key in keys {
                    all_ngrams = g.get(key).iter().fold(all_ngrams, |previous, subgraph| {
                        let mut temporary = previous.clone();
                        temporary.extend(
                            subgraph
                                .get_neighbors_all()
                                .iter()
                                .map(|ngram| ngram.push_front(key.clone())),
                        );
                        return temporary;
                    })
                }
            }
            Either::LastWords(w) => {
                let keys = w.keys();
                for key in keys {
                    all_ngrams = w.get(key).iter().fold(all_ngrams, |previous, last_words| {
                        let mut temporary = previous.clone();
                        temporary.extend(
                            last_words
                                .iter()
                                .map(|word| NGram::new(vec![key.clone(), word.clone()])),
                        );
                        return temporary;
                    })
                }
            }
        }
        return all_ngrams;
    }
    fn get_neighbors_helper(&self, tokens: &Vec<&String>) -> Option<Vec<NGram>> {

        if tokens.is_empty() {
            return Some(vec![]);
        }

        let endings = &self.follows;
        println!("These are the tokens: {:?}", tokens);
        match tokens.first() {
            None => return Some(self.get_neighbors_all()),
            Some(&key_token) => match endings {
                Either::SubGraph(sg) => {
                    println!("Found a subgraph for token {:?}", key_token);

                    if !sg.contains_key(key_token) {
                        println!("Did not contain the key :(. Does however contain keys:\n{:?}", sg.keys());
                        return None;
                    }

                    let subngrams: Option<Vec<NGram>> = sg
                        .get(key_token)
                        .unwrap()
                        .get_neighbors_helper(&tokens[1..].to_vec());

                    if subngrams.is_none() {
                        return None;
                    }

                    let mut answer = Vec::<NGram>::new();
                    for ngram in subngrams.unwrap() {
                        answer.push(ngram.push_front(key_token.clone()));
                    }
                    return Some(answer);
                }
                Either::LastWords(x) => {
                    if !x.contains_key(key_token) {
                        return None;
                    }

                    return Some(
                        x.get(key_token)
                            .unwrap()
                            .iter()
                            .map(|s| NGram::new(vec![key_token.clone(), s.clone()]))
                            .collect(),
                    );
                }
            },
        }
    }
}

impl Graph<NGram> for NGramGraph {
    fn get_neighbors(&self, node: &NGram) -> Option<Vec<NGram>> {
        return self.get_neighbors_helper(&node.tokens().iter().skip( 
                if node.size() < self.size(){
                    0
                }else{
                    1
                }
            ).collect());
    }
}

fn keys(dict: PyDict, py: Python) -> Vec<String> {
    return dict
        .items(py)
        .into_iter()
        .map(|(k, _v)| FromPyObject::extract(py, &k).unwrap())
        .rev()
        .collect();
}
