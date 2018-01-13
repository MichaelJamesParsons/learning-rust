use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32,f64};
use std::io::stdin;

fn main() {
    println!("Max i8 {}", i8::MAX);
    println!("Min i8 {}", i8::MIN);
    println!("Max i16 {}", i16::MAX);
    println!("Min i16 {}", i16::MIN);
    println!("Max i32 {}", i32::MAX);
    println!("Min i32 {}", i32::MIN);
    println!("Max i64 {}", i64::MAX);
    println!("Min i64 {}", i64::MIN);
    println!("Max u8 {}", u8::MAX);
    println!("Max u8 {}", u8::MIN);
    println!("Max u16 {}", u8::MAX);
    println!("Max u16 {}", u8::MIN);
    println!("Max u32 {}", u8::MAX);
    println!("Max u32 {}", u8::MIN);
    println!("Max u64 {}", u64::MIN);
    println!("Max u64 {}", u64::MIN);
    println!("Max isize {}", isize::MAX);
    println!("Max isize {}", isize::MIN);
    println!("Max usize {}", usize::MAX);
    println!("Max usize {}", usize::MIN);

    let x: i8 = 120;
    println!("x: {}", x);

    let overflow: i8 = 300;
    println!("i8 overflow: {}", overflow)
}