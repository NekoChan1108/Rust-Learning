#[test]
// #[ignore]
fn test_add() {
    assert_eq!(add(1, 2), 2, "1+2 should equals to 3 not {}", 2);
}
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 3 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn panic_add(a: i32, b: i32) -> i32 {
    if a >= 100 || b >= 100 {
        panic!("panic_add a or b can't be greater than 100")
    }
    a + b
}