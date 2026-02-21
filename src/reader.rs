use core::ptr::{self, NonNull};

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Reader<'a> {
    ptr: NonNull<u8>,
    end: NonNull<u8>,
    _marker: core::marker::PhantomData<&'a [u8]>,
}

impl<'a> Reader<'a> {
    pub const fn new(buf: &'a [u8]) -> Self {
        let ptr = NonNull::from_ref(buf).cast();
        let end = unsafe { ptr.byte_add(buf.len()) };

        Self {
            ptr,
            end,
            _marker: core::marker::PhantomData,
        }
    }

    pub const fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    pub const fn rem(&self) -> usize {
        unsafe { self.end.byte_offset_from_unsigned(self.ptr) }
    }

    pub const fn try_advance(&mut self, offset: usize) -> Result<()> {
        if offset > self.rem() {
            return Err(Error::Exhausted);
        } else {
            unsafe {
                self.advance_unchecked(offset);
            }

            Ok(())
        }
    }

    pub const unsafe fn advance_unchecked(&mut self, offset: usize) {
        self.ptr = unsafe { self.ptr.byte_add(offset) };
        unsafe { core::hint::assert_unchecked(self.end.byte_offset_from(self.ptr) >= 0) };
    }

    pub const fn advance(&mut self, offset: usize) {
        let offset = if offset > self.rem() {
            self.rem()
        } else {
            offset
        };

        unsafe {
            self.advance_unchecked(offset);
        }
    }

    #[inline(always)]
    pub(crate) const fn read_raw<T: Sized + Copy>(&mut self) -> Result<T> {
        let size: usize = core::mem::size_of::<T>();

        unsafe {
            if self.rem() >= size {
                let out = ptr::read_unaligned(self.as_ptr() as *const T);
                self.advance_unchecked(size);

                return Ok(out);
            }
        }

        Err(Error::Exhausted)
    }

    #[inline(always)]
    pub const fn read_leb(&mut self) -> Result<usize> {
        let mut res: usize = 0;
        let mut shift = 0;
        let mut i = 0;

        while let Ok(byte) = self.read_u8() {
            res |= ((byte & 127) as usize) << shift;

            if byte & 128 == 0 {
                return Ok(res);
            }

            shift += 7;
            i += 1;

            if i == 3 {
                return Err(Error::Malformed);
            }
        }

        Err(Error::Exhausted)
    }

    pub fn read_list<T, F>(&mut self, len: usize, mut parse: F) -> Result<Vec<T>>
    where
        F: for<'r> FnMut(&'r mut Reader<'a>) -> Result<T>,
    {
        let mut list = Vec::new();
        list.reserve_exact(len);

        for _ in 0..len {
            list.push(parse(self)?);
        }

        Ok(list)
    }
}

macro numeric_impl($t:tt, $name:ident) {
    impl<'a> Reader<'a> {
        #[inline(always)]
        pub const fn $name(&mut self) -> Result<$t> {
            self.read_raw()
        }
    }
}

numeric_impl!(u8, read_u8);
numeric_impl!(u16, read_u16);
numeric_impl!(u32, read_u32);
numeric_impl!(i32, read_i32);
numeric_impl!(f32, read_f32);
numeric_impl!(f64, read_f64);
