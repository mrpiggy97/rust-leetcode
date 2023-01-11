#[cfg(test)]

use super::number_is_palindrome::number_is_palindrome;

#[test]
pub fn test_number_is_palindrome(){
    let mut testing_number : i32 = 121;
    let mut result : bool = number_is_palindrome(testing_number);
    let mut expected_result : bool = true;
    assert_eq!(result,expected_result);
    testing_number = 1234;
    result = number_is_palindrome(testing_number);
    expected_result = false;
    assert_eq!(result,expected_result);
}