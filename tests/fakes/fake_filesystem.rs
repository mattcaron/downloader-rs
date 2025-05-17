use downloader::filesystem::FileSystem;
use std::cell::RefCell;
use std::collections::HashMap as Map;
use std::rc::Rc;

struct FakeFileSystem {
    state: Map<String, bool>,
}

impl FileSystem for FakeFileSystem {
    fn touch(&mut self, path: &str) {
        self.state.insert(path.to_string(), true);
    }
}

pub fn fs() -> Rc<RefCell<dyn FileSystem>> {
    Rc::new(RefCell::new(FakeFileSystem { state: Map::new() }))
}
