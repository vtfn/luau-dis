#[derive(Debug)]
pub struct Sink {
    buf: [u8; 255],
    len: usize,
}

impl Sink {
    pub const fn new() -> Self {
        Self {
            buf: [0; 255],
            len: 0,
        }
    }

    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_raw_parts(self.buf.as_ptr().cast(), self.len) }
    }

    pub const fn clear(&mut self) {
        self.len = 0;
    }
}

impl std::fmt::Write for Sink {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buf[self.len] = c as u8;
        self.len += 1;
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for i in 0..s.len() {
            self.buf[self.len + i] = s.as_bytes()[i];
        }

        self.len += s.len();
        Ok(())
    }
}
