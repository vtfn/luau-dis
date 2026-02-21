use crate::{Instruction, OPCODE_NAMES, Opcode, Sink};
use std::fmt::Write;

#[repr(transparent)]
#[derive(Debug)]
pub struct LuacFormatter {
    sink: Sink,
}

impl LuacFormatter {
    pub fn new() -> Self {
        Self { sink: Sink::new() }
    }

    pub fn format(&mut self, ins: &Instruction) {
        self.sink.clear();

        let name = OPCODE_NAMES[ins.opcode() as usize];
        _ = self.sink.write_str(name);

        use Opcode::*;

        _ = match ins.opcode() {
            LOADNIL | CLOSEUPVALS | PREPVARARGS | JUMPX | COVERAGE => {
                write!(self.sink, " {}", ins.a())
            }

            JUMP | JUMPBACK => write!(self.sink, " {}", ins.d()),

            MOVE | GETUPVAL | SETUPVAL | RETURN | NOT | MINUS | LENGTH | CAPTURE => {
                write!(self.sink, " {}, {}", ins.a(), ins.b())
            }

            LOADN | LOADK | NEWCLOSURE | JUMPIF | JUMPIFNOT | DUPTABLE | FORNPREP | FORNLOOP
            | FORGPREP_INEXT | FORGPREP_NEXT | GETVARARGS | DUPCLOSURE | FORGPREP => {
                write!(self.sink, " {}, {}", ins.a(), ins.d())
            }

            FASTCALL => write!(self.sink, " {}, {}", ins.a(), ins.c()),

            LOADKX => write!(self.sink, " {}, {}", ins.a(), ins.aux()),

            LOADB | GETTABLE | SETTABLE | GETTABLEN | SETTABLEN | CALL | ADD | SUB | MUL | DIV
            | MOD | POW | ADDK | SUBK | MULK | DIVK | MODK | POWK | AND | OR | ANDK | ORK
            | CONCAT | SUBRK | DIVRK | FASTCALL1 | IDIV | IDIVK => {
                write!(self.sink, " {}, {}, {}", ins.a(), ins.b(), ins.c())
            }

            GETGLOBAL | SETGLOBAL => write!(self.sink, " {}, {}, {}", ins.a(), ins.c(), ins.aux()),

            GETIMPORT | FORGLOOP | JUMPIFEQ | JUMPIFLE | JUMPIFLT | JUMPIFNOTEQ | JUMPIFNOTLE
            | JUMPIFNOTLT => {
                write!(self.sink, " {}, {}, {}", ins.a(), ins.d(), ins.aux())
            }

            NEWTABLE | JUMPXEQKNIL | JUMPXEQKB | JUMPXEQKN | JUMPXEQKS => {
                write!(self.sink, " {}, {}, {}", ins.a(), ins.d(), ins.aux())
            }

            #[rustfmt::skip]
            GETTABLEKS | SETTABLEKS => write!(self.sink, " {}, {}, {}, {}", ins.a(), ins.b(), ins.c(), ins.aux()),

            #[rustfmt::skip]
            NAMECALL | SETLIST | FASTCALL3 | FASTCALL2 | FASTCALL2K => write!(self.sink, " {}, {}, {}, {}", ins.a(), ins.b(), ins.c(), ins.aux()),

            NOP | BREAK | NATIVECALL => Ok(()),
        };
    }

    pub const fn as_str(&self) -> &str {
        self.sink.as_str()
    }
}
