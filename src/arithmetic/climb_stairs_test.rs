#[cfg(test)]
use crate::arithmetic::climb_stairs::climb_stairs;

#[test]
pub fn test_climb_stairs(){
    let mut n : i32 = 5;
    let mut expected_result : i32 = 8;
    let mut result : i32 = climb_stairs(n);
    assert_eq!(result, expected_result);

    n = 4;
    expected_result = 5;
    result = climb_stairs(n);
    assert_eq!(result, expected_result);

    n = 7;
    expected_result = 21;
    result = climb_stairs(n);
    assert_eq!(result, expected_result);

    n = 6;
    expected_result = 13;
    result = climb_stairs(n);
    assert_eq!(result, expected_result);
}