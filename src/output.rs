use std::io::{sink, Result, Sink, Write};

use either::Either::{self, Left, Right};

#[derive(Clone, Copy, Debug)]
pub struct Output<O, S> {
    buffer: Either<O, S>,
}

impl<O> Output<O, Sink> {
    fn new(buffer: O) -> Self {
        Self { buffer: Left(buffer) }
    }

    fn new_sink() -> Self {
        Self { buffer: Right(sink()) }
    }

    #[must_use]
    pub fn is_sink(&self) -> bool {
        self.buffer.is_right()
    }
}

impl<O: Write, S: Write> Write for Output<O, S> {
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
        let o = Output::new(Vec::<Vec<u8>>::new());

        assert!(o.buffer.is_left());

        let o = Output::<Vec<u8>, Sink>::new_sink();

        assert!(o.buffer.is_right());
    }

    #[test]
    fn writing_output() {
        let mut o = Output::new(Vec::new());

        let r = o.write("some text".as_bytes());
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 9);
        assert_eq!(o.buffer.left().unwrap(), "some text".as_bytes());

        let mut o = Output::<Vec<_>, _>::new_sink();

        let r = o.write("some text".as_bytes());
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 9);
    }

    #[test]
    fn is_sink() {
        assert!(!Output::new(stdout()).is_sink());
        assert!(Output::<Vec<u8>, _>::new_sink().is_sink());
    }
}
