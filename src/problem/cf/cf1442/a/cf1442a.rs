#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    solve(scan, out);
}

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
    //    pub fn token_bytes(&mut self) -> Vec<u8> {
    //        let s = self.token::<String>();
    //        return s.as_bytes().into();
    //    }
}

// const MOD:usize = 1000000000+7;
// https://codeforces.com/contest/1442/problem/A
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut s = scan.token::<i32>();
        let mut p = s;
        for _ in 0..n - 1 {
            let a = scan.token::<i32>();
            if a < p {
                s += a - p;
            }
            p = a;
        }
        if s < 0 {
            writeln!(out, "NO").ok();
        } else {
            writeln!(out, "YES").ok();
        }
    }
}
