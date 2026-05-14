//! The definitions of section entry.
use crate::{HEADER_SIZE, SECTION_SIZE};

/// A section entry.
#[repr(C)]
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
    pub length: u32
}

/// The iterator of each sections
#[derive(Debug, Clone, Copy)]
pub(crate) struct SectionIter {
    pub buf: &'static [u8],
    pub total: u16,
    pub current: u16,
}

// Iterator implementations
impl Iterator for SectionIter {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        let base = HEADER_SIZE + self.current as usize * SECTION_SIZE; 
        let buf = &self.buf[base..base + SECTION_SIZE];

        // Check: is current over than total
        if self.current >= self.total {
            return None;
        }

        // Now convert it
        let section = unsafe { *(buf.as_ptr() as *const Section) };
        Some(section)
    }
}
