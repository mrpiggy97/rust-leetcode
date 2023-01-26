pub fn plus_one(digits : Vec<i32>) -> Vec<i32>{
    let mut digits_clone : Vec<i32> = digits.clone();
    let end_index : usize = digits_clone.len();
    for current_index in (0..end_index).rev(){
        let mut current_val : i32 = digits_clone[current_index];
        current_val += 1;
        if current_val < 10{
            digits_clone[current_index] = current_val;
            return digits_clone
        }else{
            digits_clone[current_index] = 0;
        }
    }
    if digits_clone[0] == 0{
        digits_clone.insert(0, 1);
    }
    digits_clone
}