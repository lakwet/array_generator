use rand::{thread_rng, Rng};

use std::env;

pub fn helper_random_array_uniform_u64(size: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    let mut array: Vec<u64> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: u64 = rng.gen();
        array.push(value);
    }
    array
}

// Uniform 10^9
pub fn helper_random_array_uniform_10_9_u64(size: usize) -> Vec<u64> {
    let mut rng = thread_rng();
    let mut array: Vec<u64> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: u64 = rng.gen_range(0, 1_000_000_000);
        array.push(value);
    }
    array
}

// Uniform
pub fn helper_random_array_uniform_u32(size: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut array: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: u32 = rng.gen();
        array.push(value);
    }
    array
}

// Uniform 10^9
pub fn helper_random_array_uniform_10_9_u32(size: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut array: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: u32 = rng.gen_range(0, 1_000_000_000);
        array.push(value);
    }
    array
}

// Uniform
pub fn helper_random_array_uniform_i32(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut array: Vec<i32> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: i32 = rng.gen();
        array.push(value);
    }
    array
}

// 10^9 values
pub fn helper_random_array_109_i32(size: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut array: Vec<i32> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: i32 = rng.gen_range(-1_000_000_000, 1_000_000_000);
        array.push(value);
    }
    array
}

// Uniform
pub fn helper_random_array_uniform_i64(size: usize) -> Vec<i64> {
    let mut rng = thread_rng();
    let mut array: Vec<i64> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: i64 = rng.gen();
        array.push(value);
    }
    array
}

// Small values
pub fn helper_random_array_109_i64(size: usize) -> Vec<i64> {
    let mut rng = thread_rng();
    let mut array: Vec<i64> = Vec::with_capacity(size);
    for _ in 0..size {
        let value: i64 = rng.gen_range(-1_000_000_000, 1_000_000_000);
        array.push(value);
    }
    array
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let n = args[4].parse::<usize>().unwrap();
    if args[1] == "--type".to_string() {
        match args[2].as_str() {
            "u32" => {
                let arr = helper_random_array_uniform_u32(n);
                let file_path = format!("./numbers_u32_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();

                let arr = helper_random_array_uniform_10_9_u32(n);
                let file_path = format!("./numbers_u32_109_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();
            },
            "u64" => {
                let arr = helper_random_array_uniform_u64(n);
                let file_path = format!("./numbers_u64_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();

                let arr = helper_random_array_uniform_10_9_u64(n);
                let file_path = format!("./numbers_u64_109_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();
            },
            "i32" => {
                let arr = helper_random_array_uniform_i32(n);
                let file_path = format!("./numbers_i32_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();

                let arr = helper_random_array_109_i32(n);
                let file_path = format!("./numbers_i32_109_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();
            },
            "i64" => {
                let arr = helper_random_array_uniform_i64(n);
                let file_path = format!("./numbers_i64_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();

                let arr = helper_random_array_109_i64(n);
                let file_path = format!("./numbers_i64_109_{}", n);
                let mut arr_string = arr.into_iter().map(|elt| elt.to_string()).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceInt".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();
            },
            "tuple_u32" => {
                let arr1 = helper_random_array_uniform_u32(n);
                let arr2 = helper_random_array_uniform_u32(n);
                let file_path = format!("./numbers_u32u32_{}", n);
                let mut arr_string = arr1
                    .into_iter()
                    .zip(arr2.into_iter())
                    .map(|(i1, i2)| format!("{} {}", i1, i2)).collect::<Vec<String>>();
                arr_string.insert(0, "sequenceIntPair".to_string());
                let output_string = arr_string.join("\n");
                std::fs::write(file_path, output_string).unwrap();
            },
            _ => {},
        }
    }
}
