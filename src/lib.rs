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

use header::{HeaderError, Header};
use sections::{SectionError, SectionHdr, SectionIndex};
pub use utils::*;
pub use utils::{builder::Builder, parser::Parser};

/// Generic result type in this crate
pub type Result<T> = core::result::Result<T, Error>;

/// The header size.
pub const HEADER_SIZE: usize = core::mem::size_of::<Header>();

/// The section header size.
pub const SECTION_HDR_SIZE: usize = core::mem::size_of::<SectionHdr>();

/// The section entry size.
pub const SECTION_INDEX_SIZE: usize = core::mem::size_of::<SectionIndex>();

/// The version of this crate.
pub const VERSION_CURRENT: u32 = 3;

/// The minimum version of this crate.
pub const VERSION_MINIMAL: u32 = 3;

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
