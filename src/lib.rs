// Copyright (c) 2021-2025 José Manuel Barroso Galindo <theypsilon@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// You can download the latest version of this tool from:
// https://github.com/theypsilon/downloader-rs

//! MiSTer downloader library.
//!
//! This is a Rust implementation of the algorithm used in the
//! [Downloader_MiSTer Python module](https://github.com/MiSTer-devel/Downloader_MiSTer)
//! for maximum speed.

/// Initialize the downloader library.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn downloader_init() -> i32 {
    0
}

/// A test function
///
/// # Returns
///
/// The answer. We're still trying to determine the question, but that requires
/// more computing power than we have. Perhaps if people would stop wasting
/// GPUs on cryptomining and AI, we might find an answer.
///
pub fn the_42() -> i32 {
    42
}

pub mod delme;
pub mod filesystem;
pub mod other;
