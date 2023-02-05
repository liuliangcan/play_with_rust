mod problem;
#[allow(unused)]
use std::collections::*;
// use std::intrinsics::atomic_load_acquire;
use std::io::{BufRead, BufWriter, Write};

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}
impl<R: ::std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }
    pub fn token<T: ::std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
   pub fn token_bytes(&mut self) -> Vec<u8> {
       let s = self.token::<String>();
       return s.as_bytes().into();
   }
}
fn main() {
    let mut b = 1;

    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let n = scan.token::<usize>();
    let mut a = vec![vec![]; n];
    for i in 0..n {
        // let v = scan.token::<String>();
        a[i] = scan.token_bytes();
        println!("{:?}",String::from_utf8_lossy(&a[i]));
    }
    // let mut ans = 0;
    println!("{:?}",a);
    // for s in a {
    //     println!("{}", s);
    //     println!("{}",s.len());
    //     println!("{:?}",s.as_bytes().len());
    //
    // }
}

