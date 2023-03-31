#[allow(unused)]
use std::collections::*;
use std::io::{BufRead, BufWriter, Write};

// const MOD:i64 = 1000000000+7;
#[allow(unused)]
pub fn solve(scan: &mut Scanner<impl BufRead>, out: &mut impl Write) {
    let n = scan.token::<usize>();
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for i in 0..n {
        a.push(scan.token::<i64>());
    }
    for i in 0..n {
        b.push(scan.token::<i64>());
    }
    if n & 1 > 0 && a[n / 2] != b[n / 2] {
        writeln!(out, "No");
        return;
    }
    let mut cnt = HashMap::new();
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let mut x = a[l];
        let mut y = a[r];
        if x < y {
            (x, y) = (y, x);
        }
        *cnt.entry((x, y)).or_insert(0) += 1;
        let mut x = b[l];
        let mut y = b[r];
        if x < y {
            (x, y) = (y, x);
        }
        *cnt.entry((x, y)).or_insert(0) -= 1;
        l += 1;
        r -= 1;
    }
    for &v in cnt.values() {
        if v != 0 {
            writeln!(out, "No");
            return;
        }
    }
    writeln!(out, "yes").ok();
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

// https://codeforces.com/problemset/problem/1365/F
// 本模板由 https://github.com/liuliangcan/play_with_python/blob/main/tools/gen_code_tool/gen_template.py 自动生成;中文题面描述可移步
