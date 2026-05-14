//! The proka executable header definitions.
#![no_std]

pub mod header;
pub mod sections;

use header::Header;
use sections::{SectionIter, Section};

/// The header size.
pub const HEADER_SIZE: usize = core::mem::size_of::<Header>();

/// The section entry size
pub const SECTION_SIZE: usize = core::mem::size_of::<Section>();

/// The parser of the proka executable.
///
/// # Usage
/// To use this parser, you must put an slice into the initializations.
///
/// If the content of the proka executable is in memory, the best way
/// is to use `core::slice::from_raw_parts`.
#[derive(Debug, Clone, Copy)]
pub struct Parser {
    buf: &'static [u8],
    header: Header,
    total_sections: u16,
}

impl Parser {
    /// Initialize the parser by passing an slice.
    ///
    /// # Safety
    /// You must ensure these before invoking this function:
    ///
    ///  - The slice's content is a valid executable;
    ///  - The slice's pointer is accessable and mapped;
    ///  - The slice's length is larger than 128 bytes.
    ///
    /// If this crate has used by kernel, you need to do mapping 
    /// first, and invoke this.
    pub unsafe fn init(buf: &'static [u8]) -> Result<Self, Error> {
        let header_raw = &buf[0..HEADER_SIZE];    // Header length
        let header = unsafe { *(header_raw.as_ptr() as *const Header) };

        // Check: Validate is this correct executable
        if !header.validate() {
            return Err(Error::NotValidExecutable);
        }
        
        Ok(Self {
            buf, header,
            total_sections: header.sections,
        })
    }

    /// Get the header.
    #[inline]
    pub fn header(&self) -> Header {
        self.header
    }

    /// Get each section table.
    #[allow(private_interfaces)]
    pub fn sections(&self) -> SectionIter {
        SectionIter {
            buf: self.buf,
            total: self.total_sections,
            current: 0,
        }
    }
}

/// The error type of parsing header.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The executable is not valid.
    NotValidExecutable,
}
