use itertools::Itertools;
#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = scan.token::<i64>();
    }
    if n == 1 {
        writeln!(out, "Yes\n0").ok();
        return;
    }
    let mut p = 0; // b的前缀和
    let mut s = 0; // a的前缀和
    let mut x = -1000_000_000_00;
    let mut b = Vec::new();
    let last = a[a.len() - 1];
    for i in 0..n - 1 {
        s += a[i];
        p += x * a[i];
        b.push(x);
        x += 1;
        if s == last && x < 0 {
            x = 0;
        }
    }
    let last = *b.last().unwrap();
    if last < -p * a[a.len() - 1] && -p * a[a.len() - 1] <= 1000_000_000_000 {
        b.push(-p * a[a.len() - 1]);
        writeln!(out, "Yes\n{}", b.iter().join(" ")).ok();
        return;
    }
    let mut p = 0; // b的后缀和
    let mut s = 0; // a的后缀和
    let mut x = 1000_000_000_00;
    let mut b = Vec::new();
    let first = a[0];
    for i in (1..n).rev() {
        s += a[i];
        p += x * a[i];
        b.push(x);
        x -= 1;
        if s == first && x > 0{
            x = 0;
        }
    }
    let last = *b.last().unwrap();
    if -1000_000_000_000 <= -p * a[0] && -p * a[0] < last {
        b.push(-p * a[0]);
        writeln!(out, "Yes\n{}", b.iter().rev().join(" ")).ok();
        return;
    }
    writeln!(out, "No").ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    solve(scan, out)
}

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

// https://atcoder.jp/contests/arc153/tasks/arc153_c
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
