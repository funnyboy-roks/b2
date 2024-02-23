use progress_bar as bar;
use std::io::{Read, Write};

pub struct ReaderProgress<R> {
    inner: R,
    curr: usize,
}

impl<R> ReaderProgress<R> {
    pub fn new(r: R, len: usize) -> Self {
        bar::init_progress_bar_with_eta(len);
        bar::set_progress_bar_action("Uploading", bar::Color::Green, bar::Style::Bold);
        Self { inner: r, curr: 0 }
    }
}

impl<R> Read for ReaderProgress<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.inner.read(buf) {
            Ok(n) => {
                self.curr += n;
                bar::set_progress_bar_progress(self.curr);
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriterProgress<W> {
    inner: W,
    curr: usize,
}

impl<W> WriterProgress<W> {
    pub fn new(w: W, len: usize) -> Self {
        bar::init_progress_bar_with_eta(len);
        bar::set_progress_bar_action("Downloading", bar::Color::Green, bar::Style::Bold);
        Self { inner: w, curr: 0 }
    }
}

impl<W> Write for WriterProgress<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.inner.write(buf) {
            Ok(n) => {
                self.curr += n;
                bar::set_progress_bar_progress(self.curr);
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
