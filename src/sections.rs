//! The definitions of section entry.

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