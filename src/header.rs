//! The header definitions.
use crate::{Error, HEADER_SIZE, Result};

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
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    /// The magic number, fixed to 'PKEX'
    pub magic: u32,

    /// The minimal kernel version supported.
    ///
    /// # Note
    /// As the `proka-bootloader`'s definitions, its format is similar
    /// like `[major, minor, fix]`. See `proka-bootloader` crate for more informations.
    pub min: [u16; 3],

    /// The maximum kernel supported.
    ///
    /// For notes, see above.
    pub max: [u16; 3],

    /// Signifies is this executable run as `userapp` or `coredrv`.
    pub mode: ExecMode,

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

    /// Extended bits for different mode parsing (reserved).
    pub extended: [u8; 36],
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
            min: [0; 3],
            max: [0; 3],
            mode: ExecMode::UserApp,
            sections: 0,
            entry_sec: 0,
            entry_off: HEADER_SIZE as u32,
            author: [0; 32],
            name: [0; 32],
            extended: [0u8; 36],
        }
    }

    /// Validate is this a valid proka executable.
    #[inline]
    pub fn validate(&self) -> Result<()> {
        if self.magic != PKEX_MAGIC {
            return Err(Error::HeaderError(HeaderError::MagicNumberError(self.magic)));
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

/// The executable mode.
#[repr(u32)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecMode {
    /// Run in `userapp` mode (Ring 3).
    #[default]
    UserApp,

    /// Run in `coredrv` mode (Ring 0).
    CoreDrv,
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
