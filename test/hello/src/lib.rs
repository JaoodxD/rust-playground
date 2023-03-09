use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

#[wasm_bindgen]
pub fn cosine_distance(vector1: &Float32Array, vector2: &Float32Array) -> f32 {
    let dot_product = dot_product(vector1, vector2);
    let magnitude1 = magnitude(vector1);
    let magnitude2 = magnitude(vector2);
    let denominator = magnitude1 * magnitude2;
    if denominator.abs() < f32::EPSILON {
        return 0.0;
    }
    return dot_product / denominator;
}

// Функция, которая вычисляет скалярное произведение двух векторов
fn dot_product(vector1: &Float32Array, vector2: &Float32Array) -> f32 {
    let mut result = 0.0;
    for i in 0..vector1.length() {
        result += vector1.get_index(i) * vector2.get_index(i);
    }
    return result;
}

// Функция, которая вычисляет длину вектора
fn magnitude(vector: &Float32Array) -> f32 {
    let mut result = 0.0;
    for i in 0..vector.length() {
        result += vector.get_index(i) * vector.get_index(i);
    }
    return f32::sqrt(result);
}
