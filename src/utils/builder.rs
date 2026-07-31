//! The builder for the proka executable.
use super::str_to_array;
use crate::header::{ExecMode, Header};
use crate::sections::{SectionFlag, SectionHdr, SectionIndex};
use crate::{Error, HEADER_SIZE, Result, SECTION_HDR_SIZE, SECTION_INDEX_SIZE};
#[cfg(feature = "alloc")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

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
        // Then create up a header and push into data...
        {
            let mut header = Header::default();
            header.min = self.min;
            header.max = self.max;
            header.entry_off = self.entry.0;
            header.entry_sec = self.entry.1 as u16;
            header.mode = self.mode;
            header.author = str_to_array(self.author.as_str());
            header.name = str_to_array(self.name.as_str());
            header.sections = self.sections.len() as u16;

            let header_bytes = header.to_array();
            data.extend_from_slice(&header_bytes);
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
