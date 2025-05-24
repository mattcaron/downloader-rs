use crate::fakes::macros::AnyMapExt;
use crate::{any, fs, impl_tester};
use anymap::AnyMap;
use downloader::delme::{ServiceA, ServiceB, ServiceC};
use std::cell::RefCell;
use std::rc::Rc;

// Not always you want to wrap the service in a tester, but does it easily.
impl_tester!(ServiceATester { z: RefCell<i32> }, ServiceA);
impl ServiceATester {
    pub fn call_a_many_times(&self) {
        for _ in 0..10 {
            self.a();
            *self.z.borrow_mut() += 1;
        }
    }
    /* You may even override the method from the original service like this:
    pub fn a(&self) -> i32 {
        self._0.a() + *self.z.borrow()
    } */
}

pub fn service_a() -> Rc<ServiceATester> {
    service_a_(any!())
}
pub fn service_a_(x: AnyMap) -> Rc<ServiceATester> {
    ServiceATester::new(
        ServiceA::new(x.get_or_else(|| fs!()), x.get_or_else(|| 1)),
        0.into(),
    )
}

pub fn service_b() -> Rc<ServiceB> {
    service_b_(any!())
}
pub fn service_b_(x: AnyMap) -> Rc<ServiceB> {
    Rc::new(ServiceB::new(x.get_or_else(|| fs!()), x.get_or_else(|| 2)))
}

pub fn service_c() -> Rc<ServiceC> {
    service_c_(any!())
}
pub fn service_c_(x: AnyMap) -> Rc<ServiceC> {
    let the_fs = x.get_or_else(|| fs!());
    Rc::new(ServiceC::new(
        the_fs.clone(),
        x.get_or_else(|| service_a_(any!(the_fs))).clone_testee(),
        x.get_or_else(|| service_b_(any!(the_fs))),
    ))
}
