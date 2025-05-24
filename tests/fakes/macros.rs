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

//! A collection of macros used for faking things for testing purposes

/// Create a fake filesystem, either empty or created per the supplied JSON.
#[macro_export]
macro_rules! fs {
    ($($json:tt)+) => {
        $crate::fakes::fake_filesystem::fs_from_json(serde_json::json!($($json)+))
    };

    () => {
        $crate::fakes::fake_filesystem::empty_fs()
    };
}

/// Macro to create boring boilerplate for impl testers.
///
/// This creates a struct called `tester` that takes `service` as an argument,
/// then creates boilerplate implementations of Deref and DerefMut. It also
/// creates an impl for `tester`, which implements `new` and `untester`
/// functions, as follows:
///
/// * `new` - creates a new copy of `tester` created with an argument
///   of `service`.
///
/// * `untester`- creates a new tester created with a clone of the service used
///   to create this tester.
///
/// # Arguments
///
/// * `tester` - the name of the tester to implement
/// * `service` - the service it tests
#[macro_export]
macro_rules! impl_tester {
    // $tester = the name of the wrapper type
    // $service = the inner service type
    ($tester:ident, $service:ident) => {
        impl_tester!($tester {}, $service);
    };

    ($tester:ident {
        $($field_name:ident : $field_type:ty),* $(,)?
    }, $service:ident) => {
        pub struct $tester {
            _0: ::std::rc::Rc<$service>,
            $($field_name : $field_type),*
        }

        impl ::std::ops::Deref for $tester {
            type Target = $service;
            fn deref(&self) -> &Self::Target {
                &*self._0
            }
        }

        impl ::std::ops::DerefMut for $tester {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::std::rc::Rc::get_mut(&mut self._0).unwrap_or_else(|| {
                    panic!("Cannot get mutable reference to Rc<{}>", stringify!($service))
                })
            }
        }

        impl $tester {
            pub fn new(s: $service, $($field_name: $field_type),*) -> ::std::rc::Rc<Self> {
                ::std::rc::Rc::new($tester { _0: ::std::rc::Rc::new(s), $($field_name),* })
            }
            pub fn clone_testee(&self) -> ::std::rc::Rc<$service> {
                self._0.clone()
            }
        }
    };
}

/// Create an AnyMap, storing the supplied data.
///
/// If no data is supplied, it creates an empty AnyMap.
#[macro_export]
macro_rules! any {
    // 0 elements → empty map
    () => {{
        ::anymap::AnyMap::new()
    }};

    // 1+ elements: first is plain `expr`
    ( $e:expr $(, $($rest:tt)*)? ) => {{
        let mut m = ::anymap::AnyMap::new();
        m.insert($e.clone());
        any!(@inner m $(, $($rest)*)?);
        m
    }};

    // internal: no more elements
    (@inner $m:ident) => {};

    // internal: recurse first element is plain `expr`
    (@inner $m:ident, $e:expr $(, $($rest:tt)*)? ) => {{
        $m.insert($e.clone());
        any!(@inner $m $(, $($rest)*)?);
    }};
}

pub trait AnyMapExt {
    /// Try to get `T` from the map (by `T: 'static`), `.cloned()`,
    /// otherwise call `default()` to build one.
    fn get_or_else<T, F>(&self, default: F) -> T
    where
        T: Clone + 'static,
        F: FnOnce() -> T;
}

impl AnyMapExt for ::anymap::AnyMap {
    fn get_or_else<T, F>(&self, default: F) -> T
    where
        T: Clone + 'static,
        F: FnOnce() -> T,
    {
        self.get::<T>().cloned().unwrap_or_else(default)
    }
}
