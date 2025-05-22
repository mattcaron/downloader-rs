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

//! The delme module. It should be deleted.
//!
//! This is just a POC. It should be deleted after real components with their
//! respective test suites are added to the project.

use std::rc::Rc;

use crate::filesystem::*;

/// A test service
#[derive(Clone)]
pub struct ServiceA {
    fs: Rc<dyn FileSystem>,
    a: i32,
}

impl ServiceA {
    /// Create a new ServiceA with a given filesystem path and value
    pub fn new(fs: Rc<dyn FileSystem>, a: i32) -> Self {
        Self { fs, a }
    }

    /// A test function
    ///
    /// Touches the file "a" in the stored filesystem path and returns the
    /// stored value.
    pub fn a(&self) -> i32 {
        self.fs.touch("a");
        self.a
    }
}

/// Another test service
pub struct ServiceB {
    fs: Rc<dyn FileSystem>,
    b: i32,
}

impl ServiceB {
    /// Create a new ServiceB with a given filesystem path and value
    pub fn new(fs: Rc<dyn FileSystem>, b: i32) -> Self {
        Self { fs, b }
    }

    /// Another test function
    ///
    /// Touches the file "b" in the stored filesystem path and returns the
    /// stored value.
    pub fn b(&self) -> i32 {
        self.fs.touch("b");
        self.b
    }
}

/// A third test service
pub struct ServiceC {
    fs: Rc<dyn FileSystem>,
    service_a: Rc<ServiceA>,
    service_b: Rc<ServiceB>,
}

impl ServiceC {
    /// Create a new ServiceC with a given filesystem path, ServiceA and
    /// ServiceB structs.
    pub fn new(fs: Rc<dyn FileSystem>, service_a: Rc<ServiceA>, service_b: Rc<ServiceB>) -> Self {
        Self {
            fs,
            service_a,
            service_b,
        }
    }

    /// Calculate the result
    ///
    /// Touches the file calc" in the stored filesystem path and returns the
    /// result of ServiceA + ServiceB.
    pub fn calc(&self) -> i32 {
        self.fs.touch("calc");
        self.service_a.a() + self.service_b.b()
    }
}
