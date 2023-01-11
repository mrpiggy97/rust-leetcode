pub fn count_range_sum(nums : Vec<i32>, lower : i32, upper : i32) -> i32{
    let mut count : i32 = 0;
    for num in nums{
        if num >= lower && num <= upper{
            count += 1;
        }
    }
    count
}