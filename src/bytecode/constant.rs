use crate::{Error, Reader, Result};

#[derive(Debug)]
pub enum Constant {
    Nil,
    Bool(bool),
    Number(f64),
    String(usize),
    Import(usize),
    Table(Vec<usize>),
    Closure(usize),
    Vector(f32, f32, f32, f32),
}

impl Constant {
    pub(crate) fn parse(reader: &mut Reader<'_>) -> Result<Self> {
        match reader.read_u8()? {
            0 => Ok(Constant::Nil),
            1 => Ok(Constant::Bool(reader.read_u8()? != 0)),
            2 => Ok(Constant::Number(reader.read_f64()?)),
            3 => Ok(Constant::String(reader.read_leb()? - 1)),
            4 => Ok(Constant::Import(reader.read_u32()? as usize)),
            5 => {
                let len = reader.read_leb()?;

                Ok(Constant::Table(
                    reader.read_list(len, |reader| reader.read_leb())?,
                ))
            }
            6 => Ok(Constant::Closure(reader.read_leb()?)),
            7 => Ok(Constant::Vector(
                reader.read_f32()?,
                reader.read_f32()?,
                reader.read_f32()?,
                reader.read_f32()?,
            )),
            _ => Err(Error::Malformed),
        }
    }
}
