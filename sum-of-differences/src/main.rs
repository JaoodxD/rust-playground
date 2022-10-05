fn main() {
    println!("1:{:?}", sum_of_differences(&[1, 2, 10]));
    println!("2:{:?}", sum_of_differences2(&[1, 2, 10]));
    println!("1:{:?}", sum_of_differences(&[]));
    println!("2:{:?}", sum_of_differences2(&[]));
    println!("1:{:?}", sum_of_differences(&[-17, 17]));
    println!("2:{:?}", sum_of_differences2(&[-17, 17]));
}
fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() > 1 {
        let max = arr.iter().max().unwrap();
        let min = arr.iter().min().unwrap();
        return Some(max - min);
    }
    None
}
fn sum_of_differences2(arr: &[i8]) -> Option<i8> {
    if arr.len() < 2 {
        return None;
    }
    let mut v = arr.to_vec();
    v.sort_unstable();
    Some(v.windows(2).map(|w| w[1] - w[0]).sum())
}
