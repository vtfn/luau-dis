use crate::{Error, Reader, Result};

mod instruction;
pub use instruction::*;

#[derive(Debug)]
pub struct Decoder<'a> {
    reader: Reader<'a>,
}

impl<'a> Decoder<'a> {
    pub const fn new(buf: &'a [u8]) -> Self {
        let reader = Reader::new(buf);

        Self { reader }
    }

    pub fn decode(&mut self) -> Result<Instruction> {
        #[allow(invalid_value)]
        let mut inst = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        match self.decode_out(&mut inst) {
            Ok(_) => Ok(inst),
            Err(e) => Err(e),
        }
    }

    pub fn decode_out(&mut self, out: &mut Instruction) -> Result<()> {
        let word = self.reader.read_u32()?;

        if let None = OPCODES.get((word & 0xFF) as usize) {
            return Err(Error::Malformed);
        }

        let aux = if HAX_AUX[(word & 0xFF) as usize] {
            self.reader.read_u32()?
        } else {
            0
        };

        out.word = word;
        out.aux = aux;
        Ok(())
    }
}
