mod problem;
use std::cmp::{max, min};
use std::collections::HashMap;
#[allow(unused)]
use std::collections::*;
use std::fmt::Debug;
use std::io;
use std::io::{BufRead, BufWriter, Write};
use std::str::FromStr;

pub fn input<T: std::str::FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("IO error");
    return inp.trim().parse::<T>().unwrap();
}

pub fn input_vec<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let v: Vec<T> = input::<String>() //inputing a string
        .split(' ') // spliting it by white space
        .map(|x| x.parse::<T>().expect("Nan")) //mapping each of them to the type T
        .collect(); // collecting the mapped vector
    v
}

fn main() {
    let mut a = vec![vec![1, 2, 3, 4, 5]; 4];
    println!("{:?}", a);
    let t = a[1].clone();
    a[0].extend(t);
    println!("{:?}", a);
}
