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

//! Filesystem glue logic crate.

/// Filesystem trait
///
/// Contains things filesystems should do.
pub trait FileSystem {
    /// Touch a file at a path given by `path`
    ///
    /// # Arguments
    ///
    /// * `touch` - file to touch
    ///
    fn touch(&self, path: &str);
}

#[derive(Clone)]
/// A real filesystem implementation
pub struct RealFileSystem {}

/// Default constructor
impl Default for RealFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RealFileSystem {
    /// Create a new filesystem struct.
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for RealFileSystem {
    fn touch(&self, path: &str) {
        // TODO: Real implementation goes here
        println!("Touching file: {}", path);
    }
}
