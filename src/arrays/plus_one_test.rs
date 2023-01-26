#[cfg(test)]

use crate::arrays::plus_one::plus_one;

#[test]
pub fn plus_one_test(){
    let vec_test : Vec<i32> = vec![7,3,7,9];
    let expected_result : Vec<i32> = vec![7,3,8,0];
    let result : Vec<i32> = plus_one(vec_test);
    assert_eq!(result, expected_result);
}