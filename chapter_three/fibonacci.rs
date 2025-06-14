fn main() {
    let pos = 40;
    let nth: i64 = get_nth_fibonacci(pos);
    println!("the {pos}th fibonnacci number is {nth}")
}

fn get_nth_fibonacci(pos: i64) -> i64 {
    if pos == 0 {
        0
    } else if pos == 1 {
        1
    } else {
        get_nth_fibonacci(pos - 1) + get_nth_fibonacci(pos - 2)
    }
}
