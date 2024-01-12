// #![feature(portable_simd)]
// use core::simd::prelude::*;

fn main() {
    let vec_of_vec3s = vec![
        vec![1.1, 2.2, 3.3],
        vec![4.4, 5.5, 6.6],
        vec![7.7, 8.8, 9.9]
    ];

    for vec3 in vec_of_vec3s {
        for i in vec3 {
            println!("{}", i);
        }
    }

    println!("Hello, world!");
}
