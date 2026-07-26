//! The definitions of section entry.
use crate::{Error, HEADER_SIZE, Result, SECTION_HDR_SIZE, SECTION_INDEX_SIZE, slice_to_str};
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};

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
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SectionHdr {
    /// The flag of the section.
    pub flag: SectionFlag,

    /// Paddings of header.
    pub _pad1: [u8; 3],

    /// The offset of the section start.
    pub base: u32,

    /// The size of the section.
    pub size: u32,

    /// Paddings of header.
    pub _pad2: [u8; 4],
}

bitflags! {
    /// Flags of this section.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, Pod, Zeroable)]
    pub struct SectionFlag: u8 {
        /// The section is loadable.
        const LOADABLE = 1;

        /// The section is executable.
        const EXECABLE = 1 << 1;
    }
}

impl SectionHdr {
    /// Convert this object to array.
    #[inline]
    pub const fn to_array(&self) -> [u8; SECTION_HDR_SIZE] {
        // SAFETY: used `#[repr(C)]`
        unsafe { core::ptr::read(self as *const Self as *const [u8; SECTION_HDR_SIZE]) }
    }

    /// Validate is this section not corrupted.
    #[inline]
    pub fn validate(&self) -> Result<()> {
        // Check: Is size 0
        if self.size == 0 {
            return Err(Error::SectionError(SectionError::LengthError));
        }

        Ok(())
    }
}

/// The index of each section entry.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SectionIndex {
    /// The base offset of the section header ([`SectionHdr`]).
    pub base: u32,

    /// The total length of the section name.
    pub name_len: u32,
}

impl SectionIndex {
    /// Convert this object to array.
    #[inline]
    pub const fn to_array(&self) -> [u8; SECTION_INDEX_SIZE] {
        // SAFETY: used `#[repr(C)]`
        unsafe { core::ptr::read(self as *const Self as *const [u8; SECTION_INDEX_SIZE]) }
    }
}

/// The iterator of each sections
#[derive(Debug, Clone, Copy)]
pub struct SectionTable<'a> {
    buf: &'a [u8],
    total: u16,
    current: u16,
}

impl<'a> SectionTable<'a> {
    /// Create a new object of this table.
    pub(crate) fn new(buf: &'a [u8], total: u16) -> Self {
        Self {
            buf,
            total,
            current: 0,
        }
    }

    /// A safe way to get [`SectionIndex`] by index.
    pub fn get(&self, index: usize) -> Option<SectionIndex> {
        // Check: Is given index out of bounds?
        if index >= self.total as usize {
            return None;
        }

        // Get the section index content
        let offset = HEADER_SIZE + index * SECTION_INDEX_SIZE;
        let slice = &self.buf[offset..offset + SECTION_INDEX_SIZE];

        // Initialize and copy content to entry
        let mut section_index = SectionIndex::zeroed();
        bytemuck::bytes_of_mut(&mut section_index).copy_from_slice(slice);
        Some(section_index)
    }

    /// Get the section header by using [`SectionIndex`].
    pub fn get_hdr_secindex(&self, section_index: SectionIndex) -> SectionHdr {
        // Get slice through section index
        let offset = section_index.base as usize;
        let slice = &self.buf[offset..offset + SECTION_HDR_SIZE];

        // Initialize and copy content to entry
        let mut hdr = SectionHdr::zeroed();
        bytemuck::bytes_of_mut(&mut hdr).copy_from_slice(slice);
        hdr
    }

    /// Get the section header by using index number.
    pub fn get_hdr_idx(&self, index: usize) -> Option<SectionHdr> {
        // Get slice through section index
        let section_index = self.get(index)?;
        Some(self.get_hdr_secindex(section_index))
    }

    /// Get the section name by using [`SectionIndex`].
    pub fn get_name_secindex(&self, section_index: SectionIndex) -> &str {
        // Get slice through section index
        let offset = section_index.base as usize + SECTION_HDR_SIZE;
        let content = &self.buf[offset..offset + section_index.name_len as usize];
        slice_to_str(content).expect("Failed to get section's name")
    }

    /// Get the section name by using index number.
    pub fn get_name_idx(&self, index: usize) -> Option<&str> {
        // Get slice through section index
        let section_index = self.get(index)?;
        Some(self.get_name_secindex(section_index))
    }
}

impl<'a> Iterator for SectionTable<'a> {
    type Item = SectionIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.get(self.current as usize)?;
        self.current += 1;
        Some(result)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_length() {
        assert_eq!(crate::SECTION_HDR_SIZE, 16);
        assert_eq!(core::mem::size_of::<SectionHdr>(), 16)
    }
}
