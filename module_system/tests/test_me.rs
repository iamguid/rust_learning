use crate::module_a;

#[test]
fn test_test() {
    assert_eq!(module_a::a::test_me(1, 2), 3);
}