fn main() {
    let res = cosine_distance(&[13.,23.,44.],&[13.,22.,43.]);
    println!("{}", res);
}

fn cosine_distance(v1: &[f64], v2: &[f64]) -> f64 {
    let dot_product = v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum::<f64>();
    let v1_norm = v1.iter().map(|&a| a * a).sum::<f64>().sqrt();
    let v2_norm = v2.iter().map(|&a| a * a).sum::<f64>().sqrt();
    let denominator = v1_norm * v2_norm;
    if denominator == 0.0 {
        return 0.0;
    }
    dot_product / denominator
}
