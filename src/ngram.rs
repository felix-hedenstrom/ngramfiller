use crate::graph::Node;

use cpython::FromPyObject;

use cpython::{PyList, PyObject, Python};

use cpython::PythonObject;

#[derive(Eq, PartialEq, Clone, Hash, Debug, PartialOrd, Ord)]
pub struct NGram {
    // https://stackoverflow.com/questions/25754863/how-to-create-a-rust-struct-with-string-members
    tokens: Vec<String>,
}

impl NGram {
    pub fn new(tokens_: Vec<String>) -> NGram {
        NGram { tokens: tokens_ }
    }
    pub fn push_front(&self, new_element: String) -> Self {
        let mut new_tokens = self.tokens.clone();
        new_tokens.insert(0, new_element);
        return NGram::new(new_tokens);
    }
    pub fn as_vec(&self) -> Vec<String> {
        return self.tokens.clone();
    }

    pub fn size(&self) -> usize {
        return self.len();
    }

    pub fn len(&self) -> usize {
        return self.tokens.len();
    }

    pub fn tokens(&self) -> &Vec<String> {
        return &self.tokens;
    }

    pub fn from_pylist(py: Python, list: PyList) -> NGram {
        let as_vec: Vec<PyObject> = FromPyObject::extract(py, &list.as_object()).unwrap();
        let mut answer: Vec<String> = Vec::new();

        for word in as_vec {
            answer.push(word.to_string());
        }

        return NGram::new(answer);
    }
}

impl std::ops::Index<usize> for NGram {
    type Output = str;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.tokens[i];
    }
}

impl Node for NGram {
    // Two ngrams are equivalent if they share the same last values.
    // If 'other' has length 2 and 'self' has length 3, the last two elements
    // of 'self' should be the same as in 'other'
    // TODO with more knowledge of rust this could probably be made faster
    fn equivalent(&self, other: &NGram) -> bool {
        if self.len() < other.len() {
            return false;
        }

        let diff: usize = self.len() - other.len();
        let mut i = diff;

        while i < self.len() {
            if self[i] != other[i - diff] {
                return false;
            }
            i = i + 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equivalent() {
        assert!(NGram::new(vec![
            String::from("This"),
            String::from("is"),
            String::from("a"),
            String::from("test")
        ])
        .equivalent(&NGram::new(vec![String::from("test")])));
    }

    #[test]
    fn test_different() {
        assert!(!NGram::new(vec![String::from("test")])
            .equivalent(&NGram::new(vec![String::from("to"), String::from("test")])));
    }
    #[test]
    fn test_push_front() {
        let ng: NGram = NGram::new(vec![String::from("test")]);
        assert_eq!(
            ng.push_front(String::from("a")),
            NGram::new(vec![String::from("a"), String::from("test")])
        );
    }

    #[test]
    fn test_tokens() {
        let ng: NGram = NGram::new(vec![
            String::from("this"),
            String::from("is"),
            String::from("a"),
            String::from("test"),
        ]);
        assert_eq!(
            ng.tokens(),
            &vec![
                String::from("this"),
                String::from("is"),
                String::from("a"),
                String::from("test")
            ]
        );
    }
}
