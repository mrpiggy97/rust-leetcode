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
    let mut total : i32 = 0;
    let num_as_string : String = format!("{}", num);
    for num_str in num_as_string.split(""){
        let num_as_i32 : i32 = num_str.parse().unwrap_or_default();
        total += num_as_i32;
    }
    total
}

pub fn get_number_with_new_base(number : i32, base : i32) -> i32{
    let mut spaces_to_move : i32 = 10-base;
    let end_number : i32 = number+1;
    let mut index : i32 = 1;
    let mut current_number : i32 = 0;
    let mut number_before_decimal_jump : i32 = get_number_before_decimal_jump(base, index);
    let mut count : i32 = 0;
    // get the number with new base
    for num in base..end_number{
        println!("{}", number_before_decimal_jump);
        if num == number_before_decimal_jump{
            count = 0;
            let decimal_jump : i32 = get_decimal_jump(base, index);
            spaces_to_move = decimal_jump;
            current_number = num + spaces_to_move;
            index += 1;
            number_before_decimal_jump = get_number_before_decimal_jump(base, index);
        }else{
            if count == base{
                spaces_to_move += 7;
                count = 0;
            }
            current_number = num + spaces_to_move;
            count += 1;
        }
    }
    current_number
}