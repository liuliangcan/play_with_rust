mod problem;
use std::cmp::{max, min};
#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
use std::fmt::Debug;
use std::io;
use std::str::FromStr;
use std::collections::HashMap;

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

fn eval(v: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut last = 0;
    let mut ans = vec![];
    for (l, r) in v {
        if l <= &last { continue; }
        ans.push((*l, *r));
        last = *r;
    }
    ans
}

fn main(){
    let n: usize = input();
    let a: Vec<i32> = input_vec();
    let mut mp = HashMap::<i32, Vec<(usize, usize)>>::new();
    let pref: Vec<i32> = a.iter().scan(0, |state, x| { *state += x; return Some(*state); }).collect();
    for i in 0..n {
        for j in i..n {
            let mut s = pref[j];
            if i > 0 { s -= pref[i - 1]; }
            if !mp.contains_key(&s) {
                mp.insert(s, Vec::new());
            }
            mp.get_mut(&s).unwrap().push((i + 1, j + 1));
        }
    }

    mp.values_mut()
        .map(|x| x.sort_by(|(_, j1), (_, j2)|  j1.cmp(j2) ))
        .last();
    println!("{:?}", mp);
    let res = mp.values()
        .map(|x| eval(x))
        .max_by(|u, v| u.len().cmp(&v.len())).unwrap();
    println!("{}", res.len());
    res.into_iter().for_each(|(l, r)| println!("{l} {r}"));
}
