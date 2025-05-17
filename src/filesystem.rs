pub trait FileSystem {
    fn touch(&mut self, path: &str);
}

#[derive(Clone)]
pub struct RealFileSystem {}

impl RealFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for RealFileSystem {
    fn touch(&mut self, path: &str) {
        // TODO: Real implementation goes here
        println!("Touching file: {}", path);
    }
}
