#[cfg(test)]
use crate::arrays::search_insert::search_insert;

#[test]
pub fn test_search_insert(){
    let test_vec : Vec<i32> = vec![1,2,3,4,6];
    let mut expected_result : i32 = 4;
    let mut result : i32 = search_insert(test_vec, 5);
    assert_eq!(expected_result, result);

    let second_test_vec : Vec<i32> = vec![1,2,3,4,5];
    expected_result = 4;
    result = search_insert(second_test_vec, 5);
    assert_eq!(result, expected_result);
}