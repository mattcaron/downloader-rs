// This test suite just checks that the repository structure is working fine

use downloader::the_42;
use fakes::objects::file_a;

mod fakes;

#[test]
fn it_runs() {
    assert_eq!(true, true);
}

#[test]
fn it_uses_fakes() {
    assert_eq!(file_a, "a");
}

#[test]
fn it_uses_the_42() {
    assert_eq!(the_42(), 42);
}
