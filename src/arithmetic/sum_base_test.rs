#[cfg(test)]

use super::sum_base;
#[test]
pub fn test_sum_base(){
    let mut num : i32 = 8;
    let mut result : i32 = sum_base::sum_base(num, 3);
    let mut expected_result : i32 = 4;
    assert_eq!(result,expected_result);

    num = 9;
    result = sum_base::sum_base(num, 3);
    expected_result = 1;
    assert_eq!(result,expected_result);

    num = 10;
    result = sum_base::sum_base(num, 3);
    expected_result = 2;
    assert_eq!(result, expected_result);

    num = 23;
    result = sum_base::sum_base(num, 3);
    expected_result = 5;
    assert_eq!(result, expected_result);

    num = 26;
    result = sum_base::sum_base(num, 3);
    expected_result = 6;
    assert_eq!(result, expected_result);

    num = 27;
    result = sum_base::sum_base(num, 3);
    expected_result = 1;
    assert_eq!(result, expected_result);

    num = 6;
    result = sum_base::sum_base(num, 6);
    expected_result = 1;
    assert_eq!(result, expected_result);

    num = 15;
    expected_result = 5;
    result = sum_base::sum_base(num, 6);
    assert_eq!(result, expected_result);

    num = 42;
    expected_result = 2;
    result = sum_base::sum_base(num, 6);
    assert_eq!(result, expected_result);

    num = 10;
    expected_result = 1;
    result = sum_base::sum_base(num, 10);
    assert_eq!(result, expected_result);

    num = 8;
    result = sum_base::sum_base(num, 2);
    expected_result = 1;
    assert_eq!(result, expected_result);
}