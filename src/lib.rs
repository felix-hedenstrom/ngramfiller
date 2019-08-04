mod graph;
#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult, PyDict, PyList, PyInt, PyErr};

fn test(_py: Python, val: PyDict) -> PyResult<u32> {
    return Ok(1);
}


fn depth(_py: Python, val: PyDict) -> PyResult<u32> {
    return Ok(depth_internal(_py, val));
}

fn bfs(_py: Python, val: PyDict, n: PyInt) -> PyResult<PyInt> {
    return Ok(n);
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
    r#try!(m.add(py, "test", py_fn!(py, test(val: PyDict))));
    r#try!(m.add(py, "depth", py_fn!(py, depth(val: PyDict))));
    r#try!(m.add(py, "bfs", py_fn!(py, bfs(val: PyDict, n: PyInt))));
    Ok(())
});
