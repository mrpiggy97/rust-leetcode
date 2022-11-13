use std::fmt::format;

// whenever we have to add a zero we make a jump in spaces moved
// this function returns that new value
pub fn get_decimal_jump(base : i32, index : i32) -> i32{
    let decimal_jump : i32 = ((10+base)*(10-base))*index;
    decimal_jump
}

pub fn get_number_before_decimal_jump(base : i32, index : i32) -> i32{
    let number_before_jump : i32 = (base*base) * index;
    number_before_jump
}

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
    let mut spaces_to_move : i32 = 10-base;
    let mut current_number : i32 = base;
    let mut converted_number : i32 = 0;
    let mut index : i32 = 1;
    let mut starting_number : i32 = base;
    while current_number <= number{
        let final_number : i32 = get_number_before_decimal_jump(base, index);
        let decimal_jump : i32 = get_decimal_jump(base, index); 
        index += 1;
        let mut count : i32 = 0;
        for num in starting_number..final_number{
            if count == base{
                count = 0;
                spaces_to_move += 10-base;
            }
            converted_number = num + spaces_to_move;
            count += 1;
            current_number = num;
            if current_number == number{
                return converted_number;
            }
        }
        starting_number = final_number;
        spaces_to_move = decimal_jump;
    }
    converted_number
}

pub fn sum_base(num : i32, base : i32) -> i32{
    let converted_number : i32 = get_number_with_new_base(num, base);
    let sumed_digits : i32 = sum_digits(converted_number);
    sumed_digits
}