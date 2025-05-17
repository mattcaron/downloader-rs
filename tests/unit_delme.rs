// This is just a POC. It should be deleted after real test suites are added to the project.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

mod fakes;

use fakes::any;
use fakes::fake_delme::{service_a_with, service_b_with, service_c, service_c_with};
use fakes::fake_filesystem::fs;

#[test]
fn service_c_calc___when_instantiated_through_prod_structs_and_fake_fn___returns_expected_value() {
    // This example shows how not using a test complayer can lead to a lot of boilerplate code.
    // It is better to avoid that to focus in what the test is really testing.

    use downloader::delme::{ServiceA, ServiceB, ServiceC};
    use std::cell::RefCell;
    use std::rc::Rc;

    let the_fs = fs();
    let the_service_a = Rc::new(RefCell::new(ServiceA::new(the_fs.clone(), 1)));
    let the_service_b = Rc::new(RefCell::new(ServiceB::new(the_fs.clone(), 2)));
    let mut the_service_c = ServiceC::new(the_fs.clone(), the_service_a, the_service_b);
    assert_eq!(the_service_c.calc(), 3);
}

#[test]
fn service_c_calc___when_instantiated_through_test_complayer_default_fn___returns_expected_value() {
    // This one uses the default instances from the complayer
    assert_eq!(service_c().calc(), 3);
}

#[test]
fn service_c_calc___when_instantiated_through_test_complayer_fn_with_custom_values_on_the_fly___returns_a_different_value()
 {
    // This one allows customization while keeping it succinct so that we can focus on the test.
    // This is the furthest I got to remove noise, but if it can be improved, please do so.
    assert_ne!(
        service_c_with(any!(rc service_a_with(any! { 42 }), fs())).calc(),
        3
    );
}

#[test]
fn service_c_calc___when_instantiated_through_a_very_common_scenario_fn___returns_a_very_common_scenario_value()
 {
    assert_eq!(
        service_c_under_a_very_common_scenario().calc(),
        very_common_scenario_value
    );
}

const very_common_scenario_value: i32 = 84;
fn service_c_under_a_very_common_scenario() -> downloader::delme::ServiceC {
    // This could be put in another module if it is used in different test suites
    service_c_with(any!(rc service_a_with(any! { 42 }), rc service_b_with(any! { 42 })))
}
