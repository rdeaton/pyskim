use pyo3::prelude::*;
use pyo3::types::PyIterator;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::io::{Read, Result};

struct PyIteratorReader<'a> {
    iter: PyIterator<'a>,
}

impl Read for PyIteratorReader<'_> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        // TODO(rdeaton): We should make sure we're copying the full item here, not just the first
        // n bytes
        let n = self.iter.next();
        if n.is_none() {
            return Ok(0);
        }
        let s = n.unwrap()?;
        let k = s.downcast_ref::<PyString>().unwrap().as_bytes();
        Ok(buf.write(k).unwrap() + buf.write(b"\n").unwrap())
    }
}

#[pymodule]
fn pyskim(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "quick_skim")]
    fn quick_skim(py: Python, it: PyObject) -> PyResult<(usize)> {
        // TODO(rdeaton): This currently crashes if given non-iterators due to
        // https://github.com/PyO3/pyo3/issues/494
        let iter = PyIterator::from_object(py, &it)?;
        let reader = PyIteratorReader { iter: iter };
        let f = BufReader::new(reader);
        for line in f.lines() {
            println!("{}", line.unwrap());
        }

        println!("Wut");
        // Ok(iter?.count())
        Ok(1)
    }
    m.add_wrapped(wrap_pyfunction!(quick_skim))?;

    Ok(())
}
