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
        let endings = &self.follows;
        match tokens.first() {
            None => return Some(self.get_neighbors_all()),
            Some(&key_token) => match endings {
                Either::SubGraph(sg) => {
                    if !sg.contains_key(key_token) {
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
        return self.get_neighbors_helper(
            &node
                .tokens()
                .iter()
                .skip(if node.size() < self.size() { 0 } else { 1 })
                .collect(),
        );
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
#[cfg(test)]
mod tests {


    fn compare_unordered<T>(a: &Vec<T>, b: &Vec<T>)
    where 
        T: Eq + std::fmt::Debug + std::cmp::Ord + Clone
    {
        let mut locala: Vec<T> = a.to_vec();
        let mut localb: Vec<T> = b.to_vec();
        assert_eq!(locala.sort(), localb.sort());

    }

    macro_rules! hashmap {
            ($( $key: expr => $val: expr ),*) => {{
                         let mut map = ::std::collections::HashMap::new();
                                  $( map.insert($key, $val); )*
                                               map
                                                   }}
    }
    macro_rules! s {
        ($str:expr) => {
            String::from($str);
        };
    }
    #[derive(Eq, PartialEq)]
    enum NGramSize {
        Two,
        Three
    }

    fn generate_testdata(ngs: NGramSize) -> NGramGraph {
        let is_lowest_ngrams = NGramGraph {
            n: 2,
            follows: Either::LastWords(hashmap![
                s!("a") => vec![s!("test"), s!("trial")],
                s!("an") => vec![s!("assessment"), s!("approval")]
            ]),
        };
        if ngs == NGramSize::Two {
            return is_lowest_ngrams;
        }
        
        return NGramGraph{
            n: 3,
            follows: 
                Either::SubGraph(
                    hashmap![
                s!("is") => is_lowest_ngrams])
        }
    }

    use super::*;
    #[test]
    fn test_get_neighbors_non_existing() {
        let testdata = generate_testdata(NGramSize::Three);
        assert_eq!(testdata.get_neighbors(&NGram::new(vec![s!("the")])), None);
    }

    #[test]
    fn test_get_neighbors_2gram() {
        let testdata = generate_testdata(NGramSize::Two);
        assert_eq!(
            testdata.get_neighbors(&NGram::new(vec![s!("a")])),
            Some(vec![
                NGram::new(vec![s!("a"), s!("test")]),
                NGram::new(vec![s!("a"), s!("trial")])
            ])
        );
    }
    #[test]
    fn test_get_neighbors_3gram() {
       
        let testdata = generate_testdata(NGramSize::Three);


        compare_unordered(
            &testdata.get_neighbors(&NGram::new(vec![s!("is")])).unwrap(),
            &vec![
                NGram::new(vec![s!("is"), s!("a"), s!("test")]),
                NGram::new(vec![s!("is"), s!("a"), s!("trial")]),
                NGram::new(vec![s!("is"), s!("an"), s!("assessment")]),
                NGram::new(vec![s!("is"), s!("an"), s!("approval")]),
            ]
        );
        compare_unordered(
            &testdata.get_neighbors(&NGram::new(vec![s!("is"), s!("an")])).unwrap(),
            &vec![
                NGram::new(vec![s!("is"), s!("an"), s!("assessment")]),
                NGram::new(vec![s!("is"), s!("an"), s!("approval")]),
            ]
        );
    }
}
