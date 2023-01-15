#[cfg(test)]

use crate::strings::longest_common_prefix::filter_values;
use crate::strings::longest_common_prefix::get_longest_common_prefix;

#[test]
pub fn test_filter_values(){
    let mut first_string : String = String::from("flow");
    let mut second_string : String = String::from("fla");
    let mut expected_result : String = String::from("fl");
    let mut result : String = filter_values(&first_string, &second_string);
    assert_eq!(result,expected_result);

    first_string = String::from("care");
    second_string = String::from("dude");
    expected_result = String::from("");
    result = filter_values(&first_string, &second_string);
    assert_eq!(result,expected_result);
}

#[test]
pub fn test_get_longest_common_prefix(){
    let mut test_strings : Vec<String> = vec![String::from("fla"), String::from("flow"), String::from("fli")];
    let mut expected_result : String = String::from("fl");
    let mut result : String = get_longest_common_prefix(test_strings);
    assert_eq!(result,expected_result);
    test_strings = vec![String::from("fla"), String::from("floricae"), String::from("cart")];
    expected_result = String::from("");
    result = get_longest_common_prefix(test_strings);
    assert_eq!(result,expected_result);
}