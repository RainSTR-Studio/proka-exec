//! # `proka-exec`
//!
//! [![Rust Nightly](https://img.shields.io/badge/rust-nightly-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
//! [![License: GPLv3](https://img.shields.io/badge/License-GPLv3-yellow.svg?style=flat-square)](https://opensource.org/license/gpl-3.0)
//! [![GitHub Stars](https://img.shields.io/github/stars/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/stargazers)
//! [![GitHub Issues](https://img.shields.io/github/issues/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/issues)
//! [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/pulls)
//! [![Documentation](https://img.shields.io/badge/docs-prokadoc-brightgreen?style=flat-square)](https://prokadoc.pages.dev/)
//!
//! Copyright (C) 2026 RainSTR Studio. All rights reserved.
//!
//! ---
//!
//! ## Introduction
//! This crate provides the definitions of headers, section
//! entrys, and some utils to help you parse the executable
//! easily.
//!
//! ## Steps to use this crate
//! Before you parse it, you should do these steps:
//!
//! - Read the executable file content;
//! - Make this file's content to a slice (`&'static [u8]`)
//! - Use [`Parser`] to parse the executable.
//!
//! After this, you can do further operations through this parser by
//! calling its functions.
//!
//! ### Note
//! If you want to do minimal reading, you can just read the header and
//! section table, other content can be read later;
//!
//! Make sure you have read the header and each sections, and they are **NOT** optional!!!
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

pub mod header;
pub mod sections;
pub mod utils;

use header::Header;
use sections::{Section, SectionIter};
pub use utils::*;

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
        let header_raw = &buf[0..HEADER_SIZE]; // Header length
        let header = unsafe { *(header_raw.as_ptr() as *const Header) };

        // Check: Validate is this correct executable
        if !header.validate() {
            return Err(Error::NotValidExecutable);
        }

        Ok(Self {
            buf,
            header,
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

    /// An unknown character in UTF-8 was found in
    /// parsing arrays
    UnknownCharacter,
}
