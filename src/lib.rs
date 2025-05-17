#[unsafe(no_mangle)]
pub extern "C" fn downloader_init() -> i32 {
    0
}

pub fn the_42() -> i32 {
    42
}

pub mod delme;
pub mod filesystem;
