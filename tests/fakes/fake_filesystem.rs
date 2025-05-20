use downloader::filesystem::FileSystem;
use std::rc::Rc;
use std::cell::RefCell;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct FakeFileSystem {
    pub state: RefCell<Map<String, Value>>,
}

impl FileSystem for FakeFileSystem {
    fn touch(&self, path: &str) {
        self.state.borrow_mut().insert(path.to_string(), Value::Bool(true));
    }
}

pub fn empty_fs() -> Rc<FakeFileSystem> {
    Rc::new(FakeFileSystem { state: RefCell::new(Map::new()) })
}

impl PartialEq for FakeFileSystem {
    fn eq(&self, other: &Self) -> bool {
        self.state.borrow().eq(&other.state.borrow())
    }
}

pub fn fs_from_json(value: Value) -> Rc<FakeFileSystem> {
    if let Value::Object(state) = value {
        Rc::new(FakeFileSystem { state: RefCell::new(state) })
    } else {
        panic!("fs_from_json expects a JSON object");
    }
}
