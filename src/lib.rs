mod graph;

mod ngram;
mod ngram_graph;
use crate::ngram_graph::NGramGraph;
use crate::graph::Graph;
use crate::ngram::NGram;

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult, PyDict, PyList, PyInt};

use cpython::FromPyObject;
use cpython::ToPyObject;
use cpython::PythonObject;
fn depth(_py: Python, val: PyDict) -> PyResult<u32> {
    return Ok(depth_internal(_py, val));
}

fn bfs(py: Python, val: PyDict, n: PyInt) -> PyResult<PyList> {

    let ngg: NGramGraph = NGramGraph::new(py, val, FromPyObject::extract(py, &n.as_object()).unwrap());
     
    let answer = ngg.get_neighbors(&NGram::new(vec![String::from("this")])).unwrap();

    let intermediate : Vec<Vec<String>> = answer.iter().map(|ngram| ngram.as_vec()).collect(); 
    return Ok(intermediate.to_py_object(py));

    //return Ok(ngg.size());

}


/// Temporary code to test how to interface with PyDict 
fn depth_internal(_py: Python, val: PyDict) -> u32 {
    if val.len(_py) == 0{
        return 1;
    }
    for (_k,v) in val.items(_py){
        let potential_dict: PyResult<PyDict> = v.extract::<PyDict>(_py);
        match potential_dict {
            Ok(v) => return 1 + depth_internal(_py, v),
            Err(_e) => return 1,
        }
    }
    return 1;

}


py_module_initializer!(libngramconnector, initlibngramconnector, PyInit_libngramconnector, |py, m | {
    
    r#try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    r#try!(m.add(py, "depth", py_fn!(py, depth(val: PyDict))));
    r#try!(m.add(py, "bfs", py_fn!(py, bfs(val: PyDict, n: PyInt))));
    Ok(())
});
