use std::{
    fmt::Debug,
    os::raw::{c_char, c_uchar},
    str::{self, Utf8Error},
};

#[derive(Debug, Clone)]
pub struct StrBuffer<const SIZE: usize>(Vec<u8>);

impl<const SIZE: usize> StrBuffer<SIZE> {
    pub fn new() -> StrBuffer<SIZE> {
        StrBuffer(vec![0; SIZE])
    }

    pub fn from_str(str: &str) -> StrBuffer<SIZE> {
        let mut new = Self::new();

        let bytes = str.as_bytes();

        assert!(bytes.len() <= SIZE - 1);

        for i in 0..bytes.len() {
            new.0[i] = bytes[i];
        }

        new.0[bytes.len()] = 0;

        new
    }

    pub fn capacity(&self) -> usize {
        SIZE
    }

    pub unsafe fn from_raw_copy(str: *const c_uchar) -> StrBuffer<SIZE> {
        let mut new = Self::new();
        let mut null_terminated = false;

        for i in 0..new.0.len() {
            let current_char = *str.add(i);

            new.0[i] = current_char;

            if current_char == 0 {
                null_terminated = true;
                break;
            }
        }

        assert!(null_terminated);

        new
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr() as *const c_char
    }

    pub fn as_mut_ptr(&self) -> *mut c_char {
        self.0.as_ptr() as *mut c_char
    }

    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        let end = self.0.iter().position(|x| *x == b'\0').unwrap();
        str::from_utf8(&self.0[0..end])
    }
}
