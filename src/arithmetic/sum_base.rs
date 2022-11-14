pub fn sum_base(number : i32, base : i32) -> i32{
    let mut current_number : i32 = number;
    let mut total : i32 = 0;
    while current_number >= base{
        let current_remainder : i32 = current_number % base;
        current_number /= base;
        total += current_remainder;
    }
    total += current_number;
    total
}