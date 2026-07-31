//! The parser of proka executable.
use crate::header::Header;
use crate::sections::{SectionError, SectionTable, SectionIndex};
use crate::{Result, Error, SECTION_HDR_SIZE, SECTION_INDEX_SIZE, HEADER_SIZE};

/// The parser of the proka executable.
///
/// # Usage
/// To use this parser, you must put an slice into the initializations.
///
/// If the content of the proka executable is in memory, the best way
/// is to use `core::slice::from_raw_parts`.
#[derive(Debug, Clone, Copy)]
pub struct Parser<'a> {
    buf: &'a [u8],
    header: Header,
    total_sections: u16,
}

impl<'a> Parser<'a> {
    /// Initialize the parser by passing a slice.
    ///
    /// This is the recommended way to initialize this parser, because it will
    /// help you do all checks and return error if something wrong, so you can
    /// leave everything about parsing to us :)
    ///
    /// # Note
    /// If this crate is used on the kernel-side, you must first map the memory
    /// that the slice points to before invoking this function.
    pub fn init(buf: &'a [u8]) -> Result<Self> {
        let header_raw = &buf[0..HEADER_SIZE]; // Header length
        let header = unsafe { *(header_raw.as_ptr() as *const Header) };

        // Check: Validate is this correct executable
        if header.validate().is_err() {
            return Err(Error::NotValidExecutable);
        }

        // Check: Is section count = 0?
        if header.sections == 0 {
            return Err(Error::NoSections);
        }

        // Check: Is the buffer contains all sections
        let offset = HEADER_SIZE + (header.sections as usize - 1) * SECTION_INDEX_SIZE;
        let index_content = &buf[offset..offset + SECTION_INDEX_SIZE];
        let index = unsafe { *(index_content.as_ptr() as *const SectionIndex) };
        let len = (index.base + index.name_len) as usize + SECTION_HDR_SIZE;
        if buf.len() < len {
            return Err(Error::ExecutableCorrupted);
        }

        // SAFETY: Already check all staff and able to do initialization
        unsafe { Ok(Self::init_unchecked(buf)) }
    }

    /// Initialize the parser by passing a slice without checking.
    ///
    /// # Safety
    /// You must ensure these if you invoke this function:
    ///
    ///  - The slice's content is a valid proka executable (match the magic);
    ///  - The slice must contain the header and all section tables.
    ///
    /// # Note
    /// Use this function to initialize is **NOT** recommended, because it might
    /// cause some problems while parsing this header.
    pub unsafe fn init_unchecked(buf: &'a [u8]) -> Self {
        let header_raw = &buf[0..HEADER_SIZE];
        let header = unsafe { *(header_raw.as_ptr() as *const Header) };

        Self {
            buf,
            header,
            total_sections: header.sections,
        }
    }

    /// Do more validation after initialization.
    ///
    /// # Content
    /// This will validates:
    ///
    ///  - Is the header min >= max;
    ///  - Is each section's base correct;
    ///  - Is the section's length not zeroed;
    ///  - Is section base out of length;
    ///  - Is entry_off is over than section length.
    pub fn validate(&self) -> Result<()> {
        // Check: Is header's min > max
        let minimal = self.header.min;
        let maximum = self.header.max;
        for (&min, &max) in minimal.iter().zip(maximum.iter()) {
            if min > max {
                return Err(Error::VersionIncorrect(minimal, maximum));
            }
        }

        // Check: Is each section's base and length correct (section check)
        let min_base = HEADER_SIZE + self.header.sections as usize * SECTION_HDR_SIZE;
        for (index, section_index) in self.sections().enumerate() {
            let section = self.sections().get_hdr_secindex(section_index);
            let base_off = section.base as usize;
            let len = section.size as usize;
            let entry_sec = self.header.entry_sec as usize;

            // Check: Is section base in metadata range
            if base_off < min_base {
                return Err(Error::SectionError(SectionError::BaseError(
                    base_off as u32,
                )));
            }

            // Check: Is section length not zeroed
            if len == 0 {
                return Err(Error::SectionError(SectionError::LengthError));
            }

            // Check: Is section entry_off out of range
            if index == entry_sec {
                let entry_off = self.header.entry_off as usize;
                if entry_off > len {
                    return Err(Error::SectionError(SectionError::EntryOffsetOutOfRange(
                        entry_off as u32,
                        len as u32,
                    )));
                }
            }
        }

        // All's fine :)
        Ok(())
    }

    /// Get the content from specified sections.
    ///
    /// # Arguments
    ///  - `secname`: The name of the section
    ///
    /// # Returns
    /// `Option<&'static [u8]>`: The content of this section, return `None` if this section not exist.
    pub fn get_section_content(&self, secname: &str) -> Option<&'a [u8]> {
        // Iterate all sections...
        for section_index in self.sections() {
            let table = self.sections();
            let name = table.get_name_secindex(section_index);
            let section = table.get_hdr_secindex(section_index);
            if secname == name {
                // Get its base and length
                let base = section.base as usize;
                let length = section.size as usize;
                let content = &self.buf[base..base + length];
                return Some(content);
            }
        }

        None
    }

    /// Get the header in this buffer.
    #[inline]
    pub fn header(&self) -> Header {
        self.header
    }

    /// Get each section table.
    pub fn sections(&self) -> SectionTable<'_> {
        SectionTable::new(self.buf, self.total_sections)
    }
}