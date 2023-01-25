pub fn search_insert(nums : Vec<i32>, target : i32) -> i32{
    let mut position : i32 = 0;
    let end_index : usize = nums.len();
    for index in 0..end_index{
        let current_val : i32 = nums[index];
        if current_val == target{
            return position;
        }
        if current_val > target{
            return position;
        }else{
            position += 1;
        }
    }
    position
}