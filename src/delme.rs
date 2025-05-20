// This is just a POC. It should be deleted after real components with their respective test suites are added to the project.

use std::rc::Rc;

use crate::filesystem::*;

#[derive(Clone)]
pub struct ServiceA {
    fs: Rc<dyn FileSystem>,
    a: i32,
}

impl ServiceA {
    pub fn new(fs: Rc<dyn FileSystem>, a: i32) -> Self {
        Self { fs, a }
    }
    pub fn a(&self) -> i32 {
        self.fs.touch("a");
        self.a
    }
}

pub struct ServiceB {
    fs: Rc<dyn FileSystem>,
    b: i32,
}

impl ServiceB {
    pub fn new(fs: Rc<dyn FileSystem>, b: i32) -> Self {
        Self { fs, b }
    }
    pub fn b(&self) -> i32 {
        self.fs.touch("b");
        self.b
    }
}

pub struct ServiceC {
    fs: Rc<dyn FileSystem>,
    service_a: Rc<ServiceA>,
    service_b: Rc<ServiceB>,
}

impl ServiceC {
    pub fn new(fs: Rc<dyn FileSystem>, service_a: Rc<ServiceA>, service_b: Rc<ServiceB>) -> Self {
        Self {
            fs,
            service_a,
            service_b,
        }
    }

    pub fn calc(&self) -> i32 {
        self.fs.touch("calc");
        self.service_a.a() + self.service_b.b()
    }
}
