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

#[macro_export]
macro_rules! fs {
    ($($json:tt)+) => {
        $crate::fakes::fake_filesystem::fs_from_json(serde_json::json!($($json)+))
    };

    () => {
        $crate::fakes::fake_filesystem::empty_fs()
    };
}

#[macro_export]
macro_rules! impl_tester {
    // $tester = the name of the wrapper type
    // $service = the inner service type
    ($tester:ident, $service:ident) => {
        // 1) new type
        pub struct $tester(pub $service);

        // 2) Deref -> &Service
        impl ::std::ops::Deref for $tester {
            type Target = $service;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // 3) DerefMut -> &mut Service
        impl ::std::ops::DerefMut for $tester {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        // 4) impl ctor + “untester”
        impl $tester {
            /// Create a reference‐counted tester
            pub fn new(service: $service) -> ::std::rc::Rc<Self> {
                ::std::rc::Rc::new($tester(service))
            }

            /// Extract the inner `Service` as a fresh `Rc<Service>`
            pub fn untester(&self) -> ::std::rc::Rc<$service>
            where
                $service: Clone,
            {
                ::std::rc::Rc::new(self.0.clone())
            }
        }
    };
}
