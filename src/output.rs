use std::io::{sink, Result, Sink, Write};

use either::Either::{self, Left, Right};

#[derive(Clone, Copy, Debug)]
pub struct Output<O: Write> {
    buffer: Either<O, Sink>,
}

impl<O: Write> Output<O> {
    #[must_use]
    pub fn new(buffer: O) -> Self {
        Self { buffer: Left(buffer) }
    }

    #[must_use]
    pub fn new_sink() -> Self {
        Self { buffer: Right(sink()) }
    }

    #[must_use]
    pub fn is_sink(&self) -> bool {
        self.buffer.is_right()
    }

    /// Send terminal codes to clear the last line of text. Only makes sense
    /// when writing to stdout/err.
    ///
    /// # Errors
    ///
    /// Returns the number of bytes written or an error if there was a problem
    /// writing to the buffer.
    pub fn clear_last_line(&mut self) -> Result<usize> {
        self.write_all(b"\x1b[1A")?;
        self.write(b"\r\x1b[2K")
    }
}

impl<O: Write> Write for Output<O> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let writer = &mut self.buffer as &mut dyn Write;

        writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        let writer = &mut self.buffer as &mut dyn Write;

        writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use std::io::stdout;

    use super::*;

    #[test]
    fn creating_an_output() {
        let o = Output::new(Vec::<u8>::new());

        assert!(o.buffer.is_left());

        let o = Output::<Vec<u8>>::new_sink();

        assert!(o.buffer.is_right());
    }

    #[test]
    fn writing_output() {
        let mut o = Output::new(Vec::new());

        let r = o.write(b"some text");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 9);
        assert_eq!(o.buffer.left().unwrap(), b"some text");

        let mut o = Output::<Vec<_>>::new_sink();

        let r = o.write(b"some text");
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 9);
    }

    #[test]
    fn is_sink() {
        assert!(!Output::new(stdout()).is_sink());
        assert!(Output::<Vec<u8>>::new_sink().is_sink());
    }

    #[test]
    fn clear_last_line() {
        let mut o = Output::new(Vec::new());

        let r = o.clear_last_line();

        assert!(r.is_ok());
        assert_eq!(o.buffer.left().unwrap(), b"\x1b[1A\r\x1b[2K");
    }
}
