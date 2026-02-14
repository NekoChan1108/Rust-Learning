mod common;
use automated_test::test::add;

#[test]
fn integration_test() {
    common::utils::setup();
    common::core::core();
    assert_eq!(add(2, 2), 4);
    println!("Integration test passed");
}
