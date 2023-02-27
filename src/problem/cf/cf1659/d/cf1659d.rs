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
    let mut c = vec![0i32; n];
    for i in 0..n {
        c[i] = scan.token::<i32>();
    }
    let mut k = (c.iter().map(|&x| x as i64).sum::<i64>() / (n as i64)) as i32;
    let mut d = vec![0i32; n];
    let mut sd = 0i32;
    let mut a = vec![0i32; n];
    for (i, &v) in c.iter().enumerate().rev() {
        sd += d[i];
        if v + sd == i as i32 + 1 {
            a[i] = 1;
        }
        sd -= 1;
        if i as i32 - k >= 0 {
            d[i - k as usize] += 1;
        }
        if a[i] == 1 {
            k -= 1
        }
    }

    // writeln!(out, "{}", itertools::join(a, " ")).ok();
    for &v in &a {
        write!(out, "{} ", v).ok();
    }
    writeln!(out).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// https://codeforces.com/problemset/problem/1659/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
