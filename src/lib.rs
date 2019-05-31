extern crate skim;

use pyo3::prelude::*;
use pyo3::types::PyIterator;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;
use std::io::BufReader;
use std::io::Write;
use std::io::{Read, Result};

use skim::{Skim, SkimOptionsBuilder};

struct PyIteratorReader<'a> {
    iter: PyIterator<'a>,
    current_item: Option<&'a [u8]>,
    buffer_dist: usize,
}

impl<'a> PyIteratorReader<'a> {
    fn new(iter: PyIterator<'a>) -> PyIteratorReader<'_> {
        Self {
            iter: iter,
            current_item: None,
            buffer_dist: 0,
        }
    }
}

impl Read for PyIteratorReader<'_> {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        // TODO(rdeaton): We should make sure we're copying the full item here, not just the first
        // n bytes
        if self.current_item.is_none() {
            let n = self.iter.next();
            if n.is_none() {
                return Ok(0);
            }
            let s = n.unwrap()?;
            let raw_buf = s.downcast_ref::<PyString>().unwrap().as_bytes();
            self.current_item = Some(raw_buf);
        }
        let pybuf = self.current_item.unwrap();
        dbg!(pybuf.len());
        dbg!(self.buffer_dist);
        dbg!(buf.len());
        if pybuf.len() - self.buffer_dist < buf.len() {
            let ret =
                Ok(buf.write(&pybuf[self.buffer_dist..]).unwrap() + buf.write(b"\n").unwrap());
            self.current_item = None;
            self.buffer_dist = 0;
            return ret;
        } else {
            let ret = buf.write(&pybuf[self.buffer_dist..]).unwrap();
            self.buffer_dist = self.buffer_dist + ret;
            return Ok(ret);
        }
    }
}

#[pymodule]
fn pyskim(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "quick_skim")]
    fn quick_skim(
        py: Python<'static>,
        it: PyObject,
        ansi: Option<bool>,
        bind: Option<Vec<&str>>,
        cmd: Option<&str>,
        cmd_prompt: Option<&str>,
        color: Option<&str>,
        delimiter: Option<&str>,
        exact: Option<bool>,
        expect: Option<&str>,
        header: Option<&str>,
        header_lines: Option<usize>,
        height: Option<&str>,
        inline_info: Option<bool>,
        interactive: Option<bool>,
        layout: Option<&str>,
        margin: Option<&str>,
        min_height: Option<&str>,
        multi: Option<bool>,
        no_height: Option<bool>,
        no_hscroll: Option<bool>,
        nth: Option<&str>,
        preview: Option<&str>,
        preview_window: Option<&str>,
        print0: Option<bool>,
        print_cmd: Option<bool>,
        print_query: Option<bool>,
        prompt: Option<&str>,
        query: Option<&str>,
        read0: Option<bool>,
        regex: Option<bool>,
        replstr: Option<&str>,
        reverse: Option<bool>,
        tabstop: Option<&str>,
        tac: Option<bool>,
        tiebreak: Option<&str>,
        with_nth: Option<&str>,
    ) -> PyResult<(PyObject)> {
        // TODO(rdeaton): This currently crashes if given non-iterators due to
        // https://github.com/PyO3/pyo3/issues/494
        let iter = PyIterator::from_object(py, &it)?;
        let reader = PyIteratorReader::new(iter);
        let f = BufReader::new(reader);

        //let iter2 = PyIterator::from_object(py, &it)?;
        //let reader2 = PyIteratorReader::new(iter2);
        //let f2 = BufReader::new(reader2);
        //for line in f2.lines() {
        //println!("debug");
        //}

        let mut ops = SkimOptionsBuilder::default();
        if ansi.is_some() {
            ops.ansi(ansi.unwrap());
        }
        if bind.is_some() {
            ops.bind(bind.unwrap());
        }
        if cmd.is_some() {
            ops.cmd(cmd);
        }
        if cmd_prompt.is_some() {
            ops.cmd_prompt(cmd_prompt);
        }
        if color.is_some() {
            ops.color(color);
        }
        if delimiter.is_some() {
            ops.delimiter(delimiter);
        }
        if exact.is_some() {
            ops.exact(exact.unwrap());
        }
        if header.is_some() {
            ops.header(header);
        }
        if header_lines.is_some() {
            ops.header_lines(header_lines.unwrap());
        }
        if height.is_some() {
            ops.height(height);
        }
        if inline_info.is_some() {
            ops.inline_info(inline_info.unwrap());
        }
        if interactive.is_some() {
            ops.interactive(interactive.unwrap());
        }
        if layout.is_some() {
            ops.layout(layout.unwrap());
        }
        if margin.is_some() {
            ops.margin(margin);
        }
        if min_height.is_some() {
            ops.min_height(min_height);
        }
        if multi.is_some() {
            ops.multi(multi.unwrap());
        }
        if no_height.is_some() {
            ops.no_height(no_height.unwrap());
        }
        if no_hscroll.is_some() {
            ops.no_hscroll(no_hscroll.unwrap());
        }
        if nth.is_some() {
            ops.nth(nth);
        }
        if preview.is_some() {
            ops.preview(preview);
        }
        if preview_window.is_some() {
            ops.preview_window(preview_window);
        }
        if print0.is_some() {
            ops.print0(print0.unwrap());
        }
        if print_cmd.is_some() {
            ops.print_cmd(print_cmd.unwrap());
        }
        if print_query.is_some() {
            ops.print_query(print_query.unwrap());
        }
        if prompt.is_some() {
            ops.prompt(prompt);
        }
        if query.is_some() {
            ops.query(query);
        }
        if read0.is_some() {
            ops.read0(read0.unwrap());
        }
        if regex.is_some() {
            ops.regex(regex.unwrap());
        }
        if replstr.is_some() {
            ops.replstr(replstr);
        }
        if reverse.is_some() {
            ops.reverse(reverse.unwrap());
        }
        if tabstop.is_some() {
            ops.tabstop(tabstop);
        }
        if tac.is_some() {
            ops.tac(tac.unwrap());
        }
        if with_nth.is_some() {
            ops.with_nth(with_nth);
        }
        if expect.is_some() {
            ops.expect(Some(expect.unwrap().to_string()));
        }
        if tiebreak.is_some() {
            ops.tiebreak(Some(tiebreak.unwrap().to_string()));
        }
        if min_height.is_some() {
            ops.min_height(min_height);
        }

        let options = ops.build().unwrap();

        let selected_items = Skim::run_with(&options, Some(Box::new(f)))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

        let mut ret = Vec::new();
        for item in selected_items.iter() {
            ret.push(item.get_output_text().to_string());
        }
        Ok(ret.to_object(py))
    }
    m.add_wrapped(wrap_pyfunction!(quick_skim))?;

    Ok(())
}
