use std::rc::Rc;
use anymap::AnyMap;
use downloader::delme::{ServiceA, ServiceB, ServiceC};
use crate::fakes::macros::AnyMapExt;
use crate::{any, fs, impl_tester};

impl_tester!(ServiceATester, ServiceA);
impl ServiceATester {
    pub fn call_a_many_times(&self) {
        for _ in 0..10 {
            self.0.a();
        }
    }
}

pub fn service_a() -> Rc<ServiceATester> {
    service_a_(any!())
}
pub fn service_a_(x: AnyMap) -> Rc<ServiceATester> {
    ServiceATester::new(ServiceA::new(x.get_or_else(|| fs!()), x.get_or_else(|| 1)))
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
        x.get_or_else(|| service_a_(any!(the_fs))).untester(),
        x.get_or_else(|| service_b_(any!(the_fs))),
    ))
}
