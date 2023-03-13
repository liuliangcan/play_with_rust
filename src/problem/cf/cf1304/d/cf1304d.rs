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
    let s = scan.token_bytes();
    let mut ans = vec![vec![0;n]; 2];
    for i in 0..n {
        ans[0][i] = n - i;
        ans[1][i] = i + 1;
    }
    let mut i = 0;
    while i < n - 1 {
        let mut l = i;
        while i < n - 1 && s[i] == s[l] {
            i += 1;
        }
        let z = if s[l] == b'>' { 1 } else { 0 };
        let mut r = i;
        while l < r {
            (ans[z][l], ans[z][r]) = (ans[z][r], ans[z][l]);
            l += 1;
            r -= 1;
        }
    }
    for i in 0..2 {
        for v in ans[i].iter() {
            write!(out, "{} ", *v).ok();
        }
        writeln!(out).ok();
    }
}

pub fn run(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let t = scan.token::<usize>();
    for _ in 0..t {
        solve(scan, out)
    }
}
// http://codeforces.com/problemset/problem/1304/D
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
