pub fn number_is_palindrome(number : i32) -> bool{
    let number_str : String = number.to_string();
    let number_vec : Vec<u8> = number_str.into_bytes();
    let mut beggining_index : usize = 0;
    let mut end_index : usize = number_vec.len()-1;
    while beggining_index < end_index{
        let current_left : u8 = number_vec[beggining_index];
        let current_right : u8 = number_vec[end_index];
        if current_left != current_right{
            return false;
        }else{
            beggining_index += 1;
            end_index -= 1;
        }
    }
    true
}