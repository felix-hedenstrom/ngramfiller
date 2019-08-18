mod graph;

mod ngram;
mod ngram_graph;

use crate::graph::Graph;
use crate::ngram::NGram;
use crate::ngram_graph::NGramGraph;

#[macro_use]
extern crate cpython;

use cpython::{PyDict, PyInt, PyList, PyResult, Python};

use cpython::FromPyObject;

use cpython::PythonObject;
use cpython::ToPyObject;
fn depth(_py: Python, val: PyDict) -> PyResult<u32> {
    return Ok(depth_internal(_py, val));
}

fn bfs(py: Python, val: PyDict, n: PyInt, start: PyList, end: PyList) -> PyResult<PyList> {
    dbg!("started translation");
    dbg!(std::time::SystemTime::now());
    let ngg: NGramGraph = NGramGraph::new(
        py,
        val,
        FromPyObject::extract(py, &n.as_object())
            .expect("failed to extract an integer from the n argument"),
    );
    dbg!("translated datastructure");
    dbg!(std::time::SystemTime::now());
    let start: NGram = NGram::from_pylist(py, start);
    let end: NGram = NGram::from_pylist(py, end);
    dbg!("started search {:?}"); 
    dbg!(std::time::SystemTime::now());
    let path: Option<Vec<NGram>> = ngg.bfs(start, end);
    dbg!("finished search"); 
    dbg!(std::time::SystemTime::now());
    let mut answer: Vec<Vec<String>> = vec![];

    return match path {
        None => Ok(PyList::new(py, &vec![])),
        Some(p) => {
            for ngram in p {
                answer.push(ngram.as_vec());
            }
            Ok(answer.to_py_object(py))
        }
    };
}

/// Temporary code to test how to interface with PyDict
fn depth_internal(_py: Python, val: PyDict) -> u32 {
    if val.len(_py) == 0 {
        return 1;
    }
    for (_k, v) in val.items(_py) {
        let potential_dict: PyResult<PyDict> = v.extract::<PyDict>(_py);
        match potential_dict {
            Ok(v) => return 1 + depth_internal(_py, v),
            Err(_e) => return 1,
        }
    }
    return 1;
}

py_module_initializer!(
    libngramconnector,
    initlibngramconnector,
    PyInit_libngramconnector,
    |py, m| {
        r#try!(m.add(py, "__doc__", "This module is implemented in Rust"));
        r#try!(m.add(py, "depth", py_fn!(py, depth(val: PyDict))));
        r#try!(m.add(
            py,
            "bfs",
            py_fn!(py, bfs(val: PyDict, n: PyInt, start: PyList, end: PyList))
        ));
        Ok(())
    }
);
