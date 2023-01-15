pub fn filter_values(first_string : &String, second_string : &String) -> String{
    let mut filtered_string : String = String::from("");
    let first_string_vec : Vec<u8> = first_string.clone().into_bytes();
    let second_string_vec : Vec<u8> = second_string.clone().into_bytes();
    let mut index : usize = 0;
    while index < first_string_vec.len() && index < second_string_vec.len(){
        if index == first_string_vec.len() || index == second_string_vec.len(){
            break;
        }
        let first_val : u8 = first_string_vec[index];
        let second_val : u8 = second_string_vec[index];
        if first_val == second_val{
            filtered_string.push(first_val as char);
        }else{
            return filtered_string;
        }
        index += 1;
    }
    filtered_string
}

// get a string that is common from the beggining between the members
// of str_vec
pub fn get_longest_common_prefix(str_vec : Vec<String>) -> String{
    let mut filtered_string : String = String::from("");
    let mut index : usize = 0;
    let end_index : usize = str_vec.len();
    while index < end_index{
        let current_val : &String = &str_vec[index];
        if index == 0{
            filtered_string = current_val.clone();
            index += 1;
        }else{
            filtered_string = filter_values(&filtered_string, current_val);
            if filtered_string.is_empty(){
                return filtered_string;
            }
            index += 1;
        }
    }
    filtered_string
}