#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};
#[allow(unused)]
fn main() {
    let sin = std::io::stdin();
    let scan = &mut Scanner::new(sin.lock());
    let sout = std::io::stdout();
    let out = &mut BufWriter::new(sout.lock());
    run(scan, out);
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
    #[allow(unused)]
    pub fn token_bytes(&mut self) -> Vec<u8> {
        let s = self.token::<String>();
        return s.as_bytes().into();
    }
}

// const MOD:i64 = 1000000000+7;
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut z = vec![0]; // 前缀和
    let mut y = vec![0]; // 前缀和递增的部分对应的下标
    let mut s = 0;
    for i in 0..n {
        s += scan.token::<i64>();
        if s > *z.last().unwrap() {
            z.push(s);
            y.push(i);
        }
    }
    let mx = *z.last().unwrap();
    for _ in 0..m {
        let x = scan.token::<i64>();
        if x <= mx {
            write!(out, "{} ", y[z.partition_point(|&v| v < x)]).ok();
        } else if s <= 0 {
            write!(out, "-1 ").ok();
        } else {
            let cnt = (x - mx + s - 1) / s;
            let p = z.partition_point(|&v| v < x - cnt * s);
            write!(out, "{} ", (cnt as usize) * n + y[p]).ok();
        }
    }
    writeln!(out).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// https://codeforces.com/problemset/problem/1490/G
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
