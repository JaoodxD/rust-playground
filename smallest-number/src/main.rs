fn main() {
    println!("Hello, world!");
    let arr = &[2, 3, 1, 0, -1];
    println!("{}", find_smallest_int(arr));
    println!("{}", find_smallest_int2(arr));

}
fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}
fn find_smallest_int2(arr: &[i32]) -> i32 {
    let mut min: i32 = arr[0];
    for n in arr {
        if *n < min {
            min = *n;
        }
    }
    min
}
