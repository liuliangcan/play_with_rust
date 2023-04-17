#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let k = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        a.push(scan.token::<i64>());
    }
    let mut w = Vec::with_capacity(k);
    for i in 0..k {
        w.push(scan.token::<usize>());
    }
    a.sort_unstable();
    w.sort_unstable();
    let mut ans = 0;
    for i in 0..k {
        if w[i] == 1 {
            ans += a[n - i - 1] * 2;
        } else {
            ans += a[n - i - 1];
        }
    }
    let mut j = 0;
    for i in (0..k).rev() {
        if w[i] == 1 {
            break;
        }
        ans += a[j];
        j += w[i] - 1;
    }
    writeln!(out, "{}", ans).ok();
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
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

// https://codeforces.com/problemset/problem/1369/C
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
