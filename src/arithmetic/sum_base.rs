pub fn sum_digits(num : i32) -> i32{
    let num_string : String = format!("{}", num);
    let mut total : i32 = 0;
    for digit_str in num_string.split(""){
        if !digit_str.is_empty(){
            let digit : i32 = digit_str.parse().unwrap();
            total += digit;
        }
    }
    total
}

pub fn get_number_with_new_base(number : i32, base : i32) -> i32{
    let mut current_number : i32 = number;
    let mut str_number : String = String::from("");
    while current_number >= base{
        let current_remainder : i32 = current_number % base;
        current_number /= base;
        let remainder_as_string : String= format!("{}", current_remainder);
        str_number = format!("{}{}",remainder_as_string, str_number);
    }
    str_number = format!("{}{}",current_number,str_number);
    let converted_number : i32 = str_number.parse().unwrap();
    converted_number
}

pub fn sum_base(num : i32, base : i32) -> i32{
    let converted_number : i32 = get_number_with_new_base(num, base);
    let sumed_digits : i32 = sum_digits(converted_number);
    sumed_digits
}