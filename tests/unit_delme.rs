// This is just a POC. It should be deleted after real test suites are added to the project.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

mod fakes;

use fakes::fake_delme::*;
use fakes::objects::*;

#[test]
fn service_c_calc___when_instantiated_through_prod_structs_and_fake_fn___returns_expected_value() {
    // This example shows how not using a test complayer can lead to a lot of boilerplate code.
    // It is better to avoid that to focus in what the test is really testing.

    use downloader::delme::{ServiceA, ServiceB, ServiceC};
    use std::rc::Rc;

    let the_fs = fs!();
    let the_service_a = Rc::new(ServiceA::new(the_fs.clone(), 1));
    let the_service_b = Rc::new(ServiceB::new(the_fs.clone(), 2));
    let the_service_c = ServiceC::new(the_fs.clone(), the_service_a.clone(), the_service_b.clone());
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
    assert_ne!(service_c_(any!(service_a_(any! { 42 }), fs!())).calc(), 3);
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
fn service_c_under_a_very_common_scenario() -> std::rc::Rc<downloader::delme::ServiceC> {
    // This could be put in another module if it is used in different test suites
    service_c_(any!(service_a_(any! { 42 }), service_b_(any! { 42 })))
}

#[test]
fn service_c_calc___when_fs_was_empty___produces_fs_with_file_a_b_and_calc() {
    // This is an example of a data oriented broad unit test:
    // 1. The method calc is called, and that has a side effect on the fs.
    // 2. The test checks that the fs has the expected state after the call.
    // This is how most of the tests are written in the 'ref_tests' directory
    // and they should be adapted to rust following the same semantics.

    // Arrange:
    // We need here the part of the state that will participate in the assertion.
    let the_fs = fs!(); // This is the default empty fs

    // Act:
    // Executes the system under test "service_c.calc()" to change the fs.
    service_c_(any!(the_fs)).calc();

    // Assert:
    // The FS now contains just the expected files
    assert_eq!(the_fs, fs!({file_a: true, file_b: true, file_calc: true}));
}

#[test]
fn service_a___when_called_many_times___does_not_panic() {
    // This is just to demonstrate that having a tester allows you to call
    // methods from a service that are not part of production code.

    // If all is well structured, converting a service to a tester should only
    // require code changes in the compatibility test layer (fake module), while
    // keeping the test suites and/or prod code intact.

    // Testers are useful to to reduce boilerplate in test suites, by adding
    // an interface that reduces noise in the test suites.

    // This particular example is silly, but real testers are used well in
    // the fake_online_importer.py from the MiSTer Downloader repository.

    let a = service_a();
    a.call_a_many_times();
    assert_eq!(a.a(), 1);
}
