use crate::error::NolineError;
/// IO wrapper for stdin and stdout
use embedded_io::Error;

pub struct IO<RW>
where
    RW: embedded_io::Read + embedded_io::Write,
{
    rw: RW,
}

impl<RW> IO<RW>
where
    RW: embedded_io::Read + embedded_io::Write,
{
    pub fn new(rw: RW) -> Self {
        Self { rw }
    }

    pub fn inner(&mut self) -> &mut RW {
        &mut self.rw
    }
}

impl<RW> embedded_io::ErrorType for IO<RW>
where
    RW: embedded_io::Read + embedded_io::Write,
{
    type Error = NolineError;
}

impl<RW> embedded_io::Read for IO<RW>
where
    RW: embedded_io::Read + embedded_io::Write,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, NolineError> {
        self.rw
            .read(buf)
            .map_err(|e| NolineError::ReadError(e.kind().into()))
    }
}

impl<RW> embedded_io::Write for IO<RW>
where
    RW: embedded_io::Read + embedded_io::Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, NolineError> {
        self.rw
            .write_all(buf)
            .map_err(|e| NolineError::WriteError(e.kind().into()))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), NolineError> {
        self.rw
            .flush()
            .map_err(|e| NolineError::WriteError(e.kind().into()))
    }
}

impl<RW> core::fmt::Write for IO<RW>
where
    RW: embedded_io::Read + embedded_io::Write,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.rw.write(s.as_bytes()).or(Err(core::fmt::Error))?;
        Ok(())
    }
}

#[cfg(any(test, doctest, feature = "std"))]
pub mod std_sync {
    use std::io::{Read, Stdin, Stdout, Write};

    pub struct StdIOWrapper {
        stdin: Stdin,
        stdout: Stdout,
    }
    impl StdIOWrapper {
        pub fn new() -> Self {
            Self {
                stdin: std::io::stdin(),
                stdout: std::io::stdout(),
            }
        }
    }

    impl Default for StdIOWrapper {
        fn default() -> Self {
            Self::new()
        }
    }

    impl embedded_io::ErrorType for StdIOWrapper {
        type Error = embedded_io::ErrorKind;
    }

    impl embedded_io::Read for StdIOWrapper {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let _ = self
                .stdin
                .read_exact(&mut buf[0..1])
                .map_err(|e| e.kind())?;
            Ok(1)
        }
    }

    impl embedded_io::Write for StdIOWrapper {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            Ok(self.stdout.write(buf).map_err(|e| e.kind())?)
        }
        fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(self.stdout.flush().map_err(|e| e.kind())?)
        }
    }
}
