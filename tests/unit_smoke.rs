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

//! Smoke test
//!
//! This test suite just checks that the repository structure is working fine.

#![allow(dead_code)]

use downloader::the_42;
use fakes::objects::file_calc;

mod fakes;

#[test]
fn it_runs() {
    assert_eq!(true, true);
}

#[test]
fn it_uses_fakes() {
    assert_eq!(file_calc, "calc");
}

#[test]
fn it_uses_the_42() {
    assert_eq!(the_42(), 42);
}
