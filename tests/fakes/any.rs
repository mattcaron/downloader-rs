#[macro_export]
macro_rules! any {
    // 0 elements → empty map
    () => {{
        ::anymap::AnyMap::new()
    }};

    // 1+ elements: first is `rc expr`
    ( rc $e:expr $(, $($rest:tt)*)? ) => {{
        let mut m = ::anymap::AnyMap::new();
        m.insert(std::rc::Rc::new(std::cell::RefCell::new($e)));
        // recurse on the rest if any
        any!(@inner m $(, $($rest)*)?);
        m
    }};

    // 1+ elements: first is plain `expr`
    ( $e:expr $(, $($rest:tt)*)? ) => {{
        let mut m = ::anymap::AnyMap::new();
        m.insert($e);
        any!(@inner m $(, $($rest)*)?);
        m
    }};

    // internal: no more elements
    (@inner $m:ident) => {};

    // internal: recurse first element is `rc expr`
    (@inner $m:ident, rc $e:expr $(, $($rest:tt)*)? ) => {{
        $m.insert(std::rc::Rc::new(std::cell::RefCell::new($e)));
        any!(@inner $m $(, $($rest)*)?);
    }};

    // internal: recurse first element is plain `expr`
    (@inner $m:ident, $e:expr $(, $($rest:tt)*)? ) => {{
        $m.insert($e);
        any!(@inner $m $(, $($rest)*)?);
    }};
}

#[macro_export]
macro_rules! rc {
    // Default: Rc<RefCell<T>>
    ($expr:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($expr))
    };

    // Arc<Mutex<T>>
    (arc mutex $expr:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($expr))
    };

    // Arc<RefCell<T>> (uncommon, but possible)
    (arc ref $expr:expr) => {
        std::sync::Arc::new(std::cell::RefCell::new($expr))
    };

    // Rc<Mutex<T>> (uncommon)
    (rc mutex $expr:expr) => {
        std::rc::Rc::new(std::sync::Mutex::new($expr))
    };
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
