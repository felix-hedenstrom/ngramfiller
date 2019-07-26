use crate::graph::Node;


struct NGram {
    tokens: Vec<String>
}

impl NGram {
    fn new(tokens_: Vec<String>) -> NGram{
        NGram{
            tokens: tokens_,
        }
    }

    fn len(&self) -> usize{
        return self.tokens.len()
    }
}

impl std::ops::Index<usize> for NGram{
    type Output = str;
    fn index(&self, i: usize) -> &Self::Output{
        return &self.tokens[i];
    }
}

impl Node for NGram {
    // Two ngrams are equivalent if they share the same last values.
    // If 'other' has length 2 and 'self' has length 3, the last two elements
    // of 'self' should be the same as in 'other'
    // TODO with more knowledge of rust this could probably be made faster 
    fn equivalent(&self, other: &NGram) -> bool{
        if self.len() < other.len(){
            return false;
        }

        let diff: usize = self.len() - other.len();
        let mut i = diff;
        
        println!("{:?}",diff);

        while i < self.len(){
            if self[i] != other[i - diff]{
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
        assert!(
            NGram::new(vec![String::from("This"), String::from("is"), String::from("a"), String::from("test")])
            .equivalent(
                &NGram::new(vec![String::from("test")]))
            
        );
    }
    
    #[test]
    fn test_different() {
        assert!(
            !NGram::new(vec![String::from("test")])
            .equivalent(
                &NGram::new(vec![String::from("to"), String::from("test")])    
            )

        );
    }
}