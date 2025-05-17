// This is just a POC. It should be deleted after real components with their respective test suites are added to the project.

use std::cell::RefCell;
use std::rc::Rc;

use crate::filesystem::*;

#[derive(Clone)]
pub struct ServiceA {
    fs: Rc<RefCell<dyn FileSystem>>,
    a: i32,
}

impl ServiceA {
    pub fn new(fs: Rc<RefCell<dyn FileSystem>>, a: i32) -> Self {
        Self { fs, a }
    }
    pub fn a(&mut self) -> i32 {
        self.fs.borrow_mut().touch("a");
        self.a
    }
}

#[derive(Clone)]
pub struct ServiceB {
    fs: Rc<RefCell<dyn FileSystem>>,
    b: i32,
}

impl ServiceB {
    pub fn new(fs: Rc<RefCell<dyn FileSystem>>, b: i32) -> Self {
        Self { fs, b }
    }
    pub fn b(&mut self) -> i32 {
        self.fs.borrow_mut().touch("b");
        self.b
    }
}

#[derive(Clone)]
pub struct ServiceC {
    fs: Rc<RefCell<dyn FileSystem>>,
    service_a: Rc<RefCell<ServiceA>>,
    service_b: Rc<RefCell<ServiceB>>,
}

impl ServiceC {
    pub fn new(
        fs: Rc<RefCell<dyn FileSystem>>,
        service_a: Rc<RefCell<ServiceA>>,
        service_b: Rc<RefCell<ServiceB>>,
    ) -> Self {
        Self {
            fs,
            service_a,
            service_b,
        }
    }

    pub fn calc(&mut self) -> i32 {
        self.fs.borrow_mut().touch("calc");
        self.service_a.borrow().a + self.service_b.borrow().b
    }
}
