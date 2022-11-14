#[cfg(test)]

use super::sum_base;

#[test]
pub fn test_sum_digits(){
    let num : i32 = 23;
    let result : i32 = sum_base::sum_digits(num);
    let mut expected_result : i32 = 5;
    assert_eq!(result, expected_result);
    let num2 : i32 = 57;
    let result2 : i32 = sum_base::sum_digits(num2);
    expected_result = 12;
    assert_eq!(result2,expected_result);
}

#[test]
pub fn test_get_number_with_new_base(){
    let mut num : i32 = 8;
    let mut result : i32 = sum_base::get_number_with_new_base(num, 3);
    let mut expected_result : i32 = 22;
    assert_eq!(result,expected_result);

    num = 9;
    result = sum_base::get_number_with_new_base(num, 3);
    expected_result = 100;
    assert_eq!(result,expected_result);

    num = 10;
    result = sum_base::get_number_with_new_base(num, 3);
    expected_result = 101;
    assert_eq!(result, expected_result);

    num = 23;
    result = sum_base::get_number_with_new_base(num, 3);
    expected_result = 212;
    assert_eq!(result, expected_result);

    num = 26;
    result = sum_base::get_number_with_new_base(num, 3);
    expected_result = 222;
    assert_eq!(result, expected_result);

    num = 27;
    result = sum_base::get_number_with_new_base(num, 3);
    expected_result = 1000;
    assert_eq!(result, expected_result);

    num = 6;
    result = sum_base::get_number_with_new_base(num, 6);
    expected_result = 10;
    assert_eq!(result, expected_result);

    num = 15;
    expected_result = 23;
    result = sum_base::get_number_with_new_base(num, 6);
    assert_eq!(result, expected_result);

    num = 42;
    expected_result = 110;
    result = sum_base::get_number_with_new_base(num, 6);
    assert_eq!(result, expected_result);

    num = 10;
    expected_result = 10;
    result = sum_base::get_number_with_new_base(num, 10);
    assert_eq!(result, expected_result);

    num = 8;
    result = sum_base::get_number_with_new_base(num, 2);
    expected_result = 1000;
    assert_eq!(result, expected_result);
}

#[test]
pub fn test_sum_base(){
    let num : i32 = 42;
    let result : i32 = sum_base::sum_base(num, 6);
    let expected_result : i32 = 2;
    assert_eq!(result, expected_result);
}