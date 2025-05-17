use crate::fakes::any::AnyMapExt;
use crate::fakes::fake_filesystem::fs;
use crate::{any, rc};
use anymap::AnyMap;
use downloader::delme::{ServiceA, ServiceB, ServiceC};

pub fn service_a() -> ServiceA {
    service_a_with(any!())
}
pub fn service_a_with(x: AnyMap) -> ServiceA {
    ServiceA::new(x.get_or_else(|| fs()), x.get_or_else(|| 1))
}

pub fn service_b() -> ServiceB {
    service_b_with(any!())
}
pub fn service_b_with(x: AnyMap) -> ServiceB {
    ServiceB::new(x.get_or_else(|| fs()), x.get_or_else(|| 2))
}

pub fn service_c() -> ServiceC {
    service_c_with(any!())
}
pub fn service_c_with(x: AnyMap) -> ServiceC {
    ServiceC::new(
        x.get_or_else(|| fs()),
        x.get_or_else(|| rc!(service_a())),
        x.get_or_else(|| rc!(service_b())),
    )
}
