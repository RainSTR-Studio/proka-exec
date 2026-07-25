//! The definitions of section entry.
use crate::{HEADER_SIZE, Result, Error, SECTION_SIZE};
use core::ops::Index;

/// Errors in section
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionError {
    /// Section length error.
    ///
    /// Will appear if recorded section length is 0.
    LengthError,

    /// Section base error.
    ///
    /// Will appear if specified section base is lower than metadata length (128+32).
    ///
    /// Contains the incorrect base.
    BaseError(u32),


    /// Entry offset out of range.
    /// 
    /// Will appear if entry offset is out of range of the section.
    /// 
    /// Contains the incorrect entry offset (0) and the section length (1).
    EntryOffsetOutOfRange(u32, u32),
}

/// A section entry.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Section {
    /// The section name (16 bytes max).
    pub name: [u8; 16],

    /// Assign is this section loadable
    pub is_loadable: bool,

    /// Assign is this section executable
    pub is_execable: bool,

    /// The offset of the section start.
    pub base: u32,

    /// The length of the section.
    pub length: u32,

    /// Reserved bits
    pub _reserved: [u8; 6],
}

impl Section {
    /// Convert this object to array.
    #[inline]
    pub const fn to_array(&self) -> [u8; SECTION_SIZE] {
        // SAFETY: used `#[repr(C)]`
        unsafe { core::ptr::read(self as *const Self as *const [u8; SECTION_SIZE]) }
    }

    /// Validate is this section not corrupted.
    #[inline]
    pub fn validate(&self) -> Result<()> {
        // Check: Is length 0
        if self.length == 0 {
            return Err(Error::SectionError(SectionError::LengthError));
        }

        // Check: Is base lower than metadata length (128+32)
        if self.base as usize >= (HEADER_SIZE + SECTION_SIZE) {
            return Err(Error::SectionError(SectionError::BaseError(self.base)));
        }

        Ok(())
    }
}

/// The iterator of each sections
#[derive(Debug, Clone, Copy)]
pub struct SectionIter<'a> {
    buf: &'a [u8],
    total: u16,
    current: u16,
}
impl<'a> SectionIter<'a> {
    pub(crate) fn new(buf: &'a [u8], total: u16, current: u16) -> Self {
        Self {
            buf,
            total,
            current,
        }
    }
}

/// Iterator implementations.
impl<'a> Iterator for SectionIter<'a> {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        // Check: is current over than total
        if self.current >= self.total {
            return None;
        }

        let base = HEADER_SIZE + self.current as usize * SECTION_SIZE;
        let buf = &self.buf[base..base + SECTION_SIZE];

        // Now convert it
        let section = unsafe { *(buf.as_ptr() as *const Section) };

        // Plus current value and return
        self.current += 1;
        Some(section)
    }
}

/// The index implementations.
///
/// # Panics
/// This will panic if your index is over than the length.
impl Index<usize> for SectionIter<'_> {
    type Output = Section;

    fn index(&self, index: usize) -> &Self::Output {
        // Check: Is index out if bounds
        if index >= self.total as usize {
            panic!(
                "proka-exec: index out of bounds, the max size is {}, but index {} was got.",
                self.total, index
            )
        }

        // Calculate target
        let base = HEADER_SIZE;
        let target = base + index * SECTION_SIZE;

        // Get and convert
        let buf = &self.buf[target..target + SECTION_SIZE];
        unsafe { &*(buf.as_ptr() as *const Section) }
    }
}
