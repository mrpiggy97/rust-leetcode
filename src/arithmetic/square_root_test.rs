#[cfg(test)]
use crate::arithmetic::square_root::square_root;

#[test]
pub fn test_square_root(){
    let x : i32 = 2147483647;
    let expected_result : i32 = 46340;
    let result : i32 = square_root(x);
    assert_eq!(result, expected_result);
}