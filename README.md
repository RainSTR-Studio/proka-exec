# Proka Executable - The definition of the proka executable

[![Rust Nightly](https://img.shields.io/badge/rust-nightly-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: GPLv3](https://img.shields.io/badge/License-GPLv3-yellow.svg?style=flat-square)](https://opensource.org/license/gpl-3.0)
[![GitHub Stars](https://img.shields.io/github/stars/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/RainSTR-Studio/proka-exec?style=flat-square)](https://github.com/RainSTR-Studio/proka-exec/pulls)
[![Documentation](https://img.shields.io/badge/docs-prokadoc-brightgreen?style=flat-square)](https://prokadoc.pages.dev/)

**Copyright (C) 2026 RainSTR Studio. Licensed under GNU GPLv3.**

---

## Introduction
This is the official project of the `proka-exec`, which contains the definition of the proka's headers, sections, 
and some utilities to help you parse the proka executable file easily.

Please note that this project is written in **Rust**, so you can use this crate through [crates.io](https://crates.io) and `cargo` to use this crate.

## Structures of this executable
This executable is structured as follows:
- PKE Headers - Records the basic information of this executable;
- Section index - Records the section header's offset and section name's length;
- Section metadata- Records the section flags, data offset and its length;
- Data - The binary content.

We can use this picture to explain their segmented structure:
`[Headers] [Section Index] [Section Metadata] [Data]`

Also, the `[Section Metadata]` can be separated as follows:
`[Section Headers] [Section Name]`

In the picture above, the `[Section Headers]`'s length is fixed, which is recorded in [`SECTION_HDR_SIZE`]; 
The section name is different - It's dynamic, so you can store almost infinite words in it!

## Contributing
Thank you to all contributors!

 - **zhangxuan2011** <zx20110412@outlook.com>

### How to contribute
We welcome your contributions: Bug reports, Pull Requests (features, fixes, optimizations), documentation improvements, and feedback.

Also don't forget to add your name to [**Contributors List**](#contributors)! :)

# License
This project is under license [**GPL-v3**](LICENSE), and you should follow this license during contributing.

See [LICENSE](LICENSE) for more details.
