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
    let _ = scan.token::<usize>();
    let x = scan.token::<usize>();
    let y = scan.token::<usize>();
    let s = scan.token_bytes();
    let t = scan.token_bytes();
    let mut a = Vec::new();
    for (i, (&c, &d)) in s.iter().zip(t.iter()).enumerate() {
        if c != d {
            a.push(i)
        }
    }
    let m = a.len();
    if m % 2 == 1 {
        writeln!(out, "-1").ok();
        return;
    }
    if y <= x || m == 0 {
        if m == 2 && a[0] + 1 == a[1] {
            writeln!(out, "{}", x.min(2 * y)).ok();
            return;
        }
        writeln!(out, "{}", m / 2 * y).ok();
    } else {
        let mut f = vec![0; m + 1];
        f[1] = y;
        for (i, &v) in a.iter().enumerate() {
            if i > 0 {
                f[i + 1] = (f[i] + y).min(f[i - 1] + (v - a[i - 1]) * 2 * x)
            }
        }
        writeln!(out, "{}", f[m] / 2).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// https://codeforces.com/problemset/problem/1733/D2
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
