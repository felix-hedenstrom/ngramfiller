mod graph;

#[macro_use]
extern crate cpython;


use cpython::{Python, PyResult, PyDict};

fn test(_py: Python, val: PyDict) -> PyResult<u32> {
    return Ok(1);
}


fn depth(_py: Python, val: PyDict) -> PyResult<u32> {
    return Ok(depth_interal(_py, val));
}


fn depth_interal(_py: Python, val: PyDict) -> u32 {
    if val.len(_py) == 0{
        return 0
    }
    for i in val.items(_py){
        return 0;
    }
    return 0;

}


py_module_initializer!(libngramconnector, initlibngramconnector, PyInit_libngramconnector, |py, m | {
    
    r#try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    r#try!(m.add(py, "test", py_fn!(py, test(val: PyDict))));
    r#try!(m.add(py, "depth", py_fn!(py, depth(val: PyDict))));
    Ok(())
});
