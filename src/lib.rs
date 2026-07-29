//! # `proka-exec`
//!
//! [![Rust Nightly](https://img.shields.io/badge/rust-nightly-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
//! [![License: GPLv3](https://img.shields.io/badge/License-GPLv3-yellow.svg?style=flat-square)](https://opensource.org/license/gpl-3.0)
//! [![GitHub Stars](https://img.shields.io/github/stars/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/stargazers)
//! [![GitHub Issues](https://img.shields.io/github/issues/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/issues)
//! [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/pulls)
//! [![Documentation](https://img.shields.io/badge/docs-prokadoc-brightgreen?style=flat-square)](https://prokadoc.pages.dev/)
//! 
//! Copyright (C) 2026 RainSTR Studio. Licensed under GNU GPLv3.
//! 
//! ---
//! 
//! ## Introduction
//! This crate provides the definitions of headers, section index, section metadata,
//! and some utils to help you parse the executable easily.
//! 
//! ## Structures of this executable
//! This executable is structured as follows:
//! - PKE headers - Records the basic information of this executable;
//! - Section index - Records the section header's offset and section name's length;
//! - Section metadata- Records the section flags, data offset and its length;
//! - Data - The binary content.
//! 
//! We can use this picture to explain their segmented structure:
//! 
//! `[Headers] [Section Index 1] [Section Index 2] ... [Section Metadata 1] [Section Metadata 2] ... [Data]`
//! 
//! Simultaneously, the `[Section Metadata]` can be separated as follows:
//! 
//! `[Section Headers] [Section Name]`
//! 
//! In the picture above, the `[Section Headers]`'s length is fixed, which is recorded in [`SECTION_HDR_SIZE`]; 
//! The section name is different - It's dynamic, so you can store almost infinite words in it!
//!
//! ## Steps to use this crate
//! ### Parsing
//! Before you parse it, you should do these steps:
//!
//! - Read the executable file content;
//! - Make this file's content to a slice (`&[u8]`)
//! - Use [`Parser`] to parse the executable.
//!
//! After this, you can do further operations through this parser by
//! calling its functions.
//! 
//! ### Building
//! Here we provided a tool [`Builder`] to help you do building process easily.
//! 
//! Before parsing, make sure you have enabled feature `alloc`, or you can't find where [`Builder`] is.
//! 
//! Then you can do these steps:
//!  - Set up author name (32 bytes limit);
//!  - Set up program name (32 bytes limit);
//!  - Set up the executable type (UserApp/CoreDrv);
//!  - Append section content (Can push multiple sections);
//!  - Build the executable.
//! 
//! ## Example Usage
//! ### Parsing
//! ```rust
//! use proka_exec::Parser;
//! use std::path::PathBuf;
//!
//! let file = PathBuf::from("tests/testbin/sample.pke");
//! let content = std::fs::read(file).expect("Failed to read file");
//! let parser = Parser::init(&content).expect("Failed to parse PKE format");
//! 
//! // More API see below
//! ```
//!
//! ### Building
//! ```rust
//! use proka_exec::{Builder, header::ExecMode};
//! use std::path::PathBuf;
//! 
//! static EXAMPLE_CONTENT: &[u8] = b"Hello, World!";
//!
//! let mut builder = Builder::new();
//! builder.set_author("example");
//! builder.set_name("appname");
//! builder.set_mode(ExecMode::UserApp);
//! builder.append(EXAMPLE_CONTENT, ".example.section", false, false, None);    // (data, name, is_loadable, is_execable, entry)
//! let content = builder.build().expect("Failed to build executable");
//! std::fs::write("example.pke", &content).expect("Failed to write file");
//! ```
//!
//! 
//! # LICENSE
//! This crate is under license [GPL-v3](https://github.com/RainSTR-Studio/proka-exec/blob/main/LICENSE),
//! and you must follow its rules.
//!
//! See [LICENSE](https://github.com/RainSTR-Studio/proka-exec/blob/main/LICENSE) file for more details.
//!
//! ## MSRV
//! This crate's MSRV is `1.85.0` stable.
#![no_std]

// Alloc features...
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod header;
pub mod sections;
pub mod utils;

#[cfg(feature = "alloc")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use header::HeaderError;
use header::{ExecMode, Header};
use sections::{SectionError, SectionHdr, SectionIndex, SectionTable};
pub use utils::*;

#[cfg(feature = "alloc")]
use crate::sections::SectionFlag;

/// Generic result type in this crate
pub type Result<T> = core::result::Result<T, Error>;

/// The header size.
pub const HEADER_SIZE: usize = core::mem::size_of::<Header>();

/// The section header size.
pub const SECTION_HDR_SIZE: usize = core::mem::size_of::<SectionHdr>();

/// The section entry size
pub const SECTION_INDEX_SIZE: usize = core::mem::size_of::<SectionIndex>();

/// The error type of parsing header.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Section inner error.
    ///
    /// See [`SectionError`] for more details.
    SectionError(SectionError),

    /// Header inner error.
    ///
    /// See [`HeaderError`] for more details.
    HeaderError(HeaderError),

    /// The executable is not valid
    ///
    /// Will appear if magic is not correct.
    NotValidExecutable,

    /// The section which is corrupted.
    ///
    /// Will appear if:  
    ///  - The buffer size is lower than specified length;
    ///  - Append an unexecable and unloadable section within an entry address (`Builder` only).
    ExecutableCorrupted,

    /// The version that was written in file is incorrect.
    ///
    /// Will appear if:
    ///  - The max version is lower than the min version;
    ///  - Passing a max version which is lower than min version (`Builder` only).
    ///
    /// # Arguments
    ///  - 0: The min version;
    ///  - 1: The max version.
    VersionIncorrect([u16; 3], [u16; 3]),

    /// An unknown character in UTF-8 was found in
    /// parsing arrays
    ///
    /// May appear in converting slice to `&str`.
    UnknownCharacter,

    /// No sections in the current executable.
    ///
    /// Will appear if you try to build without any appending.
    NoSections,
}

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

/// The builder of the proka executable.
#[derive(Debug, Clone)]
#[cfg(feature = "alloc")]
pub struct Builder<'a> {
    min: [u16; 3],
    max: [u16; 3],
    entry: (u32, usize), // (offset, index)
    author: String,
    name: String,
    mode: ExecMode,
    sections: Vec<InnerSections<'a>>,
}

#[cfg(feature = "alloc")]
impl Default for Builder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl<'a> Builder<'a> {
    /// Create up a empty builder.
    pub fn new() -> Self {
        Self {
            min: [0; 3],
            max: [0; 3],
            entry: (0, 0),
            author: String::new(),
            name: String::new(),
            mode: ExecMode::UserApp,
            sections: Vec::new(),
        }
    }

    /// Set up the author.
    ///
    /// # Note
    /// If the author that you provide is longer than 32,
    /// it may truncated.
    pub fn set_author(&mut self, author: &str) {
        self.author = author.to_string();
    }

    /// Set up the program name.
    ///
    /// # Note
    /// If the name that you provide is longer than 32,
    /// it may truncated.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Set the mode of this program.
    pub fn set_mode(&mut self, mode: ExecMode) {
        self.mode = mode;
    }

    /// Set the min version.
    pub fn set_min(&mut self, min: [u16; 3]) {
        self.min = min;
    }

    /// Set the max version.
    pub fn set_max(&mut self, max: [u16; 3]) {
        self.max = max;
    }

    /// Append a section and specify its name.
    ///
    /// # Arguments
    ///  - `data`: The data that you want to append;
    ///  - `name`: The section name;
    ///  - `is_loadable`: Assign is this loadable section or not;
    ///  - `is_execable`: Assign is this executable section or not;
    ///  - `entry`: The offset of the entry point, pass `None` if no entry point.
    ///
    /// # Errors
    /// This will return error once these happened:
    ///  - Provide an entry address which is unloadable or unexecable;
    ///
    /// # Note
    ///  - If you try to provide a name which is over than 16 bytes, it may truncated;
    ///  - If you provide the entry offset for multiple times, once you invoke `build()`, it will
    ///    use that latest set one.
    pub fn append(
        &mut self,
        data: &'a [u8],
        name: &'a str,
        is_loadable: bool,
        is_execable: bool,
        entry: Option<u32>,
    ) -> Result<()> {
        // Check: Is entry is Some(...) within unloadable & unexecable
        if entry.is_some() && !(is_execable && is_loadable) {
            return Err(Error::ExecutableCorrupted);
        }

        let flag = match (is_loadable, is_execable) {
            (true, true) => SectionFlag::LOADABLE | SectionFlag::EXECABLE,
            (true, false) => SectionFlag::LOADABLE,
            (false, true) => SectionFlag::EXECABLE,
            (false, false) => SectionFlag::empty(),
        };

        let section = InnerSections {
            secinfo: SectionHdr {
                flag,
                _pad1: [0; 3],
                base: 0, // Will replace during building...
                size: data.len() as u32,
                _pad2: [0; 4],
            },
            secindex: SectionIndex {
                base: 0, // Will replace during building...
                name_len: name.len() as u32,
            },
            name,
            data,
        };
        self.sections.push(section);

        // Set entry if Some(...)...
        if let Some(ent_offset) = entry {
            let sec_index = self.sections.len() - 1;
            self.entry = (ent_offset, sec_index);
        }
        Ok(())
    }

    /// Build the whole file to a valid exec format.
    ///
    /// Will return error if no section was appended.
    pub fn build(self) -> Result<Vec<u8>> {
        // Check: Is section list empty
        if self.sections.is_empty() {
            return Err(Error::NoSections);
        }

        // Check: Is min version lower than max version
        for (&min, &max) in self.min.iter().zip(self.max.iter()) {
            if min > max {
                return Err(Error::VersionIncorrect(self.min, self.max));
            }
        }

        // Create up a data...
        let mut data: Vec<u8> = Vec::new();

        // Then create up a header and push into data...
        {
            let header = Header {
                min: self.min,
                max: self.max,
                entry_off: self.entry.0,
                entry_sec: self.entry.1 as u16,
                mode: self.mode,
                author: str_to_array(self.author.as_str()),
                name: str_to_array(self.name.as_str()),
                sections: self.sections.len() as u16,
                ..Default::default()
            }
            .to_array();
            data.extend_from_slice(&header);
        }

        // And section index...
        let mut cnt = 0;
        for section in &self.sections {
            let mut secindex = section.secindex;
            secindex.base = (HEADER_SIZE + self.sections.len() * SECTION_INDEX_SIZE + cnt) as u32;
            data.extend_from_slice(&secindex.to_array());
            cnt += SECTION_HDR_SIZE + secindex.name_len as usize;
        }

        // And each section info...
        // Here we didn't empty the `cnt`, so that `cnt` is already store the whole section index.
        // So that seems no something wrong in this calculation...
        for section in &self.sections {
            let mut secinfo = section.secinfo;

            // Update base...
            // Note: The `cnt` does not empty, which means that is already store the whole section index.
            secinfo.base = (HEADER_SIZE + self.sections.len() * SECTION_INDEX_SIZE + cnt) as u32;

            // Push...
            data.extend_from_slice(&secinfo.to_array());
            data.extend_from_slice(section.name.as_bytes());
            cnt += section.data.len();
        }

        // And each section's data...
        for section in &self.sections {
            data.extend_from_slice(section.data);
        }

        // Return
        Ok(data)
    }
}

/// Internal section form.
#[derive(Debug, Clone, Copy)]
struct InnerSections<'a> {
    pub secinfo: SectionHdr,
    pub secindex: SectionIndex,
    pub name: &'a str,
    pub data: &'a [u8],
}
