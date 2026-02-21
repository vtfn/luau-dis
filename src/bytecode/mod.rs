use crate::{Error, Reader, Result};

mod constant;
pub use constant::Constant;

mod proto;
pub use proto::Proto;

#[derive(Debug)]
pub struct Bytecode<'a> {
    strings: Vec<&'a str>,
    protos: Vec<Proto<'a>>,
    main: usize,
}

impl<'a> Bytecode<'a> {
    pub fn parse(buf: &'a [u8]) -> Result<Self> {
        let mut reader = Reader::new(buf);

        if reader.read_u8()? != 6 || reader.read_u8()? != 3 {
            unimplemented!()
        }

        let strings = {
            let len = reader.read_leb()?;

            reader.read_list(len, |reader| {
                let str_len = reader.read_leb()?;

                if reader.rem() < str_len {
                    return Err(Error::Exhausted);
                }

                let ptr = reader.as_ptr();

                unsafe {
                    reader.advance_unchecked(str_len);
                    Ok(std::str::from_raw_parts(ptr, str_len))
                }
            })?
        };

        reader.try_advance(1)?;

        let protos = {
            let len = reader.read_leb()?;

            reader.read_list(len, Proto::parse)
        }?;

        let main = reader.read_leb()?;

        if reader.rem() != 0 {
            return Err(Error::Malformed);
        }

        Ok(Self {
            strings,
            protos,
            main,
        })
    }

    pub fn strings(&self) -> &[&str] {
        self.strings.as_slice()
    }

    pub fn protos(&self) -> &[Proto<'_>] {
        self.protos.as_slice()
    }

    pub fn main(&self) -> usize {
        self.main
    }
}
