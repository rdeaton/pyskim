extern crate skim;

use pyo3::prelude::*;
use pyo3::types::PyIterator;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::io::{Read, Result};

use skim::{Skim, SkimOptionsBuilder};
use std::io::Cursor;

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
    fn quick_skim(py: Python, it: PyObject) -> PyResult<(PyObject)> {
        // TODO(rdeaton): This currently crashes if given non-iterators due to
        // https://github.com/PyO3/pyo3/issues/494
        let iter = PyIterator::from_object(py, &it)?;
        let reader = PyIteratorReader { iter: iter };
        let f = BufReader::new(reader);
        for line in f.lines() {
            println!("{}", line.unwrap());
        }

        let mut ret = Vec::new();

        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(true)
            .build()
            .unwrap();

        let input = "aaaaa\nbbbb\nccc".to_string();

        let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

        for item in selected_items.iter() {
            ret.push(item.get_output_text().to_string());
        }

        Ok(ret.to_object(py))
    }
    m.add_wrapped(wrap_pyfunction!(quick_skim))?;

    Ok(())
}
