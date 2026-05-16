//! The header definitions.
use crate::HEADER_SIZE;

/// The magic number, fixed to 'PKEX'
pub const PKEX_MAGIC: u32 = 0x58454B50;

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

    /// Signates is this executable run as `userapp` or `coredrv`.
    pub mode: ExecMode,

    /// The section table count.
    pub sections: u16,

    /// The author name (max length is 32 bytes).
    pub author: [u8; 32],

    /// The executable/project name.
    pub name: [u8; 32],

    /// Extended bits for different mode parsing.
    pub extended: [u8; 42],
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
            author: [0u8; 32],
            name: [0u8; 32],
            ..Default::default()
        }
    }

    /// Validate is this a valid proka executable.
    #[inline]
    pub fn validate(&self) -> bool {
        self.magic == PKEX_MAGIC
    }

    /// Convert this header to array
    #[inline]
    pub const fn to_array(&self) -> [u8; HEADER_SIZE] {
        // SAFETY: used `#[repr(C)]`
        unsafe { core::ptr::read(self as *const Self as *const [u8; HEADER_SIZE]) }
    }
}

/// The executable mode.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ExecMode {
    /// Run in `userapp` mode (Ring 3).
    UserApp,

    /// Run in `coredrv` mode (Ring 0).
    CoreDrv,
}
