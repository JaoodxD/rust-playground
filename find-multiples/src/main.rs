fn main() {
    println!("Hello, world!");
    println!("find_multiples: {:?}", find_multiples(2, 5));
    println!("find_multiples2: {:?}", find_multiples2(3, 20));
    println!("find_multiples3: {:?}", find_multiples3(4, 25));

}
fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for i in n..=limit {
        if i % n == 0 {
            vec.push(i);
        }
    }
    vec
}
fn find_multiples2(n: u32, limit: u32) -> Vec<u32> {
    (n..=limit)
    .step_by(n as usize)
    .collect()
}
fn find_multiples3(n: u32, limit: u32) -> Vec<u32> {
    (1..=limit/n)
    .map(|x| x * n)
    .collect()
}

