use crate::bytecode::Constant;
use crate::{Error, Reader, Result};

#[derive(Debug)]
pub struct Proto<'a> {
    pub attr: Attributes,
    pub name: Option<usize>,
    pub instructions: &'a [u8],
    pub constants: Vec<Constant>,
    pub protos: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct Attributes {
    pub max_stack_size: u8,
    pub num_params: u8,
    pub num_upvals: u8,
    pub is_vararg: u8,
    pub flags: u8,
}

impl<'a> Proto<'a> {
    pub(crate) fn parse(reader: &mut Reader<'a>) -> Result<Self> {
        let attr: Attributes = reader.read_raw()?;

        let type_info_len = reader.read_leb()?;
        reader.try_advance(type_info_len)?;

        let instructions = {
            let ins_len = reader.read_leb()?;

            let ins_len = ins_len * 4;

            if ins_len > reader.rem() {
                return Err(Error::Exhausted);
            }

            let ptr = reader.as_ptr();

            unsafe {
                reader.advance_unchecked(ins_len);
                std::slice::from_raw_parts(ptr, ins_len)
            }
        };

        let constants = {
            let len = reader.read_leb()?;

            reader.read_list(len, Constant::parse)
        }?;

        let protos = {
            let len = reader.read_leb()?;

            reader.read_list(len, Reader::read_leb)
        }?;

        let _line = reader.read_leb()?;

        let name = reader.read_leb()?;
        let name = if name != 0 { Some(name - 1) } else { None };

        let has_line_info = reader.read_u8()? != 0;

        if has_line_info {
            let logspan = reader.read_u8()?;

            let ins_count = instructions.len() / 4;
            let _line_info_delta = reader.read_list(ins_count, Reader::read_u8);

            let baseline_size = (ins_count - 1) / 2usize.pow(logspan as u32) + 1;
            let _abs_line_info = reader.read_list(baseline_size, Reader::read_u32);
        }

        let has_debug_info = reader.read_u8()?;

        if has_debug_info != 0 {
            return Err(Error::Unimplemented);
        }

        Ok(Proto {
            attr,
            name,
            instructions,
            constants,
            protos,
        })
    }
}
