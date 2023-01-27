pub fn square_root(x : i32) -> i32{
    if x == 0 || x == 1{
        return x;
    }
    let mut result : i32 = 0;
    for value in 0..x as i64{
        let y : i64 = value * value;
        if y > x as i64{
            return result;
        }
        if y == x as i64{
            return value as i32;
        }
        if y < x as i64{
            result = value as i32;
        }
    }
    result
}