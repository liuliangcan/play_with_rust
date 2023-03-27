#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut p = Vec::new();
    for i in 0..n {
        p.push((scan.token::<i64>(), scan.token::<i64>()));
    }
    if n <= 4 {
        writeln!(out, "YES").ok();
        return;
    }
    fn is_line(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        // 判断三点是否共线
        return (a.1 - b.1) * (a.0 - c.0) == (a.1 - c.1) * (a.0 - b.0);
    }
    fn f(a: (i64, i64), b: (i64, i64), p: &Vec<(i64, i64)>) -> bool {
        // 判断数组以这两个点确定的直线能否再只需额外一条线
        let mut other = Vec::new();
        for &c in p.iter() {
            if !is_line(a, b, c) {
                if other.len() < 2 {
                    other.push(c);
                } else if !is_line(other[0], other[1], c) {
                    return false;
                }
            }
        }
        return true;
    }
    if f(p[0], p[1], &p) || f(p[0], p[2], &p) || f(p[2], p[1], &p) {
        writeln!(out, "YES").ok();
        return;
    }
    writeln!(out, "NO").ok();
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

// https://codeforces.com/contest/961/problem/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
