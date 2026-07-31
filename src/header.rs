//! The header definitions.
use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};

use crate::{Error, HEADER_SIZE, Result, VERSION_CURRENT, VERSION_MINIMAL};

/// The magic number, fixed to 'PKEX'
pub const PKEX_MAGIC: u32 = 0x58454B50;

/// Error types in header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderError {
    /// Magic number error.
    ///
    /// Contains the incorrect magic number.
    MagicNumberError(u32),
}

/// The main header struct, which contains the metadata of the PKE file.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Header {
    /// The magic number, fixed to 'PKEX'
    pub magic: u32,

    /// Format version used to generate this PKEX binary.
    ///
    /// Represents the PKEX file specification revision that the builder followed
    /// when creating this executable. This is separate from `version_minimum`.
    ///
    /// - Breaking binary layout changes: increment this value, set `version_minimum` equal to it.
    /// - Backward-compatible extensions (only using reserved space): increment this value,
    ///   leave `version_minimum` unchanged.
    pub version_current: u32,

    /// Lowest PKEX format version that can parse this binary correctly.
    ///
    /// Loader enforcement rule: If the maximum format version supported by the loader
    /// is less than this value, the loader **MUST reject loading this executable**.
    pub version_minimal: u32,

    /// Padding field #1.
    _pad1: [u8; 4],

    /// The minimal kernel version supported.
    ///
    /// # Note
    /// As the `proka-bootloader`'s definitions, its format is similar
    /// like `[major, minor, fix]`. See [`proka-bootloader`](https://docs.rs/proka-bootloader) crate for more informations.
    pub min: [u16; 3],

    /// Padding field #2.
    _pad2: [u8; 2],

    /// The maximum kernel supported.
    ///
    /// For notes, see above.
    pub max: [u16; 3],

    /// Padding field #3.
    _pad3: [u8; 2],

    /// Signifies is this executable run as `userapp` or `coredrv`.
    pub mode: ExecMode,

    /// Padding field #4.
    _pad4: [u8; 3],

    /// The section table count.
    pub sections: u16,

    /// The section which contains the entry point.
    pub entry_sec: u16,

    /// The entry offset of the section.
    pub entry_off: u32,

    /// The author name (max length is 32 bytes).
    pub author: [u8; 32],

    /// The executable/project name.
    pub name: [u8; 32],

    /// Reserved fields
    pub _reserved: [u8; 20],
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Header {
    /// Create a header object.
    pub fn new() -> Self {
        Self {
            magic: PKEX_MAGIC,
            version_current: VERSION_CURRENT,
            version_minimal: VERSION_MINIMAL,
            _pad1: [0; 4],
            min: [0; 3],
            _pad2: [0; 2],
            max: [0; 3],
            _pad3: [0; 2],
            mode: ExecMode::UserApp,
            _pad4: [0; 3],
            sections: 0,
            entry_sec: 0,
            entry_off: HEADER_SIZE as u32,
            author: [0; 32],
            name: [0; 32],
            _reserved: [0; 20],
        }
    }

    /// Validate is this a valid proka executable.
    #[inline]
    pub fn validate(&self) -> Result<()> {
        if self.magic != PKEX_MAGIC {
            return Err(Error::HeaderError(HeaderError::MagicNumberError(
                self.magic,
            )));
        }
        Ok(())
    }

    /// Convert this header to array
    #[inline]
    pub const fn to_array(&self) -> [u8; HEADER_SIZE] {
        // SAFETY: used `#[repr(C)]`
        unsafe { core::ptr::read(self as *const Self as *const [u8; HEADER_SIZE]) }
    }
}

bitflags! {
    /// The executable mode.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, Pod, Zeroable)]
    pub struct ExecMode: u8 {
        /// Run in `userapp` mode (Ring 3).
        const UserApp = 0x00000000;

        /// Run in `coredrv` mode (Ring 0).
        const CoreDrv = 0x00000001;
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_length() {
        assert_eq!(crate::HEADER_SIZE, 128);
        assert_eq!(core::mem::size_of::<Header>(), 128)
    }
}
